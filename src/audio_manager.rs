use std::fs::{self, File};
use std::path::PathBuf;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioMetadata {
    pub text: String,
    pub trigger_type: String,
    pub personality_state: HashMap<String, f32>,
    pub timestamp: DateTime<Utc>,
}

pub struct AudioManager {
    pub cache_dir: PathBuf,
    pub archive_dir: PathBuf,
    pub metadata_file: PathBuf,
}

impl AudioManager {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from("audio_cache");
        let archive_dir = PathBuf::from("audio_archive");
        let metadata_file = cache_dir.join("metadata.json");

        // Create directories if they don't exist
        fs::create_dir_all(&cache_dir).unwrap_or_default();
        fs::create_dir_all(&archive_dir).unwrap_or_default();

        AudioManager {
            cache_dir,
            archive_dir,
            metadata_file,
        }
    }

    pub fn get_audio_path(&mut self, text: &str, trigger_type: &str, personality_state: HashMap<String, f32>) -> PathBuf {
        // Create directories if they don't exist
        fs::create_dir_all(&self.cache_dir).unwrap_or_default();
        fs::create_dir_all(&self.archive_dir).unwrap_or_default();

        // Generate a unique filename based on the text and personality state
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        hasher.update(trigger_type.as_bytes());
        for (key, value) in personality_state.iter() {
            hasher.update(key.as_bytes());
            hasher.update(value.to_string().as_bytes());
        }
        let hash = format!("{:x}", hasher.finalize());
        
        // Save metadata
        let metadata = AudioMetadata {
            text: text.to_string(),
            trigger_type: trigger_type.to_string(),
            personality_state,
            timestamp: Utc::now(),
        };
        
        if let Ok(metadata_json) = serde_json::to_string_pretty(&metadata) {
            fs::write(&self.metadata_file, metadata_json).unwrap_or_default();
        }
        
        self.cache_dir.join(format!("{}.mp3", hash))
    }

    pub fn archive_audio(&mut self, text: &str) {
        if let Some(metadata) = self.get_metadata(text) {
            let source_path = self.get_audio_path(
                &metadata.text,
                &metadata.trigger_type,
                metadata.personality_state.clone(),
            );
            
            if source_path.exists() {
                let dest_path = self.archive_dir.join(source_path.file_name().unwrap());
                if let Ok(_) = fs::copy(&source_path, &dest_path) {
                    fs::remove_file(source_path).unwrap_or_default();
                }
            }
        }
    }

    pub fn get_metadata(&self, text: &str) -> Option<AudioMetadata> {
        if let Ok(metadata_json) = fs::read_to_string(&self.metadata_file) {
            if let Ok(metadata) = serde_json::from_str::<AudioMetadata>(&metadata_json) {
                if metadata.text == text {
                    return Some(metadata);
                }
            }
        }
        None
    }

    pub fn cleanup_old_cache(&mut self, max_age_hours: i64) {
        let now = SystemTime::now();
        
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(age) = now.duration_since(modified) {
                                if age > Duration::from_secs((max_age_hours * 3600) as u64) {
                                    fs::remove_file(entry.path()).unwrap_or_default();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
} 