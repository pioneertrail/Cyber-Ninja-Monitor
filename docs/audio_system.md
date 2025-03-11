# CyberNinja Audio System Documentation

## Table of Contents
1. [Core Components](#core-components)
2. [Audio Settings](#audio-settings)
3. [Message Architecture](#message-architecture)
4. [Audio Message Flow](#audio-message-flow)
5. [Personality System](#personality-system)
6. [Event Types](#event-types)
7. [Cache System](#cache-system)
8. [Control States](#control-states)
9. [Implementation Details](#implementation-details)

## Core Components

The audio system consists of four main components that work together:

```mermaid
[TTSManager] <---> [AIPersonality]
     ↓                   ↓
[Audio Cache] <---> [Audio Controls]
     ↓
[OpenAI TTS API]
```

- **TTSManager**: Handles text-to-speech conversion, audio playback, and message composition
- **AIPersonality**: Manages voice characteristics and selective message modifications
- **Audio Cache**: Stores and retrieves audio clips with structured caching
- **Audio Controls**: Provides user interface for audio settings

## Message Architecture

The system uses a modular message architecture for improved reusability:

```rust
enum MessagePart {
    Static(String),    // Unchanging phrases
    Dynamic(String),   // Data-driven descriptions
    Full(String),      // Complete messages (e.g., warnings)
}

enum CacheKey {
    Static(String, PersonalitySettings),  // Phrase + personality
    Dynamic(String),                      // Descriptive text
    Full(String, String),                // Event type + discretized data
}
```

### Message Composition Example
```
Message: "CPU usage is running steady"
├── Static Part: "CPU usage is"
│   └── Applies personality modifications
└── Dynamic Part: "running steady"
    └── Remains neutral for reusability
```

### Qualitative Descriptions
```
Metric Ranges (CPU Example)
├── 0-20%: "running cool"
├── 21-50%: "running steady"
├── 51-80%: "working hard"
└── 81-100%: "running hot"
```

## Audio Message Flow

The system follows an enhanced flow for generating and playing audio:

```
User Action/System Event
         ↓
[Event Trigger] ─────────────┐
         ↓                   ↓
[Message Part Generation] <- [Personality Modifiers]
         ↓                   ↑
[Cache Check] ──────> [Cache Hit?] ─── Yes ──> [Compose Audio]
         ↓              No                          ↓
[OpenAI TTS API Call]    ↑                    [Play Sequence]
         ↓               │
[Cache Storage] ─────────┘
```

## Personality System

The personality system modifies messages based on various traits:

```
Message Transformation Pipeline
├── Base Message
├── Qualitative Conversions
│   └── Numbers to Descriptive Terms
├── Personality Effects
│   ├── Drunk Level (0.0 - 1.0)
│   │   └── "s" -> "sh", "r" -> "rr"
│   ├── Sass Level (0.0 - 1.0)
│   │   └── Random Catchphrase Insertion
│   ├── Grand Pappi References (0.0 - 1.0)
│   │   └── Random Quote Insertion
│   ├── Enthusiasm (0.0 - 1.0)
│   │   └── "." -> "!", Add Emojis
│   └── Anxiety Level (0.0 - 1.0)
│       └── Add "*nervously*", "*fidgets*"
└── Final Message
```

## Event Types

The system responds to various events with appropriate audio messages:

```
System Events
├── Startup
├── Status Updates
│   ├── CPU Status
│   │   └── ["running cool", "running steady", "working hard", "running hot"]
│   └── Memory Status
│       └── ["plenty of space", "comfortable", "getting tight", "very tight"]
├── Warnings
│   ├── CPU Warnings
│   │   └── ["critically high", "very high", "high"]
│   └── Memory Warnings
│       └── ["critically full", "very full", "getting full"]
└── User Interface
    ├── Audio Test
    ├── Warp Mode Toggle
    └── Exit Message
```

## Cache System

The audio cache system uses a structured key system:

```
Cache Structure
├── Key: CacheKey
│   ├── Static(phrase, personality)
│   ├── Dynamic(description)
│   └── Full(event_type, discretized_data)
├── Value: AudioData
│   ├── Audio Buffer
│   ├── Timestamp
│   └── Metadata
└── Cache Operations
    ├── Insert
    ├── Retrieve
    ├── Check Existence
    └── Clear
```

## Control States

The audio system maintains several states:

```
Audio States
├── Enabled/Disabled
├── Muted/Unmuted
├── Normal/Warp Mode
│   └── Speech Rate: Normal(1.0)/Warp(2.0)
└── Volume Level
```

## Implementation Details

### Key Features
- Modular message composition
- Selective personality application
- Structured caching with distinct key types
- Qualitative description system
- Data discretization for warnings
- Composed message playback
- Asynchronous audio handling

### Best Practices
1. Split messages into static and dynamic parts
2. Apply personality only to static parts
3. Use qualitative descriptions for dynamic parts
4. Discretize data for warning messages
5. Cache parts separately for maximum reuse
6. Handle composed message playback smoothly
7. Implement proper error handling

### Performance Considerations
- Cache static and dynamic parts separately
- Use discretized values for warnings
- Optimize message composition
- Handle concurrent audio requests
- Manage cache size efficiently

## Usage Examples

```rust
// Initialize TTS Manager with new message system
let mut tts_manager = TTSManager::new();

// Generate message parts
let parts = generate_message("status_update", &system_data);

// Speak composed message
if let Some(ref mut tts) = tts_manager {
    tts.speak(parts, &mut audio_cache).await;
}

// Generate qualitative description
let cpu_status = get_qualitative_description("cpu", 42.0);
// Returns: "running steady"

// Discretize warning data
let discrete_value = discretize(78.5);
// Returns: "80"
```

## Configuration

The audio system can be configured through:
- Environment variables
- Settings UI
- Personality trait sliders
- Voice type selection
- Qualitative description ranges
- Discretization steps
- Composition delays

## Dependencies
- OpenAI TTS API
- Local audio playback system
- File system for caching
- Async runtime for non-blocking operations
- Message composition system 