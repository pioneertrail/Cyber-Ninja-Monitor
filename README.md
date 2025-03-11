# Cyber Ninja Monitor

A cyberpunk-themed system monitoring application with personality! Built with Rust and eframe.

## Features

- Real-time system monitoring (CPU, Memory, Disk, Network)
- Text-to-speech notifications with customizable AI personality
- Cyberpunk-inspired UI with dynamic effects
- Customizable personality traits:
  - Voice type selection
  - Drunk level
  - Sass level
  - Tech expertise
  - Grand Pappi references
  - Enthusiasm
  - Anxiety level
- Custom catchphrases
- Audio controls with volume and speech rate adjustment

## Requirements

- Rust 1.70 or higher
- OpenAI API key (for TTS functionality)
- Windows 10 or higher

## Setup

1. Clone the repository
2. Create a `.env` file in the root directory with your OpenAI API key:
   ```
   OPENAI_API_KEY=your_api_key_here
   ```
3. Build and run:
   ```bash
   cargo build
   cargo run
   ```

## Usage

1. Launch the application
2. Click the settings icon (⚙) to open the personality settings
3. Adjust voice type, personality traits, and add catchphrases
4. Use the test audio button to preview your settings
5. Monitor your system with style!

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

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

## Acknowledgments

- Inspired by Star Trek's LCARS interface
- Built with Rust's amazing ecosystem
- Special thanks to the egui community 