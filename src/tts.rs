use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use reqwest;
use serde_json::json;
use super::message_system::{MessagePart, CacheKey, PersonalitySettings};
use tokio::time::Duration as TokioDuration;
use rodio;

pub struct TTSManager {
    client: reqwest::Client,
    cache: Arc<Mutex<HashMap<CacheKey, Vec<u8>>>>,
    voice_type: String,
    volume: f32,
    speech_rate: f32,
    audio_enabled: bool,
}

impl TTSManager {
    pub fn new() -> Option<Self> {
        println!("Initializing TTSManager...");
        
        // Check if OpenAI API key is available
        if std::env::var("OPENAI_API_KEY").is_err() {
            eprintln!("Error: OPENAI_API_KEY environment variable not found");
            return None;
        }

        let tts = Self {
            client: reqwest::Client::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            voice_type: "alloy".to_string(),
            volume: 1.0,
            speech_rate: 1.0,
            audio_enabled: true,
        };

        // Initialize audio cache
        println!("Initializing audio cache...");
        if let Err(e) = tts.archive_and_clear_cache() {
            eprintln!("Warning: Failed to initialize audio cache: {}", e);
            // Continue anyway as this is not critical
        }

        Some(tts)
    }

    pub fn archive_and_clear_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Archiving and clearing audio cache");
        let mut cache = self.cache.lock().unwrap();
        
        // Create archive directory if it doesn't exist
        let archive_dir = Path::new("cache").join("tts").join("archive");
        if let Err(e) = fs::create_dir_all(&archive_dir) {
            println!("Failed to create archive directory: {}", e);
            return Ok(());
        }

        // Archive audio data
        let mut archived_count = 0;
        for (key, audio_data) in cache.iter() {
            let key_str = format!("{:?}", key);
            let safe_key = key_str.replace(['\\', '/', ':', '*', '?', '"', '<', '>', '|'], "_");
            let audio_path = archive_dir.join(format!("{}.mp3", safe_key));
            
            if let Err(e) = fs::write(&audio_path, audio_data) {
                println!("Failed to archive audio data for key {:?}: {}", key, e);
                continue;
            }
            archived_count += 1;
        }

        // Archive cache keys
        let keys: Vec<String> = cache.keys().map(|k| format!("{:?}", k)).collect();
        let keys_json = serde_json::to_string_pretty(&keys).unwrap();
        let keys_path = archive_dir.join("tts_cache_archive.json");
        
        if let Err(e) = fs::write(&keys_path, keys_json) {
            println!("Failed to archive cache keys: {}", e);
        } else {
            println!("Archived {} cache keys to: {:?}", keys.len(), keys_path);
            println!("Total audio files archived: {}", archived_count);
        }

        // Clear the cache
        cache.clear();
        println!("Clearing audio cache");
        Ok(())
    }

    pub fn clear_cache(&mut self) {
        println!("Clearing audio cache");
        self.cache.lock().unwrap().clear();
    }

    pub async fn speak(&mut self, message_parts: Vec<MessagePart>, personality: &PersonalitySettings) -> Result<(), Box<dyn std::error::Error>> {
        if !self.audio_enabled {
            println!("Audio is disabled, skipping speech");
            return Ok(());
        }

        println!("Starting speak function with {} message parts", message_parts.len());
        
        let mut audio_clips = Vec::new();
        
        for part in message_parts {
            println!("Processing message part: {:?}", part);
            
            let text = match &part {
                MessagePart::Static(text) => text,
                MessagePart::Dynamic(text) => text,
                MessagePart::Full(text) => text,
            };

            if text.trim().is_empty() {
                println!("Skipping empty text");
                continue;
            }

            println!("Generating audio for text: {}", text);
            
            let audio_data = match self.generate_audio(text).await {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Failed to generate audio: {}", e);
                    continue; // Skip this part but continue with others
                }
            };

            println!("Successfully generated audio data of size: {} bytes", audio_data.len());
            
            // Store in cache with proper CacheKey type
            let cache_key = self.get_cache_key(&part, personality);
            self.cache.lock().unwrap().insert(cache_key, audio_data.clone());
            
            // Add to clips for playback
            audio_clips.push(audio_data);
        }

        // Play all generated audio clips
        if !audio_clips.is_empty() {
            println!("Playing {} audio clips", audio_clips.len());
            if let Err(e) = self.play_composed_message(audio_clips).await {
                eprintln!("Failed to play audio: {}", e);
            }
        }

        Ok(())
    }

    fn get_cache_key(&self, message: &MessagePart, personality: &PersonalitySettings) -> CacheKey {
        match message {
            MessagePart::Static(text) => CacheKey::Static(text.clone(), personality.clone()),
            MessagePart::Dynamic(text) => CacheKey::Dynamic(text.clone()),
            MessagePart::Full(text) => CacheKey::Full("full".to_string(), text.clone()),
        }
    }

    async fn generate_audio(&self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        println!("Generating audio for text: {}", text);
        let api_key = std::env::var("OPENAI_API_KEY")?;
        let url = "https://api.openai.com/v1/audio/speech";

        println!("Making API request to OpenAI TTS endpoint");
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "tts-1",
                "input": text,
                "voice": self.voice_type,
                "speed": self.speech_rate
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            println!("OpenAI API error: {}", error_text);
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        println!("Successfully received response from OpenAI");
        let audio_data = response.bytes().await?.to_vec();
        println!("Converted response to {} bytes of audio data", audio_data.len());
        Ok(audio_data)
    }

    async fn play_composed_message(&self, clips: Vec<Vec<u8>>) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing audio output device");
        let (_stream, stream_handle) = match rodio::OutputStream::try_default() {
            Ok(output) => {
                println!("Successfully initialized audio output device");
                output
            },
            Err(e) => {
                println!("Failed to initialize audio output device: {}", e);
                return Err(e.into());
            }
        };

        let total_clips = clips.len();
        for (i, clip) in clips.into_iter().enumerate() {
            println!("Playing clip {} of {}", i + 1, total_clips);
            let sink = match rodio::Sink::try_new(&stream_handle) {
                Ok(sink) => {
                    println!("Successfully created audio sink");
                    sink
                },
                Err(e) => {
                    println!("Failed to create audio sink: {}", e);
                    return Err(e.into());
                }
            };
            
            let cursor = std::io::Cursor::new(clip);
            match rodio::Decoder::new(cursor) {
                Ok(decoder) => {
                    println!("Successfully created audio decoder");
                    sink.append(decoder);
                    sink.sleep_until_end();
                    println!("Finished playing clip {}", i + 1);
                },
                Err(e) => {
                    println!("Failed to create audio decoder: {}", e);
                    return Err(e.into());
                }
            }

            // Add a small pause between clips
            tokio::time::sleep(TokioDuration::from_millis(100)).await;
        }
        Ok(())
    }

    pub fn set_voice_type(&mut self, voice_type: String) {
        self.voice_type = voice_type;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_speech_rate(&mut self, rate: f32) {
        self.speech_rate = rate.clamp(0.5, 2.0);
    }

    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.audio_enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message_system::MessagePart;

    #[tokio::test]
    async fn test_tts_caching() {
        if let Some(tts) = TTSManager::new() {
            let personality = PersonalitySettings {
                drunk_level: 0,
                sass_level: 0,
                enthusiasm: 0,
                anxiety_level: 0,
                grand_pappi_refs: 0,
                voice_type: "alloy".to_string(),
            };
            let test_text = "Testing".to_string();
            let messages = vec![
                MessagePart::Static(test_text.clone()),
                MessagePart::Dynamic("audio system".to_string()),
            ];
            let key = tts.get_cache_key(&messages[0], &personality);
            match &messages[0] {
                MessagePart::Static(text) => {
                    assert!(matches!(key, CacheKey::Static(t, _) if t == test_text));
                }
                _ => panic!("Expected Static message part"),
            }
        }
    }

    #[tokio::test]
    async fn test_tts_integration() {
        if let Some(mut tts) = TTSManager::new() {
            let personality = PersonalitySettings {
                drunk_level: 0,
                sass_level: 0,
                enthusiasm: 0,
                anxiety_level: 0,
                grand_pappi_refs: 0,
                voice_type: "alloy".to_string(),
            };
            
            // Test with a simple static message
            let messages = vec![MessagePart::Static("Test message".to_string())];
            let result = tts.speak(messages.clone(), &personality).await;
            
            match result {
                Ok(_) => println!("TTS test succeeded"),
                Err(e) => {
                    let error_str = e.to_string();
                    if error_str.contains("OPENAI_API_KEY") || error_str.contains("environment variable not found") {
                        println!("Skipping TTS test - OpenAI API key not available");
                        return;
                    } else {
                        panic!("Unexpected error: {}", e);
                    }
                }
            }
            
            // Test with a more complex message combining static and dynamic parts
            let messages = vec![
                MessagePart::Static("Testing".to_string()),
                MessagePart::Dynamic("dynamic content".to_string())
            ];
            let result = tts.speak(messages.clone(), &personality).await;
            match result {
                Ok(_) => println!("TTS test succeeded"),
                Err(e) => {
                    let error_str = e.to_string();
                    if error_str.contains("OPENAI_API_KEY") || error_str.contains("environment variable not found") {
                        println!("Skipping TTS test - OpenAI API key not available");
                        return;
                    } else {
                        panic!("Unexpected error: {}", e);
                    }
                }
            }
        } else {
            println!("Skipping TTS integration test - TTS system not available");
        }
    }

    #[tokio::test]
    async fn test_audio_archiving() {
        println!("Starting audio archiving test");
        
        // Create a new TTS manager
        if let Some(mut tts) = TTSManager::new() {
            let personality = PersonalitySettings {
                drunk_level: 0,
                sass_level: 0,
                enthusiasm: 0,
                anxiety_level: 0,
                grand_pappi_refs: 0,
                voice_type: "alloy".to_string(),
            };

            // Generate some test audio
            let test_message = vec![MessagePart::Static("Test message for archiving".to_string())];
            println!("Generating test audio...");
            
            // Create mock audio data
            let mock_audio_data = vec![0x1, 0x2, 0x3, 0x4, 0x5]; // Mock MP3 header
            let cache_key = tts.get_cache_key(&test_message[0], &personality);
            tts.cache.lock().unwrap().insert(cache_key, mock_audio_data.clone());
            println!("Added mock audio data to cache");

            // Archive the cache
            println!("Archiving cache...");
            tts.archive_and_clear_cache();

            // Verify archive directory exists
            let archive_dir = Path::new("cache/tts/archive");
            assert!(archive_dir.exists(), "Archive directory should exist");

            // Check for archived files
            let mut found_audio = false;
            let mut found_json = false;
            if let Ok(entries) = std::fs::read_dir(archive_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "mp3") {
                            found_audio = true;
                            println!("Found archived audio file: {:?}", path);
                            // Print file size
                            if let Ok(metadata) = std::fs::metadata(&path) {
                                println!("Audio file size: {} bytes", metadata.len());
                            }
                            // Verify file contents
                            if let Ok(contents) = std::fs::read(&path) {
                                assert_eq!(contents, mock_audio_data, "Archived audio data should match original");
                                println!("Verified archived audio data matches original");
                            }
                        } else if path.file_name().map_or(false, |name| name == "tts_cache_archive.json") {
                            found_json = true;
                            println!("Found archive JSON file: {:?}", path);
                            // Print JSON contents
                            if let Ok(contents) = std::fs::read_to_string(&path) {
                                println!("Archive JSON contents: {}", contents);
                            }
                        }
                    }
                }
            }

            assert!(found_audio, "Should have found at least one archived audio file");
            assert!(found_json, "Should have found the archive JSON file");

            println!("Audio archiving test completed successfully");
        } else {
            println!("Skipping audio archiving test - TTS system not available");
        }
    }
} 