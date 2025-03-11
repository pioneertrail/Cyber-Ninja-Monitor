use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessagePart {
    Static(String),
    Dynamic(String),
    Full(String),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CacheKey {
    Static(String, PersonalitySettings),  // Phrase + personality
    Dynamic(String),                      // Descriptive text
    Full(String, String),                // Event type + discretized data
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PersonalitySettings {
    pub drunk_level: i32,
    pub sass_level: i32,
    pub enthusiasm: i32,
    pub anxiety_level: i32,
    pub grand_pappi_refs: i32,
    pub voice_type: String,
}

impl Default for PersonalitySettings {
    fn default() -> Self {
        PersonalitySettings {
            drunk_level: 0,
            sass_level: 0,
            enthusiasm: 0,
            anxiety_level: 0,
            grand_pappi_refs: 0,
            voice_type: "alloy".to_string(),
        }
    }
}

pub struct SystemData {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
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
    
    // Add CPU usage message
    let cpu_msg = format!("CPU usage is at {:.1}%", data.cpu_usage);
    parts.push(MessagePart::Static(cpu_msg));
    
    // Add memory usage message
    let memory_used_gb = data.memory_used as f64 / 1_073_741_824.0;
    let memory_total_gb = data.memory_total as f64 / 1_073_741_824.0;
    let memory_msg = format!("Memory usage: {:.1}GB / {:.1}GB", memory_used_gb, memory_total_gb);
    parts.push(MessagePart::Static(memory_msg));
    
    // Add disk usage message
    let disk_msg = format!("Disk usage is at {:.1}%", data.disk_usage);
    parts.push(MessagePart::Static(disk_msg));
    
    // Add network usage message
    let network_msg = format!("Network: ↓{:.1}MB/s ↑{:.1}MB/s", 
        data.network_rx as f64 / 1_048_576.0,
        data.network_tx as f64 / 1_048_576.0);
    parts.push(MessagePart::Static(network_msg));
    
    parts
}

impl fmt::Display for MessagePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessagePart::Static(text) => write!(f, "{}", text),
            MessagePart::Dynamic(text) => write!(f, "{}", text),
            MessagePart::Full(text) => write!(f, "{}", text),
        }
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
            cpu_usage: 45.0,
            memory_used: 6_000_000_000,
            memory_total: 16_000_000_000,
            disk_usage: 70.0,
            network_rx: 1_048_576_000,
            network_tx: 524_288_000,
        };

        let status_parts = generate_message(&data);
        assert_eq!(status_parts.len(), 4);
        
        if let MessagePart::Static(text) = &status_parts[0] {
            assert_eq!(text, "CPU usage is at 45.0%");
        } else {
            panic!("Expected CPU usage message");
        }

        if let MessagePart::Static(text) = &status_parts[1] {
            assert_eq!(text, "Memory usage: 5.6GB / 14.9GB");
        } else {
            panic!("Expected memory usage message");
        }

        if let MessagePart::Static(text) = &status_parts[2] {
            assert_eq!(text, "Disk usage is at 70.0%");
        } else {
            panic!("Expected disk usage message");
        }

        if let MessagePart::Static(text) = &status_parts[3] {
            assert_eq!(text, "Network: ↓1000.0MB/s ↑500.0MB/s");
        } else {
            panic!("Expected network usage message");
        }
    }
} 