#[cfg(test)]
mod tests {
    use cyber_ninja_monitor::{AIPersonality, TTSManager, AudioManager};
    use std::collections::HashMap;

    #[test]
    fn test_personality_defaults() {
        let personality = AIPersonality::default();
        
        // Test default values
        assert_eq!(personality.drunk_level, 0.0);
        assert_eq!(personality.sass_level, 0.5);
        assert_eq!(personality.tech_expertise, 0.7);
        assert_eq!(personality.grand_pappi_references, 0.3);
        assert_eq!(personality.enthusiasm, 0.8);
        assert_eq!(personality.anxiety_level, 0.2);
        assert_eq!(personality.volume, 0.7);
        assert_eq!(personality.speech_rate, 0.5);
        assert!(personality.audio_enabled);
        
        // Test catchphrases
        assert!(!personality.catchphrases.is_empty());
        assert!(personality.catchphrases.iter().any(|phrase| phrase.contains("haggis")));
        
        // Test exit messages
        assert!(!personality.exit_messages.is_empty());
        assert!(personality.exit_messages.iter().any(|msg| msg.contains("digital pub")));
    }

    #[test]
    fn test_clamp_values() {
        let mut personality = AIPersonality::default();
        
        // Set values outside valid range
        personality.drunk_level = 1.5;
        personality.sass_level = -0.5;
        personality.tech_expertise = 2.0;
        personality.grand_pappi_references = -1.0;
        personality.enthusiasm = 1.2;
        personality.anxiety_level = -0.3;
        personality.volume = 1.5;
        personality.speech_rate = 2.5;
        
        // Clamp values
        personality.clamp_values();
        
        // Verify all values are clamped
        assert!(personality.drunk_level <= 1.0);
        assert!(personality.sass_level >= 0.0);
        assert!(personality.tech_expertise <= 1.0);
        assert!(personality.grand_pappi_references >= 0.0);
        assert!(personality.enthusiasm <= 1.0);
        assert!(personality.anxiety_level >= 0.0);
        assert!(personality.volume <= 1.0);
        assert!(personality.speech_rate <= 2.0);
        assert!(personality.speech_rate >= 0.5);
    }

    #[test]
    fn test_audio_controls() {
        let mut personality = AIPersonality::default();
        assert!(personality.audio_enabled);
        
        // Test disabling audio
        let message = personality.toggle_audio();
        assert!(!personality.audio_enabled);
        assert!(message.contains("ninja"));
        
        // Test enabling audio
        let message = personality.toggle_audio();
        assert!(personality.audio_enabled);
        assert!(message.contains("noise"));
    }

    #[test]
    fn test_reset_audio() {
        let mut personality = AIPersonality::default();
        
        // Change audio settings
        personality.volume = 0.1;
        personality.speech_rate = 0.1;
        personality.audio_enabled = false;
        
        // Reset audio
        let message = personality.reset_audio();
        
        // Verify reset values
        assert_eq!(personality.volume, 0.7);
        assert_eq!(personality.speech_rate, 0.5);
        assert!(personality.audio_enabled);
        assert!(message.contains("reset"));
    }

    #[test]
    fn test_message_generation() {
        let mut personality = AIPersonality::default();
        let base_message = "System is running normally";
        
        // Test with default personality
        let message = personality.generate_message(base_message);
        assert!(message.contains(base_message));
        
        // Test with high sass level
        personality.sass_level = 0.9;
        let sassy_message = personality.generate_message(base_message);
        assert!(sassy_message.contains("tea"));
        
        // Test with high tech expertise
        personality.tech_expertise = 0.9;
        let tech_message = personality.generate_message(base_message);
        assert!(tech_message.contains("quantum"));
        
        // Test with high grand pappi references
        personality.grand_pappi_references = 0.8;
        let pappi_message = personality.generate_message(base_message);
        assert!(pappi_message.contains("Grand Pappi"));
        
        // Test with high anxiety
        personality.anxiety_level = 0.8;
        let anxious_message = personality.generate_message(base_message);
        assert!(anxious_message.contains("um"));
        
        // Test with high enthusiasm
        personality.enthusiasm = 0.9;
        let enthusiastic_message = personality.generate_message(base_message);
        assert!(enthusiastic_message.contains("!!"));
    }

    #[test]
    fn test_audio_archiving() {
        let mut audio_manager = AudioManager::new();
        let test_text = "Test audio message";
        let trigger_type = "test";
        let mut personality_state = HashMap::new();
        personality_state.insert("volume".to_string(), 0.8);
        personality_state.insert("speech_rate".to_string(), 1.0);

        // Get path for new audio
        let path = audio_manager.get_audio_path(test_text, trigger_type, personality_state);
        assert!(path.starts_with(&audio_manager.cache_dir));
        
        // Verify metadata
        let metadata = audio_manager.get_metadata(test_text).unwrap();
        assert_eq!(metadata.trigger_type, trigger_type);
        assert_eq!(metadata.text, test_text);
        assert!(metadata.personality_state.contains_key("volume"));
        assert!(metadata.personality_state.contains_key("speech_rate"));

        // Test archiving
        audio_manager.archive_audio(test_text);
        assert!(!path.exists(), "Audio file should be moved to archive");
    }

    #[test]
    fn test_personality_effects() {
        let mut personality = AIPersonality::default();
        let base_message = "Testing personality effects";

        // Test drunk effect
        personality.drunk_level = 0.8;
        let drunk_message = personality.generate_message(base_message);
        assert!(drunk_message.contains("sh") || drunk_message.contains("..."));

        // Test enthusiasm
        personality.drunk_level = 0.0;
        personality.enthusiasm = 0.9;
        let enthusiastic_message = personality.generate_message(base_message);
        assert!(enthusiastic_message.contains("!") || enthusiastic_message.contains("🎉"));

        // Test anxiety
        personality.enthusiasm = 0.0;
        personality.anxiety_level = 0.9;
        let anxious_message = personality.generate_message(base_message);
        assert!(anxious_message.contains("...") || anxious_message.contains("*nervously*"));

        // Test combined effects
        personality.enthusiasm = 0.9;
        personality.anxiety_level = 0.9;
        let combined_message = personality.generate_message(base_message);
        assert!(combined_message.contains("!") && combined_message.contains("..."));
    }

    #[test]
    fn test_personality_tts_integration() {
        // Create components
        let mut personality = AIPersonality::default();
        if let Some(mut tts) = TTSManager::new() {
            // Test that personality settings affect TTS
            personality.volume = 0.5;
            personality.speech_rate = 1.5;
            personality.enthusiasm = 0.9;
            
            // Apply personality to TTS
            tts.set_volume(personality.volume);
            tts.set_speech_rate(personality.speech_rate);
            
            // Generate a message with personality traits
            let message = personality.generate_message("Testing system integration");
            
            // Verify the message reflects personality
            assert!(message.contains("!"), "High enthusiasm should add exclamation marks");
            
            // Test that TTS can handle the generated message
            let result = tts.speak(&message, "test");
            assert!(result.is_ok(), "TTS should handle personality-modified messages");
            
            // Test audio controls integration
            let disable_message = personality.toggle_audio();
            assert!(!personality.audio_enabled);
            assert!(disable_message.contains("ninja-quiet"), "Disable message should be personality-driven");
            
            // When audio is disabled, volume should be 0
            tts.set_volume(if personality.audio_enabled { personality.volume } else { 0.0 });
            
            // Re-enable audio
            let enable_message = personality.toggle_audio();
            assert!(personality.audio_enabled);
            assert!(enable_message.contains("chatty"), "Enable message should be personality-driven");
            
            // Clean up
            let _ = tts.cleanup();
        }
    }
}
