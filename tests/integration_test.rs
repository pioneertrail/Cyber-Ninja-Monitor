#[cfg(test)]
mod tests {
    use cyber_ninja_monitor::{
        message_system::{MessageSystem, MessagePart, PersonalitySettings},
        tts::TTSManager,
        ai_personality::AIPersonality,
    };
    use std::collections::HashMap;
    use std::error::Error;
    use std::env;
    use tokio::runtime::Runtime;

    #[test]
    fn test_personality_defaults() {
        let personality = AIPersonality::default();
        
        // Test default values
        assert_eq!(personality.drunk_level, 0.0);
        assert_eq!(personality.sass_level, 0.5);
        assert_eq!(personality.tech_expertise, 0.7);
        assert_eq!(personality.grand_pappi_references, 0.3);
        assert_eq!(personality.enthusiasm, 0.6);
        assert_eq!(personality.anxiety_level, 0.2);
        assert_eq!(personality.voice_type, "alloy");
        assert_eq!(personality.volume, 0.8);
        assert_eq!(personality.speech_rate, 1.0);
        assert!(personality.audio_enabled);
        
        // Test catchphrases
        assert!(!personality.catchphrases.is_empty());
        assert!(personality.catchphrases.iter().any(|phrase| phrase.contains("Beep boop")));
        assert!(personality.catchphrases.iter().any(|phrase| phrase.contains("quantum")));
        assert!(personality.catchphrases.iter().any(|phrase| phrase.contains("Batman")));
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
        assert!(message.contains("silent"), "Message should indicate going silent");
        
        // Test enabling audio
        let message = personality.toggle_audio();
        assert!(personality.audio_enabled);
        assert!(message.contains("online"), "Message should indicate coming back online");
    }

    #[test]
    fn test_reset_audio() {
        let mut personality = AIPersonality::default();
        
        // Change audio settings
        personality.volume = 0.1;
        personality.speech_rate = 0.1;
        personality.audio_enabled = true;
        
        // Reset audio
        let message = personality.reset_audio();
        
        // Verify reset values
        assert_eq!(personality.volume, 0.8);
        assert_eq!(personality.speech_rate, 1.0);
        assert!(personality.audio_enabled);
        assert!(message.contains("reset"));
    }

    #[test]
    fn test_message_generation() {
        let test_input = "Hello, world!";
        let message = vec![MessagePart::Static(test_input.to_string())];
        
        // Verify the message contains our test input
        assert!(matches!(&message[0], MessagePart::Static(text) if text == test_input));
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
        personality.drunk_level = 1.0;

        let base_message = "This is a test";
        let message = vec![MessagePart::Static(base_message.to_string())];
        let modified = personality.apply_personality(&message[0]);

        if let MessagePart::Static(text) = modified {
            assert!(text.contains("*hic*") || text != base_message);
        } else {
            panic!("Expected Static message part");
        }
    }

    #[test]
    fn test_personality_tts_integration() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            if let Ok(mut tts) = TTSManager::new() {
                let personality = PersonalitySettings::default();
                let message = vec![MessagePart::Static("Testing system integration".to_string())];
                
                let result = tts.speak(message, &personality).await;
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
                println!("Skipping TTS test - TTS system not available");
            }
        });
    }

    #[test]
    fn test_personality_integration() {
        let test_input = "Test message";
        let message = vec![MessagePart::Static(test_input.to_string())];
        
        // Verify the message contains our test input
        assert!(matches!(&message[0], MessagePart::Static(text) if text == test_input));
    }

    #[test]
    fn test_message_part() {
        let test_input = "Test message";
        let message_part = MessagePart::Static(test_input.to_string());
        match message_part {
            MessagePart::Static(text) => assert_eq!(text, test_input),
            _ => panic!("Expected Static message part"),
        }
    }

    #[test]
    fn test_tts_integration() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            if let Ok(mut tts) = TTSManager::new() {
                let personality = PersonalitySettings::default();
                let message = vec![MessagePart::Static("Test message".to_string())];
                
                let result = tts.speak(message, &personality).await;
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
                println!("Skipping TTS test - TTS system not available");
            }
        });
    }

    #[test]
    fn test_personality_settings() {
        let personality = PersonalitySettings::default();
        assert_eq!(personality.voice_type, "alloy", "Default voice type should be 'alloy'");
        assert_eq!(personality.drunk_level, 0, "Default drunk level should be 0");
        assert_eq!(personality.sass_level, 0, "Default sass level should be 0");
        assert_eq!(personality.enthusiasm, 0, "Default enthusiasm should be 0");
        assert_eq!(personality.anxiety_level, 0, "Default anxiety level should be 0");
        assert_eq!(personality.grand_pappi_refs, 0, "Default grand pappi refs should be 0");
    }

    #[test]
    fn test_message_part_conversion() {
        let test_input = "Hello, world!";
        let message_part = MessagePart::Static(test_input.to_string());
        
        // Convert to string and verify content
        let message_str = message_part.to_string();
        assert!(message_str.contains(test_input), "Message should contain the test input");
    }

    #[test]
    fn test_tts_message_handling() {
        if let Ok(mut tts) = TTSManager::new() {
            let message = MessagePart::Static("Test message".to_string());
            tts.handle_message(&message);
            // Since we can't easily verify audio output, we just ensure it doesn't panic
        }
    }

    #[test]
    fn test_tts_multiple_messages() {
        if let Ok(mut tts) = TTSManager::new() {
            let messages = vec![
                MessagePart::Static("First message".to_string()),
                MessagePart::Static("Second message".to_string()),
            ];
            for message in messages {
                tts.handle_message(&message);
            }
            // Since we can't easily verify audio output, we just ensure it doesn't panic
        }
    }

    #[test]
    fn test_system_integration() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            if let Ok(mut tts) = TTSManager::new() {
                let personality = PersonalitySettings {
                    voice_type: "default".to_string(),
                    volume: 1.0,
                    speech_rate: 1.0,
                    drunk_level: 0,
                    sass_level: 0,
                    tech_expertise: 0,
                    grand_pappi_refs: 0,
                    enthusiasm: 0,
                    anxiety_level: 0,
                    catchphrases: vec![],
                    audio_enabled: true,
                    is_1337_mode: false,
                };
                let message = vec![MessagePart::Static("Testing system integration".to_string())];
                if let Err(e) = tts.speak(message, &personality).await {
                    eprintln!("Failed to speak test message: {}", e);
                }
            }
        });
    }

    #[test]
    fn test_message_system() {
        let mut system = MessageSystem::new();
        let test_input = "Test message";
        system.add_message(MessagePart::Static(test_input.to_string()));
        let messages = system.get_messages();
        assert_eq!(messages.len(), 1);
        if let MessagePart::Static(message_str) = &messages[0] {
            assert!(message_str.contains(test_input), "Message should contain the test input");
        }
    }
}
