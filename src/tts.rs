use reqwest;
use std::fs::{self, File};
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use chrono::{DateTime, Utc};
use crate::audio_manager::AudioManager;

pub struct TTSManager {
    client: reqwest::blocking::Client,
    api_key: String,
    volume: f32,
    speech_rate: f32,
    voice_type: String,
    enthusiasm: f32,
    anxiety: f32,
    sink: Arc<Mutex<Sink>>,
    _stream: OutputStream,
    audio_manager: Arc<Mutex<AudioManager>>,
    audio_enabled: bool,
}

impl TTSManager {
    pub fn new() -> Option<Self> {
        match std::env::var("OPENAI_API_KEY") {
            Ok(api_key) => {
                println!("Found OpenAI API key");
                // Initialize audio output
                let (stream, stream_handle) = OutputStream::try_default()
                    .expect("Failed to initialize audio output");
                let sink = Sink::try_new(&stream_handle)
                    .expect("Failed to create audio sink");
                sink.set_volume(1.0);
                
                Some(Self {
                    client: reqwest::blocking::Client::new(),
                    api_key,
                    volume: 0.8,
                    speech_rate: 1.0,
                    voice_type: "nova".to_string(),
                    enthusiasm: 0.5,
                    anxiety: 0.5,
                    sink: Arc::new(Mutex::new(sink)),
                    _stream: stream,
                    audio_manager: Arc::new(Mutex::new(AudioManager::new())),
                    audio_enabled: true,
                })
            }
            Err(_) => {
                println!("No OpenAI API key found in environment");
                None
            }
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Ok(sink) = self.sink.lock() {
            sink.set_volume(self.volume);
        }
    }

    pub fn set_speech_rate(&mut self, rate: f32) {
        self.speech_rate = rate.clamp(0.5, 2.0);
    }

    pub fn set_voice_type(&mut self, voice: String) {
        self.voice_type = voice;
    }

    pub fn set_enthusiasm(&mut self, level: f32) {
        self.enthusiasm = level.clamp(0.0, 1.0);
    }

    pub fn set_anxiety(&mut self, level: f32) {
        self.anxiety = level.clamp(0.0, 1.0);
    }

    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.audio_enabled = enabled;
        if let Ok(mut sink) = self.sink.lock() {
            if enabled {
                sink.set_volume(self.volume);
            } else {
                sink.set_volume(0.0);
            }
        }
    }

    pub fn speak(&mut self, text: &str, trigger_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.audio_enabled {
            println!("Audio is disabled, skipping TTS");
            return Ok(());
        }

        // Create personality state map for metadata
        let mut personality_state = HashMap::new();
        personality_state.insert("volume".to_string(), self.volume);
        personality_state.insert("speech_rate".to_string(), self.speech_rate);
        personality_state.insert("enthusiasm".to_string(), self.enthusiasm);
        personality_state.insert("anxiety".to_string(), self.anxiety);

        let cache_path = self.audio_manager.lock().unwrap().get_audio_path(text, trigger_type, personality_state);

        // If not in cache, generate and save
        if !cache_path.exists() {
            println!("Generating new audio for: {}", text);
            
            let url = "https://api.openai.com/v1/audio/speech";
            let response = self.client.post(url)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&serde_json::json!({
                    "model": "tts-1",
                    "input": text,
                    "voice": "nova",
                    "response_format": "mp3",
                    "speed": self.speech_rate
                }))
                .send()?;

            if !response.status().is_success() {
                let error_text = response.text()?;
                println!("OpenAI API Error: {}", error_text);
                return Err(format!("OpenAI API Error: {}", error_text).into());
            }

            let audio_data = response.bytes()?;
            println!("Received {} bytes of audio data", audio_data.len());
            fs::write(&cache_path, audio_data)?;
            println!("Saved audio to cache: {:?}", cache_path);
        } else {
            println!("Using cached audio for: {}", text);
        }

        // Queue the audio in the sink
        if let Ok(sink) = self.sink.lock() {
            println!("Opening audio file: {:?}", cache_path);
            let file = File::open(&cache_path)?;
            let reader = BufReader::new(file);
            match Decoder::new(reader) {
                Ok(source) => {
                    println!("Successfully decoded audio file");
                    sink.set_volume(self.volume);
                    sink.append(source);
                    println!("Audio queued for playback");
                },
                Err(e) => {
                    println!("Error decoding audio: {}", e);
                    return Err(e.into());
                }
            }
        }

        // Clean up old cache files periodically (files older than 24 hours)
        self.audio_manager.lock().unwrap().cleanup_old_cache(24);

        Ok(())
    }

    pub fn cleanup(&mut self) -> Result<(), std::io::Error> {
        let audio_manager = self.audio_manager.lock().unwrap();
        
        // Create directories if they don't exist
        fs::create_dir_all(&audio_manager.cache_dir)?;
        fs::create_dir_all(&audio_manager.archive_dir)?;

        // Clean up all cached files
        if let Ok(entries) = fs::read_dir(&audio_manager.cache_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(_) = fs::remove_file(entry.path()) {
                        println!("Cleaned up cached file: {:?}", entry.path());
                    }
                }
            }
        }
        
        // Clean up all archived files
        if let Ok(entries) = fs::read_dir(&audio_manager.archive_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(_) = fs::remove_file(entry.path()) {
                        println!("Cleaned up archived file: {:?}", entry.path());
                    }
                }
            }
        }

        // Clean up metadata file
        if let Ok(_) = fs::remove_file(&audio_manager.metadata_file) {
            println!("Cleaned up metadata file");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn create_test_tts() -> TTSManager {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        TTSManager {
            api_key: "test_key".to_string(),
            client: reqwest::blocking::Client::new(),
            volume: 0.8,
            speech_rate: 1.0,
            voice_type: "Scottish Teen".to_string(),
            enthusiasm: 0.5,
            anxiety: 0.5,
            sink: Arc::new(Mutex::new(sink)),
            _stream: stream,
            audio_manager: Arc::new(Mutex::new(AudioManager::new())),
            audio_enabled: true,
        }
    }

    fn cleanup_test_files() {
        // Clean up test directories if they exist
        let paths = ["audio_cache", "audio_archive"];
        for path in paths.iter() {
            if Path::new(path).exists() {
                let _ = fs::remove_dir_all(path);
            }
        }
    }

    // Run before each test
    fn setup() -> TTSManager {
        cleanup_test_files();
        let tts = create_test_tts();
        
        // Ensure directories exist after cleanup
        let audio_manager = tts.audio_manager.lock().unwrap();
        fs::create_dir_all(&audio_manager.cache_dir).unwrap();
        fs::create_dir_all(&audio_manager.archive_dir).unwrap();
        drop(audio_manager);
        
        tts
    }

    // Run after each test
    fn teardown(tts: &mut TTSManager) {
        let _ = tts.cleanup();
        cleanup_test_files();
    }

    #[test]
    fn test_personality_traits() {
        let mut tts = setup();
        
        // Test enthusiasm
        tts.set_enthusiasm(0.8);
        assert_eq!(tts.enthusiasm, 0.8);
        
        // Test anxiety
        tts.set_anxiety(0.8);
        assert_eq!(tts.anxiety, 0.8);
        
        // Test speech rate
        tts.set_speech_rate(1.8);
        assert_eq!(tts.speech_rate, 1.8);

        teardown(&mut tts);
    }

    #[test]
    fn test_volume_bounds() {
        let mut tts = setup();
        
        tts.set_volume(1.5);
        assert_eq!(tts.volume, 1.0);
        tts.set_volume(-0.5);
        assert_eq!(tts.volume, 0.0);

        teardown(&mut tts);
    }

    #[test]
    fn test_speech_rate_bounds() {
        let mut tts = setup();
        
        tts.set_speech_rate(2.5);
        assert_eq!(tts.speech_rate, 2.0);
        tts.set_speech_rate(0.3);
        assert_eq!(tts.speech_rate, 0.5);

        teardown(&mut tts);
    }

    #[test]
    fn test_cleanup() {
        let mut tts = setup();
        
        // Ensure directories exist before creating test files
        let audio_manager = tts.audio_manager.lock().unwrap();
        fs::create_dir_all(&audio_manager.cache_dir).unwrap();
        fs::create_dir_all(&audio_manager.archive_dir).unwrap();
        drop(audio_manager);  // Release the lock
        
        // Create some test files
        let test_text = "Test cleanup";
        let mut personality_state = HashMap::new();
        personality_state.insert("volume".to_string(), 0.8);
        
        let cache_path = tts.audio_manager.lock().unwrap().get_audio_path(test_text, "test", personality_state);
        fs::write(&cache_path, b"test data").unwrap();
        
        // Verify file was created
        assert!(cache_path.exists());
        
        // Run cleanup
        tts.cleanup().unwrap();
        
        // Verify files were cleaned up
        assert!(!cache_path.exists());
    }
} 