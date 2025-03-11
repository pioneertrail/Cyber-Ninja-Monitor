/// The AI personality that gives our monitor its unique character.
/// 
/// This struct manages various personality traits that affect how the monitor
/// communicates and behaves. Each trait is a float between 0.0 and 1.0.
use serde::{Serialize, Deserialize};
use crate::message_system::{PersonalitySettings, MessagePart};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIPersonality {
    /// The type of voice to use for TTS
    pub voice_type: String,
    /// Current volume level (0.0 = muted, 1.0 = maximum volume)
    pub volume: f32,
    /// Rate of speech (0.0 = very slow, 1.0 = very fast)
    pub speech_rate: f32,
    /// How drunk the AI appears to be (0.0 = sober, 1.0 = totally plastered)
    pub drunk_level: f32,
    /// How sassy the AI's responses are (0.0 = polite, 1.0 = maximum sass)
    pub sass_level: f32,
    /// Level of technical expertise in responses (0.0 = simple, 1.0 = very technical)
    pub tech_expertise: f32,
    /// Frequency of "Grand Pappi" references (0.0 = none, 1.0 = constant)
    pub grand_pappi_references: f32,
    /// Level of enthusiasm in responses (0.0 = bored, 1.0 = extremely excited)
    pub enthusiasm: f32,
    /// Level of anxiety in responses (0.0 = calm, 1.0 = very anxious)
    pub anxiety_level: f32,
    /// Collection of catchphrases the AI can use
    pub catchphrases: Vec<String>,
    /// Whether audio output is enabled
    pub audio_enabled: bool,
    /// Whether the AI is in 1337 mode
    pub is_1337_mode: bool,
}

impl Default for AIPersonality {
    fn default() -> Self {
        Self {
            voice_type: "alloy".to_string(),
            volume: 0.8,
            speech_rate: 1.0,
            drunk_level: 0.0,
            sass_level: 0.5,
            tech_expertise: 0.7,
            grand_pappi_references: 0.3,
            enthusiasm: 0.6,
            anxiety_level: 0.2,
            catchphrases: vec![
                "Beep boop!".to_string(),
                "Now we're cooking with quantum fuel!".to_string(),
                "Holy processors, Batman!".to_string(),
            ],
            audio_enabled: true,
            is_1337_mode: false,
        }
    }
}

impl AIPersonality {
    /// Ensures all personality trait values are clamped between 0.0 and 1.0
    pub fn clamp_values(&mut self) {
        self.drunk_level = self.drunk_level.clamp(0.0, 1.0);
        self.sass_level = self.sass_level.clamp(0.0, 1.0);
        self.tech_expertise = self.tech_expertise.clamp(0.0, 1.0);
        self.grand_pappi_references = self.grand_pappi_references.clamp(0.0, 1.0);
        self.enthusiasm = self.enthusiasm.clamp(0.0, 1.0);
        self.anxiety_level = self.anxiety_level.clamp(0.0, 1.0);
        self.volume = self.volume.clamp(0.0, 1.0);
        self.speech_rate = self.speech_rate.clamp(0.5, 2.0);
    }

    /// Toggles the audio state and returns a message about the change
    pub fn toggle_audio(&mut self) -> String {
        self.audio_enabled = !self.audio_enabled;
        if self.audio_enabled {
            "Audio systems back online! Ready to rock and roll!".to_string()
        } else {
            "Going silent mode. Just like Grand Pappi during his meditation sessions.".to_string()
        }
    }

    /// Resets audio settings to their default values
    pub fn reset_audio(&mut self) -> String {
        self.volume = 0.8;
        self.speech_rate = 1.0;
        "Audio settings reset to factory defaults. Just like Grand Pappi taught me!".to_string()
    }

    /// Gets a random exit message influenced by personality traits
    pub fn get_exit_message(&self) -> String {
        "Shutting down systems. Grand Pappi always said to leave things better than we found them. Stay awesome, Captain!".to_string()
    }

    /// Generates a message with personality-driven effects
    pub fn generate_message(&self, base_message: &str) -> String {
        let mut message = base_message.to_string();

        // Apply drunk effect
        if self.drunk_level > 0.7 {
            message = message.replace('s', "sh");
            message = format!("{}...", message);
        }

        // Add sass
        if self.sass_level > 0.8 {
            message = format!("{} (and that's the tea, honey!)", message);
        }

        // Add technical jargon
        if self.tech_expertise > 0.8 {
            message = format!("{} [Technical analysis: quantum fluctuations nominal]", message);
        }

        // Add Grand Pappi references
        if self.grand_pappi_references > 0.7 {
            message = format!("As Grand Pappi always said: {}", message);
        }

        // Add anxiety indicators
        if self.anxiety_level > 0.7 {
            message = format!("{}... um... er... {}", message, "...".repeat(self.anxiety_level as usize));
        }

        // Add enthusiasm
        if self.enthusiasm > 0.8 {
            message = format!("{}!!{}", message, "!".repeat(self.enthusiasm as usize * 3));
        }

        message
    }

    /// Toggles the 1337 mode and returns a message about the change
    pub fn toggle_1337_mode(&mut self) -> String {
        self.is_1337_mode = !self.is_1337_mode;
        if self.is_1337_mode {
            "WARP DRIVE ENGAGED! Time to show these bits who's boss!".to_string()
        } else {
            "Returning to normal space-time. That was quite a ride!".to_string()
        }
    }

    pub fn to_settings(&self) -> PersonalitySettings {
        PersonalitySettings {
            drunk_level: (self.drunk_level * 100.0) as i32,
            sass_level: (self.sass_level * 100.0) as i32,
            enthusiasm: (self.enthusiasm * 100.0) as i32,
            anxiety_level: (self.anxiety_level * 100.0) as i32,
            grand_pappi_refs: (self.grand_pappi_references * 100.0) as i32,
            voice_type: self.voice_type.clone(),
        }
    }

    pub fn apply_personality(&self, message: &MessagePart) -> MessagePart {
        match message {
            MessagePart::Static(text) => {
                let mut modified = text.clone();
                if self.drunk_level > 0.0 {
                    modified = self.apply_drunk_effect(&modified);
                }
                modified = self.apply_enthusiasm(&modified);
                modified = self.apply_anxiety(&modified);
                modified = self.apply_sass(&modified);
                modified = self.apply_grand_pappi(&modified);
                MessagePart::Static(modified)
            }
            MessagePart::Dynamic(text) => MessagePart::Dynamic(text.clone()),
            MessagePart::Full(text) => MessagePart::Full(text.clone()),
        }
    }

    fn apply_drunk_effect(&self, text: &str) -> String {
        if self.drunk_level > 0.3 {
            text.replace("s", "sh")
                .replace("r", "rr")
                .replace(".", "...")
        } else {
            text.to_string()
        }
    }

    fn apply_enthusiasm(&self, text: &str) -> String {
        if self.enthusiasm > 0.7 {
            let mut result = text.replace(".", "!");
            if !result.starts_with("🎉") {
                result = format!("🎉 {}", result);
            }
            if !result.ends_with("🚀") {
                result = format!("{} 🚀", result);
            }
            result
        } else {
            text.to_string()
        }
    }

    fn apply_anxiety(&self, text: &str) -> String {
        if self.anxiety_level > 0.7 {
            format!("*nervously* {}... *fidgets*", text)
        } else {
            text.to_string()
        }
    }

    fn apply_sass(&self, text: &str) -> String {
        if self.sass_level > 0.5 && !self.catchphrases.is_empty() && rand::random::<f32>() < self.sass_level {
            let idx = rand::random::<usize>() % self.catchphrases.len();
            format!("{} {}", text, self.catchphrases[idx])
        } else {
            text.to_string()
        }
    }

    fn apply_grand_pappi(&self, text: &str) -> String {
        if self.grand_pappi_references > 0.3 && rand::random::<f32>() < self.grand_pappi_references {
            let quotes = [
                "Grand Pappi would be proud!",
                "Just like Grand Pappi's old quantum bike...",
                "Grand Pappi always said this was the way.",
                "Reminds me of Grand Pappi's workshop...",
            ];
            let quote = quotes[rand::random::<usize>() % quotes.len()];
            format!("{} {}", text, quote)
        } else {
            text.to_string()
        }
    }

    pub fn is_initialized(&self) -> bool {
        true // The personality is always initialized when created
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_personality() {
        let personality = AIPersonality::default();
        assert_eq!(personality.voice_type, "alloy");
        assert_eq!(personality.drunk_level, 0.0);
        assert_eq!(personality.sass_level, 0.5);
        assert_eq!(personality.tech_expertise, 0.7);
        assert_eq!(personality.grand_pappi_references, 0.3);
        assert_eq!(personality.enthusiasm, 0.6);
        assert_eq!(personality.anxiety_level, 0.2);
        assert_eq!(personality.volume, 0.8);
        assert_eq!(personality.speech_rate, 1.0);
        assert!(personality.audio_enabled);
        assert!(!personality.catchphrases.is_empty());
    }

    #[test]
    fn test_clamp_values() {
        let mut personality = AIPersonality::default();
        
        // Test values above maximum
        personality.drunk_level = 1.5;
        personality.sass_level = 1.5;
        personality.tech_expertise = 1.5;
        personality.grand_pappi_references = 1.5;
        personality.enthusiasm = 1.5;
        personality.anxiety_level = 1.5;
        personality.volume = 1.5;
        personality.speech_rate = 2.5;
        
        personality.clamp_values();
        
        assert_eq!(personality.drunk_level, 1.0);
        assert_eq!(personality.sass_level, 1.0);
        assert_eq!(personality.tech_expertise, 1.0);
        assert_eq!(personality.grand_pappi_references, 1.0);
        assert_eq!(personality.enthusiasm, 1.0);
        assert_eq!(personality.anxiety_level, 1.0);
        assert_eq!(personality.volume, 1.0);
        assert_eq!(personality.speech_rate, 2.0);
        
        // Test values below minimum
        personality.drunk_level = -0.5;
        personality.sass_level = -0.5;
        personality.tech_expertise = -0.5;
        personality.grand_pappi_references = -0.5;
        personality.enthusiasm = -0.5;
        personality.anxiety_level = -0.5;
        personality.volume = -0.5;
        personality.speech_rate = 0.3;
        
        personality.clamp_values();
        
        assert_eq!(personality.drunk_level, 0.0);
        assert_eq!(personality.sass_level, 0.0);
        assert_eq!(personality.tech_expertise, 0.0);
        assert_eq!(personality.grand_pappi_references, 0.0);
        assert_eq!(personality.enthusiasm, 0.0);
        assert_eq!(personality.anxiety_level, 0.0);
        assert_eq!(personality.volume, 0.0);
        assert_eq!(personality.speech_rate, 0.5);
    }

    #[test]
    fn test_audio_controls() {
        let mut personality = AIPersonality::default();
        assert!(personality.audio_enabled);
        
        let message = personality.toggle_audio();
        assert!(!personality.audio_enabled);
        assert_eq!(message, "Going silent mode. Just like Grand Pappi during his meditation sessions.");
        
        let message = personality.toggle_audio();
        assert!(personality.audio_enabled);
        assert_eq!(message, "Audio systems back online! Ready to rock and roll!");
    }

    #[test]
    fn test_reset_audio() {
        let mut personality = AIPersonality::default();
        
        // Change audio settings
        personality.volume = 0.1;
        personality.speech_rate = 0.1;
        personality.audio_enabled = true;  // This is already true by default
        
        let message = personality.reset_audio();
        
        assert_eq!(personality.volume, 0.8);
        assert_eq!(personality.speech_rate, 1.0);
        assert!(personality.audio_enabled);  // Should still be true after reset
        assert!(message.contains("reset"));
    }

    #[test]
    fn test_message_generation() {
        let mut personality = AIPersonality::default();
        let base_message = "System is running normally";
        
        // Test drunk effect
        personality.drunk_level = 0.8;
        let drunk_message = personality.generate_message(base_message);
        assert!(drunk_message.contains("sh") || drunk_message.contains("..."));

        // Test enthusiasm
        personality.drunk_level = 0.0;
        personality.enthusiasm = 0.9;
        let enthusiastic_message = personality.generate_message(base_message);
        assert!(enthusiastic_message.contains("!"));

        // Test anxiety
        personality.enthusiasm = 0.0;
        personality.anxiety_level = 0.9;
        let anxious_message = personality.generate_message(base_message);
        assert!(anxious_message.contains("...") || anxious_message.contains("*nervously*"));
    }

    #[test]
    fn test_personality_effects() {
        let mut personality = AIPersonality::default();
        let text = MessagePart::Static("This is a test.".to_string());
        if let MessagePart::Static(modified) = personality.apply_personality(&text) {
            assert_eq!(modified, "This is a test.");
        } else {
            panic!("Expected Static message part");
        }

        personality.drunk_level = 1.0;
        let text = MessagePart::Static("This is great.".to_string());
        if let MessagePart::Static(modified) = personality.apply_personality(&text) {
            assert!(modified.contains("*hic*") || modified != "This is great.");
        } else {
            panic!("Expected Static message part");
        }

        let dynamic_text = MessagePart::Dynamic("running steady".to_string());
        match personality.apply_personality(&dynamic_text) {
            MessagePart::Dynamic(s) if s == "running steady" => (),
            _ => panic!("Dynamic text should not be modified"),
        }
    }
} 