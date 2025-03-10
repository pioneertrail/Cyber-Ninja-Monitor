# CyberNinja System Monitor

A cyberpunk-themed system monitoring application built with Rust, featuring real-time system stats, voice feedback, and Star Trek-inspired warp drive effects.

## Official Documentation References

### Core Technologies
- [Rust Programming Language](https://doc.rust-lang.org/book/) - The Rust Book
- [Rust Standard Library](https://doc.rust-lang.org/std/) - Standard library documentation
- [Cargo Book](https://doc.rust-lang.org/cargo/) - Rust's package manager
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learning Rust through examples
- [Rust Reference](https://doc.rust-lang.org/reference/) - Detailed language reference

### GUI Framework
- [egui](https://docs.rs/egui/) - Immediate mode GUI framework
- [eframe](https://docs.rs/eframe/) - egui framework wrapper
- [egui Demo App](https://www.egui.rs/#demo) - Official examples and demos
- [epaint](https://docs.rs/epaint/) - egui's painting primitives

### System Monitoring
- [sysinfo](https://docs.rs/sysinfo/) - System information gathering
- [chrono](https://docs.rs/chrono/) - Date and time functionality
- [Windows Performance Counters](https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-portal) - Windows system monitoring
- [Linux Proc Documentation](https://www.kernel.org/doc/html/latest/filesystems/proc.html) - Linux system statistics

### Audio and Text-to-Speech
- [rodio](https://docs.rs/rodio/) - Audio playback
- [OpenAI API Documentation](https://platform.openai.com/docs/api-reference/audio) - TTS API
- [OpenAI Text-to-Speech Guide](https://platform.openai.com/docs/guides/text-to-speech) - TTS implementation guide
- [OpenAI Models Documentation](https://platform.openai.com/docs/models) - Available voice models

### Graphics and Styling
- [tiny-skia](https://docs.rs/tiny-skia/) - 2D rendering
- [usvg](https://docs.rs/usvg/) - SVG loading and parsing
- [resvg](https://docs.rs/resvg/) - SVG rendering
- [SVG 2 Specification](https://www.w3.org/TR/SVG2/) - SVG format reference
- [WebGL Fundamentals](https://webglfundamentals.org/) - Graphics programming concepts

### Networking and Data
- [reqwest](https://docs.rs/reqwest/) - HTTP client
- [dotenv](https://docs.rs/dotenv/) - Environment variable management
- [tokio](https://docs.rs/tokio/) - Async runtime
- [serde](https://docs.rs/serde/) - Serialization framework

### Animation and Effects
- [Game Programming Patterns](https://gameprogrammingpatterns.com/) - Animation and game loop patterns
- [WebGL 2 Shaders](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL)) - Shader programming reference
- [Real-Time Rendering](https://www.realtimerendering.com/) - Graphics effects documentation

### Testing and Development
- [Rust Test Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html) - Testing in Rust
- [Rust Async Book](https://rust-lang.github.io/async-book/) - Asynchronous programming
- [Rust Performance Book](https://nnethercote.github.io/perf-book/) - Performance optimization

## Features

- Real-time system monitoring
  - CPU usage
  - Memory usage
  - Disk usage
  - Network statistics
  - Process monitoring

- Interactive UI
  - Cyberpunk-themed design
  - Animated grid effects
  - Neon color schemes
  - Responsive layouts

- Voice Feedback
  - Text-to-speech notifications
  - Personality-driven responses
  - Configurable voice settings

- Special Effects
  - Star Trek-inspired warp drive visualization
  - Dynamic lighting and animations
  - Glitch effects for alerts
  - Spinning shurikens

## Configuration

### Environment Variables
```env
OPENAI_API_KEY=your_api_key_here
```

### Audio Settings
- Voice Type: "nova" (OpenAI TTS voice)
- Volume Range: 0.0 - 1.0
- Speech Rate: 0.5 - 2.0

### Personality Traits
- Drunk Level: 0.0 - 1.0
- Sass Level: 0.0 - 1.0
- Tech Expertise: 0.0 - 1.0
- Enthusiasm: 0.0 - 1.0
- Anxiety Level: 0.0 - 1.0

## Building and Running

1. Install Rust and Cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository
```bash
git clone <repository_url>
cd cyberninjaapp
```

3. Set up environment variables
```bash
cp .env.example .env
# Edit .env with your OpenAI API key
```

4. Build and run
```bash
cargo run --release
```

## Contributing

Please read our contributing guidelines and code of conduct before submitting pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by Star Trek's LCARS interface
- Built with Rust's amazing ecosystem
- Special thanks to the egui community 