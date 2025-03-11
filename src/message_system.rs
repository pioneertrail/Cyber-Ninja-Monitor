use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub enum MessagePart {
    Static(String),
    Dynamic(String),
    Full(String),
}

impl MessagePart {
    pub fn text(&self) -> &str {
        match self {
            MessagePart::Static(text) => text,
            MessagePart::Dynamic(text) => text,
            MessagePart::Full(text) => text,
        }
    }

    pub fn static_text(text: String) -> Self {
        MessagePart::Static(text)
    }
}

pub struct MessageSystem {
    messages: Vec<MessagePart>,
}

impl MessageSystem {
    pub fn new() -> Self {
        MessageSystem {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, part: MessagePart) {
        self.messages.push(part);
    }

    pub fn get_messages(&self) -> &[MessagePart] {
        &self.messages
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CacheKey {
    Static(String, PersonalitySettings),  // Phrase + personality
    Dynamic(String),                      // Descriptive text
    Full(String, String),                // Event type + discretized data
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersonalitySettings {
    pub voice_type: String,
    pub volume: f32,
    pub speech_rate: f32,
    pub drunk_level: i32,
    pub sass_level: i32,
    pub tech_expertise: i32,
    pub grand_pappi_refs: i32,
    pub enthusiasm: i32,
    pub anxiety_level: i32,
    pub catchphrases: Vec<String>,
    pub audio_enabled: bool,
    pub is_1337_mode: bool,
}

impl Default for PersonalitySettings {
    fn default() -> Self {
        PersonalitySettings {
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
        }
    }
}

impl PartialEq for PersonalitySettings {
    fn eq(&self, other: &Self) -> bool {
        self.voice_type == other.voice_type &&
        (self.volume - other.volume).abs() < f32::EPSILON &&
        (self.speech_rate - other.speech_rate).abs() < f32::EPSILON &&
        self.drunk_level == other.drunk_level &&
        self.sass_level == other.sass_level &&
        self.tech_expertise == other.tech_expertise &&
        self.grand_pappi_refs == other.grand_pappi_refs &&
        self.enthusiasm == other.enthusiasm &&
        self.anxiety_level == other.anxiety_level &&
        self.catchphrases == other.catchphrases &&
        self.audio_enabled == other.audio_enabled &&
        self.is_1337_mode == other.is_1337_mode
    }
}

impl Eq for PersonalitySettings {}

impl std::hash::Hash for PersonalitySettings {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.voice_type.hash(state);
        (self.volume.to_bits()).hash(state);
        (self.speech_rate.to_bits()).hash(state);
        self.drunk_level.hash(state);
        self.sass_level.hash(state);
        self.tech_expertise.hash(state);
        self.grand_pappi_refs.hash(state);
        self.enthusiasm.hash(state);
        self.anxiety_level.hash(state);
        self.catchphrases.hash(state);
        self.audio_enabled.hash(state);
        self.is_1337_mode.hash(state);
    }
}

pub struct SystemData {
    pub cpu_usage: Vec<(String, f32)>,
    pub memory_total: u64,
    pub memory_used: u64,
    pub memory_usage: f32,
    pub disk_total: u64,
    pub disk_available: u64,
    pub disk_usage: f32,
    pub network_rx: u64,
    pub network_tx: u64,
}

pub fn get_qualitative_description(metric: &str, value: f32) -> String {
    match metric {
        "cpu" => match value {
            v if v <= 20.0 => "running cool",
            v if v <= 50.0 => "running steady",
            v if v <= 80.0 => "working hard",
            _ => "running hot",
        },
        "memory" => match value {
            v if v <= 30.0 => "plenty of space",
            v if v <= 60.0 => "comfortable",
            v if v <= 80.0 => "getting tight",
            _ => "very tight",
        },
        "disk" => match value {
            v if v <= 50.0 => "lots of room",
            v if v <= 75.0 => "decent space",
            v if v <= 90.0 => "filling up",
            _ => "nearly full",
        },
        _ => "unknown",
    }.to_string()
}

pub fn discretize(value: f32) -> String {
    let rounded = (value / 5.0).round() * 5.0;
    format!("{:.0}", rounded)
}

pub fn generate_message(data: &SystemData) -> Vec<MessagePart> {
    let mut parts = Vec::new();

    // CPU Usage
    for (_name, usage) in &data.cpu_usage {
        parts.push(MessagePart::Static(format!("CPU Usage: {:.1}%", usage)));
    }

    // Memory Usage
    let memory_text = format!(
        "Memory: {:.1}GB/{:.1}GB ({:.1}%)",
        data.memory_used as f64 / 1_073_741_824.0,
        data.memory_total as f64 / 1_073_741_824.0,
        data.memory_usage,
    );
    parts.push(MessagePart::Static(memory_text));

    // Disk Usage
    let disk_text = format!(
        "Disk: {:.1}GB/{:.1}GB ({:.1}%)",
        data.disk_available as f64 / 1_073_741_824.0,
        data.disk_total as f64 / 1_073_741_824.0,
        data.disk_usage,
    );
    parts.push(MessagePart::Static(disk_text));

    // Network Usage
    let network_text = format!(
        "Network: {:.1}MB/s Up, {:.1}MB/s Down",
        data.network_tx as f64 / 1_048_576.0,
        data.network_rx as f64 / 1_048_576.0,
    );
    parts.push(MessagePart::Static(network_text));

    parts
}

pub fn generate_status_message(cpu: f32, memory: f32, disk: f32, network: f32) -> Vec<MessagePart> {
    vec![
        MessagePart::Static(format!("CPU Usage: {:.1}%", cpu)),
        MessagePart::Static(format!("Memory Usage: {:.1}%", memory)),
        MessagePart::Static(format!("Disk Usage: {:.1}%", disk)),
        MessagePart::Static(format!("Network Usage: {:.1}%", network)),
    ]
}

impl fmt::Display for MessagePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qualitative_description() {
        assert_eq!(get_qualitative_description("cpu", 15.0), "running cool");
        assert_eq!(get_qualitative_description("cpu", 45.0), "running steady");
        assert_eq!(get_qualitative_description("cpu", 75.0), "working hard");
        assert_eq!(get_qualitative_description("cpu", 90.0), "running hot");
    }

    #[test]
    fn test_discretize() {
        assert_eq!(discretize(42.7), "45");
        assert_eq!(discretize(78.2), "80");
        assert_eq!(discretize(91.6), "90");
    }

    #[test]
    fn test_message_generation() {
        let data = SystemData {
            cpu_usage: vec![("CPU0".to_string(), 45.0)],
            memory_total: 16_000_000_000,
            memory_used: 8_000_000_000,
            memory_usage: 50.0,
            disk_total: 500_000_000_000,
            disk_available: 250_000_000_000,
            disk_usage: 50.0,
            network_rx: 1_000_000,
            network_tx: 500_000,
        };

        let status_parts = generate_message(&data);
        assert_eq!(status_parts.len(), 4);
        
        if let MessagePart::Static(text) = &status_parts[0] {
            assert_eq!(text, "CPU Usage: 45.0%");
        } else {
            panic!("Expected CPU usage message");
        }

        if let MessagePart::Static(text) = &status_parts[1] {
            assert_eq!(text, "Memory: 50.0%");
            assert_eq!(text, "8.0 GB / 16.0 GB");
        } else {
            panic!("Expected memory usage message");
        }

        if let MessagePart::Static(text) = &status_parts[2] {
            assert_eq!(text, "Disk: 50.0%");
            assert_eq!(text, "250.0 GB free of 500.0 GB");
        } else {
            panic!("Expected disk usage message");
        }

        if let MessagePart::Static(text) = &status_parts[3] {
            assert_eq!(text, "Network: ↓1.0 MB/s ↑0.5 MB/s");
        } else {
            panic!("Expected network usage message");
        }
    }

    #[test]
    fn test_status_message_generation() {
        let data = SystemData {
            cpu_usage: vec![("CPU0".to_string(), 45.0)],
            memory_total: 16_000_000_000,
            memory_used: 8_000_000_000,
            memory_usage: 50.0,
            disk_total: 500_000_000_000,
            disk_available: 250_000_000_000,
            disk_usage: 50.0,
            network_rx: 1_000_000,
            network_tx: 500_000,
        };

        let status_parts = generate_message(&data);
        assert_eq!(status_parts.len(), 4);

        if let MessagePart::Static(text) = &status_parts[0] {
            assert!(text.contains("CPU Usage: 45.0%"));
        } else {
            panic!("Expected Static message part");
        }

        if let MessagePart::Static(text) = &status_parts[1] {
            assert!(text.contains("Memory: 50.0%"));
            assert!(text.contains("8.0 GB / 16.0 GB"));
        } else {
            panic!("Expected Static message part");
        }

        if let MessagePart::Static(text) = &status_parts[2] {
            assert!(text.contains("Disk: 50.0%"));
            assert!(text.contains("250.0 GB free of 500.0 GB"));
        } else {
            panic!("Expected Static message part");
        }

        if let MessagePart::Static(text) = &status_parts[3] {
            assert!(text.contains("Network: ↓1.0 MB/s ↑0.5 MB/s"));
        } else {
            panic!("Expected Static message part");
        }
    }
} 