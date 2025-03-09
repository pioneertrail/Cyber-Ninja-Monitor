use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use rodio::{OutputStream, Sink};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use std::error::Error;

#[derive(Serialize)]
struct TTSRequest {
    model: String,
    input: String,
    voice: String,
}

pub struct TTSManager {
    api_key: String,
    client: Client,
    _stream: OutputStream,
    sink: Sink,
    volume: f32,
}

impl TTSManager {
    pub fn new(api_key: String) -> Result<Self, Box<dyn Error>> {
        println!("Initializing TTS Manager...");
        let client = Client::new();
        let (stream, stream_handle) = OutputStream::try_default()?;
        println!("Audio output stream initialized");
        let sink = Sink::try_new(&stream_handle)?;
        println!("Audio sink created");
        sink.set_volume(0.8);
        println!("Initial volume set to 0.8");
        
        Ok(Self {
            api_key,
            client,
            _stream: stream,
            sink,
            volume: 0.8,
        })
    }

    pub fn set_volume(&mut self, volume: f32) {
        println!("Setting TTS volume to {}", volume);
        self.sink.set_volume(volume);
        self.volume = volume;
    }

    pub fn speak(&self, text: &str) -> Result<(), Box<dyn Error>> {
        println!("TTS attempting to speak: {}", text);
        
        // Don't make a new request if the sink is already playing
        if !self.sink.empty() {
            println!("TTS is already playing, skipping");
            return Ok(());
        }

        println!("Making API request to OpenAI TTS service...");
        let request = TTSRequest {
            model: "tts-1".to_string(),
            input: text.to_string(),
            voice: "nova".to_string(),
        };

        let response = self.client
            .post("https://api.openai.com/v1/audio/speech")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let error_text = response.text()?;
            println!("API Error: {}", error_text);
            return Err(format!("API Error: {}", error_text).into());
        }

        println!("Received response from API, reading audio data...");
        let audio_data = response.bytes()?;
        
        println!("Creating audio source...");
        let cursor = Cursor::new(audio_data);
        let source = match rodio::Decoder::new(cursor) {
            Ok(s) => s,
            Err(e) => {
                println!("Error decoding audio: {}", e);
                return Err(Box::new(e));
            }
        };

        println!("Playing audio with volume {}", self.volume);
        self.sink.append(source);
        self.sink.set_volume(self.volume);
        self.sink.play();
        println!("Audio playback started");

        Ok(())
    }

    pub fn is_speaking(&self) -> bool {
        !self.sink.empty()
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
} 