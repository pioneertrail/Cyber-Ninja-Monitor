/// The AI personality that gives our monitor its unique character.
/// 
/// This struct manages various personality traits that affect how the monitor
/// communicates and behaves. Each trait is a float between 0.0 and 1.0.
#[derive(Debug, Clone)]
pub struct AIPersonality {
    /// The type of voice to use for TTS
    pub voice_type: String,
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
    /// Current volume level (0.0 = muted, 1.0 = maximum volume)
    pub volume: f32,
    /// Rate of speech (0.0 = very slow, 1.0 = very fast)
    pub speech_rate: f32,
    /// Whether audio output is enabled
    pub audio_enabled: bool,
    /// Collection of catchphrases the AI can use
    pub catchphrases: Vec<String>,
    /// Collection of exit messages for when the program closes
    pub exit_messages: Vec<String>,
    /// Whether the AI is in 1337 mode
    pub is_1337_mode: bool,
}

impl Default for AIPersonality {
    fn default() -> Self {
        Self {
            voice_type: "nova".to_string(),
            drunk_level: 0.0,
            sass_level: 0.5,
            tech_expertise: 0.7,
            grand_pappi_references: 0.3,
            enthusiasm: 0.8,
            anxiety_level: 0.2,
            volume: 0.7,
            speech_rate: 0.5,
            audio_enabled: true,
            catchphrases: vec![
                "Aye, yer CPU's running hotter than a haggis in a microwave!".to_string(),
                "By Grand Pappi's quantum abacus!".to_string(),
                "Looks like yer RAM's been hitting the digital pub again...".to_string(),
                "Time to monitor ALL the things!".to_string(),
            ],
            exit_messages: vec![
                "Off to the digital pub!".to_string(),
                "Grand Pappi would be proud...".to_string(),
                "Time to power down these quantum circuits!".to_string(),
                "Catch you on the flip side of the motherboard!".to_string(),
            ],
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
            "Audio enabled. Time to make some noise!".to_string()
        } else {
            "Audio disabled. Entering ninja-quiet mode...".to_string()
        }
    }

    /// Resets audio settings to their default values
    pub fn reset_audio(&mut self) -> String {
        self.volume = 0.7;
        self.speech_rate = 0.5;
        self.enthusiasm = 0.8;
        self.anxiety_level = 0.2;
        self.audio_enabled = true;
        "Audio settings reset to defaults. Feeling balanced and centered!".to_string()
    }

    /// Gets a random exit message influenced by personality traits
    pub fn get_exit_message(&self) -> String {
        "Shutting down... Remember, a ninja's work is never done, but even ninjas need their rest!".to_string()
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
            // Max out drunk level and speed for warp effects
            self.drunk_level = 1.0;
            self.speech_rate = 2.0;
            "W4RP DR1V3 3NG4G3D! *hic* All systems running at maximum efficiency! Prepare for quantum acceleration!"
        } else {
            // Reset to previous state
            self.drunk_level = 0.3;
            self.speech_rate = 1.0;
            "Disengaging warp drive. Returning to normal space-time parameters. Crew recovery protocols initiated."
        }.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_personality() {
        let personality = AIPersonality::default();
        assert_eq!(personality.voice_type, "nova");
        assert_eq!(personality.drunk_level, 0.0);
        assert_eq!(personality.sass_level, 0.5);
        assert_eq!(personality.tech_expertise, 0.7);
        assert_eq!(personality.grand_pappi_references, 0.3);
        assert_eq!(personality.enthusiasm, 0.8);
        assert_eq!(personality.anxiety_level, 0.2);
        assert_eq!(personality.volume, 0.7);
        assert_eq!(personality.speech_rate, 0.5);
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
        assert_eq!(message, "Audio disabled. Entering ninja-quiet mode...");
        
        let message = personality.toggle_audio();
        assert!(personality.audio_enabled);
        assert_eq!(message, "Audio enabled. Time to make some noise!");
    }

    #[test]
    fn test_reset_audio() {
        let mut personality = AIPersonality::default();
        
        // Change audio settings
        personality.volume = 0.1;
        personality.speech_rate = 0.1;
        personality.audio_enabled = false;
        
        let message = personality.reset_audio();
        
        assert_eq!(personality.volume, 0.7);
        assert_eq!(personality.speech_rate, 0.5);
        assert!(personality.audio_enabled);
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
} 