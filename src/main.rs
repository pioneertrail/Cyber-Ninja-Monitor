use eframe::egui::{self, Color32, RichText, Vec2, Pos2, Align2, FontId, Stroke, ViewportBuilder, Rect};
use sysinfo::{System, Networks, SystemExt, NetworksExt, CpuExt, NetworkExt};
use std::time::{Duration, Instant};
use rand::Rng;
use chrono::Local;
use std::collections::VecDeque;
use std::f32::consts::PI;
use dotenv::dotenv;
use std::env;
use crate::tts::TTSManager;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
mod tts;

const LOGO: &str = r#"
   ██████╗██╗   ██╗██████╗ ███████╗██████╗     ███╗   ██╗██╗███╗   ██╗     ██╗ █████╗ 
  ██╔════╝╚██╗ ██╔╝██╔══██╗██╔════╝██╔══██╗    ████╗  ██║██║████╗  ██║     ██║██╔══██╗
  ██║      ╚████╔╝ ██████╔╝█████╗  ██████╔╝    ██╔██╗ ██║██║██╔██╗ ██║     ██║███████║
  ██║       ╚██╔╝  ██╔══██╗██╔══╝  ██╔══██╗    ██║╚██╗██║██║██║╚██╗██║██   ██║██╔══██║
  ╚██████╗   ██║   ██████╔╝███████╗██║  ██║    ██║ ╚████║██║██║ ╚████║╚█████╔╝██║  ██║
   ╚═════╝   ╚═╝   ╚═════╝ ╚══════╝╚═╝  ╚═╝    ╚═╝  ╚═══╝╚═╝╚═╝  ╚═══╝ ╚════╝ ╚═╝  ╚═╝"#;

// Digital Rain Effect
struct DigitalRain {
    drops: Vec<(f32, f32, String, f32)>, // x, y, char, speed
    chars: Vec<char>,
}

impl DigitalRain {
    fn new() -> Self {
        let chars: Vec<char> = "ﾊﾐﾋｰｳｼﾅﾓﾆｻﾜﾂｵﾘｱﾎﾃﾏｹﾒｴｶｷﾑﾕﾗｾﾈｽﾀﾇﾍ".chars().collect();
        Self {
            drops: Vec::new(),
            chars,
        }
    }

    fn update(&mut self, rect: egui::Rect, ui: &mut egui::Ui) {
        let painter = ui.painter();
        let mut rng = rand::thread_rng();
        
        // Spawn new drops
        if self.drops.len() < 100 && rng.gen::<f32>() < 0.1 {
            let x = rng.gen::<f32>() * rect.width();
            self.drops.push((
                x, 
                0.0, 
                self.chars[rng.gen_range(0..self.chars.len())].to_string(),
                1.0 + rng.gen::<f32>() * 2.0
            ));
        }

        // Update and draw drops
        self.drops.retain_mut(|(x, y, c, speed)| {
            *y += *speed;
            let alpha = ((rect.height() - *y) / rect.height()).max(0.0);
            painter.text(
                Pos2::new(*x, *y),
                Align2::CENTER_CENTER,
                c,
                FontId::monospace(14.0),
                Color32::from_rgba_unmultiplied(0, 255, 100, (alpha * 255.0) as u8),
            );
            *y < rect.height()
        });
    }
}

// Hacker Terminal
#[derive(Default)]
struct TerminalEntry {
    text: String,
    timestamp: String,
    is_command: bool,
}

struct HackerTerminal {
    input: String,
    history: VecDeque<TerminalEntry>,
    cursor_blink: bool,
    last_blink: Instant,
}

impl HackerTerminal {
    fn new() -> Self {
        let mut terminal = Self {
            input: String::new(),
            history: VecDeque::with_capacity(100),
            cursor_blink: true,
            last_blink: Instant::now(),
        };
        
        terminal.add_entry("[*] CyberNinja Terminal v1.337 initialized...", false);
        terminal.add_entry("[*] Type 'help' for available commands", false);
        terminal
    }

    fn add_entry(&mut self, text: &str, is_command: bool) {
        let timestamp = Local::now().format("%H:%M:%S").to_string();
        self.history.push_back(TerminalEntry {
            text: text.to_string(),
            timestamp,
            is_command,
        });
        
        while self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        let term_rect = ui.available_rect_before_wrap();
        ui.painter().rect_filled(
            term_rect,
            3.0,
            Color32::from_rgba_unmultiplied(0, 20, 0, 200),
        );

        ui.vertical(|ui| {
            // Terminal history
            for entry in &self.history {
                let prefix = if entry.is_command { ">" } else { "" };
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&entry.timestamp)
                        .color(Color32::from_rgb(100, 100, 100))
                        .monospace());
                    ui.label(RichText::new(format!("{} {}", prefix, entry.text))
                        .color(if entry.is_command {
                            Color32::from_rgb(0, 255, 0)
                        } else {
                            Color32::from_rgb(200, 200, 200)
                        })
                        .monospace());
                });
            }

            // Input line
            if self.last_blink.elapsed() > Duration::from_millis(500) {
                self.cursor_blink = !self.cursor_blink;
                self.last_blink = Instant::now();
            }

            let cursor = if self.cursor_blink { "█" } else { " " };
            ui.horizontal(|ui| {
                ui.label(RichText::new("root@cyberninja:~# ")
                    .color(Color32::from_rgb(255, 50, 50))
                    .monospace());
                let response = ui.text_edit_singleline(&mut self.input);
                ui.label(RichText::new(cursor)
                    .color(Color32::from_rgb(0, 255, 0))
                    .monospace());

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.execute_command();
                }
            });
        });
    }

    fn execute_command(&mut self) {
        let cmd = self.input.trim().to_lowercase();
        let input_copy = self.input.clone();
        self.add_entry(&input_copy, true);
        
        let response = match cmd.as_str() {
            "help" => "Available commands: help, scan, hack, clear, status, matrix",
            "scan" => "[!] Scanning network...\n[+] Found 1337 vulnerable systems\n[+] Backdoor opportunities detected",
            "hack" => {
                "ACCESS GRANTED - Welcome to the mainframe\n\
                [*] Bypassing security...\n\
                [*] Injecting payload...\n\
                [+] System compromised!"
            },
            "clear" => {
                self.history.clear();
                "Terminal cleared."
            },
            "status" => "[*] All systems operational\n[*] Stealth systems: ACTIVE\n[*] Neural interface: CONNECTED",
            "matrix" => "There is no spoon...",
            _ => "Command not recognized. Type 'help' for available commands.",
        };
        
        for line in response.lines() {
            self.add_entry(line, false);
        }
        self.input.clear();
    }
}

// Add color palette struct for dynamic colors
struct ColorPalette {
    primary: Color32,
    secondary: Color32,
    accent1: Color32,
    accent2: Color32,
    background: Color32,
}

impl ColorPalette {
    fn cyberpunk() -> Self {
        Self {
            primary: Color32::from_rgb(0, 255, 100),    // Neon green
            secondary: Color32::from_rgb(255, 0, 100),  // Hot pink
            accent1: Color32::from_rgb(0, 200, 255),    // Cyan
            accent2: Color32::from_rgb(255, 100, 0),    // Orange
            background: Color32::from_rgba_unmultiplied(10, 10, 20, 200),
        }
    }

    fn with_alpha(&self, color: Color32, alpha: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
    }

    fn pulse_color(&self, color: Color32, pulse: f32, base_alpha: u8) -> Color32 {
        let alpha = (base_alpha as f32 * pulse) as u8;
        self.with_alpha(color, alpha)
    }
}

// Add new structs for visual effects
struct HexGrid {
    time: f32,
    cells: Vec<(Pos2, f32)>, // Position and phase
}

impl HexGrid {
    fn new() -> Self {
        Self {
            time: 0.0,
            cells: Vec::new(),
        }
    }

    fn update(&mut self, rect: Rect, ui: &mut egui::Ui, palette: &ColorPalette, pulse: f32) {
        self.time += 0.016; // Assume 60fps
        let hex_size = 30.0;
        let hex_spacing = hex_size * 1.5;
        let vert_spacing = hex_size * 0.866; // sqrt(3)/2

        // Generate hex grid positions
        self.cells.clear();
        let mut y = rect.min.y - hex_spacing;
        let mut row = 0;
        while y < rect.max.y + hex_spacing {
            let offset = if row % 2 == 0 { 0.0 } else { hex_spacing / 2.0 };
            let mut x = rect.min.x - hex_spacing + offset;
            while x < rect.max.x + hex_spacing {
                let phase = (x + y) * 0.01 + self.time * 0.5;
                self.cells.push((Pos2::new(x, y), phase));
                x += hex_spacing;
            }
            y += vert_spacing;
            row += 1;
        }

        // Draw hexagons with varied colors
        let painter = ui.painter();
        for (pos, phase) in &self.cells {
            let points = (0..6).map(|i| {
                let angle = i as f32 * PI / 3.0;
                let x = pos.x + angle.cos() * hex_size * 0.5;
                let y = pos.y + angle.sin() * hex_size * 0.5;
                Pos2::new(x, y)
            }).collect::<Vec<_>>();

            let alpha = (phase.sin() * 0.3 + 0.1).max(0.0);
            let color = if phase.sin() > 0.5 {
                palette.primary
            } else {
                palette.accent1
            };
            
            painter.add(egui::Shape::convex_polygon(
                points,
                palette.with_alpha(color, (alpha * 255.0) as u8),
                Stroke::new(1.0, palette.with_alpha(color, (alpha * 255.0) as u8)),
            ));
        }
    }
}

struct ScanLine {
    position: f32,
    speed: f32,
}

impl ScanLine {
    fn new() -> Self {
        Self {
            position: 0.0,
            speed: 100.0,
        }
    }

    fn update(&mut self, rect: Rect, ui: &mut egui::Ui, palette: &ColorPalette) {
        self.position += self.speed * 0.016;
        if self.position > rect.height() {
            self.position = 0.0;
        }

        let y = rect.min.y + self.position;
        let painter = ui.painter();
        
        // Draw scan line with accent color
        painter.line_segment(
            [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
            Stroke::new(2.0, palette.with_alpha(palette.accent2, 100)),
        );

        // Draw glow effect with primary color
        for i in 0..5 {
            let alpha = (1.0 - i as f32 * 0.2) * 50.0;
            let offset = i as f32 * 2.0;
            let color = palette.with_alpha(palette.primary, alpha as u8);
            
            painter.line_segment(
                [Pos2::new(rect.min.x, y + offset), Pos2::new(rect.max.x, y + offset)],
                Stroke::new(1.0, color),
            );
            painter.line_segment(
                [Pos2::new(rect.min.x, y - offset), Pos2::new(rect.max.x, y - offset)],
                Stroke::new(1.0, color),
            );
        }
    }
}

// Add particle system
struct Particle {
    pos: Pos2,
    vel: Vec2,
    color: Color32,
    life: f32,
    size: f32,
    rotation: f32,
    rotation_speed: f32,
}

struct ParticleSystem {
    particles: Vec<Particle>,
    last_spawn: Instant,
    rng: rand::rngs::ThreadRng,
}

impl ParticleSystem {
    fn new() -> Self {
        Self {
            particles: Vec::with_capacity(1000),
            last_spawn: Instant::now(),
            rng: rand::thread_rng(),
        }
    }

    fn draw_shuriken(ui: &mut egui::Ui, particle: &Particle) {
        let painter = ui.painter();
        let points: Vec<Pos2> = (0..4).map(|i| {
            let angle = particle.rotation + (i as f32 * PI / 2.0);
            let x = particle.pos.x + particle.size * angle.cos();
            let y = particle.pos.y + particle.size * angle.sin();
            Pos2::new(x, y)
        }).collect();

        let alpha = (particle.life * 255.0) as u8;

        // Base shape
        painter.add(egui::Shape::convex_polygon(
            points.clone(),
            Color32::from_rgba_unmultiplied(200, 200, 220, alpha),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(180, 180, 200, alpha)),
        ));

        // Shine effect
        let shine_points: Vec<Pos2> = points.iter().map(|p| {
            let dx = p.x - particle.pos.x;
            let dy = p.y - particle.pos.y;
            let shine_factor = 0.8;
            Pos2::new(
                particle.pos.x + dx * shine_factor,
                particle.pos.y + dy * shine_factor,
            )
        }).collect();

        painter.add(egui::Shape::convex_polygon(
            shine_points,
            Color32::from_rgba_unmultiplied(255, 255, 255, (alpha as f32 * 0.5) as u8),
            Stroke::NONE,
        ));

        // Center dot
        painter.circle_filled(
            particle.pos,
            particle.size * 0.2,
            Color32::from_rgba_unmultiplied(180, 180, 200, alpha),
        );
    }

    fn update(&mut self, rect: Rect, ui: &mut egui::Ui, _palette: &ColorPalette, _pulse: f32) {
        // Get mouse position for emission point
        let emission_point = ui.input(|i| i.pointer.hover_pos())
            .unwrap_or(Pos2::new(rect.center().x, rect.center().y));

        // Spawn new particles
        if self.last_spawn.elapsed() > Duration::from_millis(20) {
            self.last_spawn = Instant::now();
            
            // Spawn particles from emission point
            for _ in 0..3 {
                let angle = self.rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                let speed = self.rng.gen_range(100.0..300.0);
                let size = self.rng.gen_range(5.0..15.0);
                
                self.particles.push(Particle {
                    pos: emission_point,
                    vel: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                    color: Color32::from_rgb(200, 200, 220),
                    life: 1.0,
                    size,
                    rotation: self.rng.gen_range(0.0..std::f32::consts::PI * 2.0),
                    rotation_speed: self.rng.gen_range(-5.0..5.0),
                });
            }
        }

        // Update particles
        let dt = 0.016; // Assume 60fps
        self.particles.retain_mut(|p| {
            p.pos += p.vel * dt;
            p.life -= dt * 0.5; // Slower fade out
            p.rotation += p.rotation_speed * dt;
            p.life > 0.0
        });

        // Draw particles
        for particle in &self.particles {
            Self::draw_shuriken(ui, particle);
        }

        // Draw the title overlay
        let title = "CYBER NINJA";
        let font_size = 72.0;
        let font_id = FontId::monospace(font_size);
        
        // Draw glowing red text layers
        for i in (0..=5).rev() {
            let alpha = (255 - i * 40) as u8;
            let color = if i == 0 {
                Color32::from_rgb(255, 50, 50) // Bright red for main text
            } else {
                Color32::from_rgba_unmultiplied(255, 0, 0, alpha) // Darker red for glow
            };

            ui.painter().text(
                rect.center() + Vec2::new(0.0, -font_size/2.0),
                Align2::CENTER_CENTER,
                title,
                font_id.clone(),
                color,
            );
        }
    }
}

// Add Shuriken animation
struct Shuriken {
    rotation: f32,
    size: f32,
    position: Pos2,
    speed: f32,
}

impl Shuriken {
    fn new(position: Pos2, size: f32) -> Self {
        Self {
            rotation: 0.0,
            size,
            position,
            speed: 3.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.rotation += self.speed * dt;
    }

    fn draw(&self, ui: &mut egui::Ui, palette: &ColorPalette, pulse: f32) {
        let painter = ui.painter();
        let points: Vec<Pos2> = (0..4).map(|i| {
            let angle = self.rotation + (i as f32 * PI / 2.0);
            let x = self.position.x + self.size * angle.cos();
            let y = self.position.y + self.size * angle.sin();
            Pos2::new(x, y)
        }).collect();

        // Draw the shuriken blades with glow effect
        for i in (0..=4).rev() {
            let alpha = (255 - i * 50) as u8;
            let size_mult = 1.0 + (i as f32 * 0.1);
            let glow_points: Vec<Pos2> = points.iter().map(|p| {
                let dx = p.x - self.position.x;
                let dy = p.y - self.position.y;
                Pos2::new(
                    self.position.x + dx * size_mult,
                    self.position.y + dy * size_mult,
                )
            }).collect();

            painter.add(egui::Shape::convex_polygon(
                glow_points.clone(),
                palette.with_alpha(palette.primary, (alpha as f32 * pulse) as u8),
                Stroke::new(1.0, palette.with_alpha(palette.accent1, alpha)),
            ));
        }

        // Draw the center circle with glow
        for i in (0..=3).rev() {
            let alpha = (255 - i * 60) as u8;
            let size = (self.size * 0.2) * (1.0 + i as f32 * 0.2);
            painter.circle_filled(
                self.position,
                size,
                palette.with_alpha(palette.secondary, (alpha as f32 * pulse) as u8),
            );
        }
    }
}

// Enhance CyberNinjaApp with new effects
struct CyberNinjaApp {
    system: System,
    start_time: Instant,
    digital_rain: DigitalRain,
    terminal: HackerTerminal,
    neon_pulse: f32,
    show_terminal: bool,
    hex_grid: HexGrid,
    scan_line: ScanLine,
    alert_glitch: Option<(String, Instant)>,
    palette: ColorPalette,
    logo_offset: f32,
    logo_scale: f32,
    particles: ParticleSystem,
    tts: Option<TTSManager>,
    last_cpu_warning: Option<Instant>,
    last_memory_warning: Option<Instant>,
    last_status_update: Option<Instant>,
    shuriken: Shuriken,
    show_settings: bool,
    settings_volume: f32,
    settings_cpu_threshold: f32,
    settings_update_interval: f32,
}

impl CyberNinjaApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        println!("Initializing CyberNinjaApp...");
        dotenv().ok();
        
        // Initialize system with CPU monitoring enabled
        let mut sys = System::new_all();
        sys.refresh_cpu(); // Initial CPU refresh
        
        let api_key = match env::var("OPENAI_API_KEY") {
            Ok(key) => {
                println!("Found OpenAI API key");
                key
            },
            Err(e) => {
                println!("Error loading OpenAI API key: {}", e);
                String::new()
            }
        };

        // Sleep briefly to get initial CPU measurements
        std::thread::sleep(Duration::from_millis(500));
        sys.refresh_cpu();

        let tts = if !api_key.is_empty() {
            match TTSManager::new(api_key) {
                Ok(tts) => {
                    println!("TTS system initialized successfully");
                    Some(tts)
                },
                Err(e) => {
                    println!("Failed to initialize TTS system: {}", e);
                    None
                }
            }
        } else {
            println!("No API key provided, TTS will be disabled");
            None
        };

        let mut app = Self {
            system: sys,
            start_time: Instant::now(),
            digital_rain: DigitalRain::new(),
            terminal: HackerTerminal::new(),
            neon_pulse: 0.0,
            show_terminal: false,
            hex_grid: HexGrid::new(),
            scan_line: ScanLine::new(),
            alert_glitch: None,
            palette: ColorPalette::cyberpunk(),
            logo_offset: 0.0,
            logo_scale: 1.0,
            particles: ParticleSystem::new(),
            tts,
            last_cpu_warning: None,
            last_memory_warning: None,
            last_status_update: None,
            shuriken: Shuriken::new(Pos2::new(0.0, 0.0), 20.0),
            show_settings: false,
            settings_volume: 0.8,
            settings_cpu_threshold: 80.0,
            settings_update_interval: 300.0,
        };

        // Initial status announcement
        println!("Attempting initial TTS announcement...");
        if let Some(tts) = &app.tts {
            if let Err(e) = tts.speak("Cyber Ninja monitoring system initialized. All systems operational.") {
                println!("Error during initial TTS announcement: {}", e);
            }
        }

        app
    }

    fn draw_enhanced_progress_bar(&self, ui: &mut egui::Ui, progress: f32, text: String, color: Color32) {
        let rect = ui.available_rect_before_wrap();
        let bar_height = 25.0; // Slightly reduced height
        let bar_width = rect.width().min(300.0); // Fixed width
        let bar_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + (rect.width() - bar_width) * 0.5, rect.min.y),
            Vec2::new(bar_width, bar_height),
        );

        // Background with more opacity
        ui.painter().rect_filled(
            bar_rect,
            5.0,
            Color32::from_rgba_unmultiplied(10, 10, 20, 230),
        );

        // Progress
        let progress_rect = Rect::from_min_size(
            bar_rect.min,
            Vec2::new(bar_rect.width() * progress, bar_height),
        );
        ui.painter().rect_filled(
            progress_rect,
            5.0,
            color,
        );

        // Scanlines effect with reduced opacity
        for y in (0..(bar_height as i32)).step_by(2) {
            let line_rect = Rect::from_min_size(
                Pos2::new(bar_rect.min.x, bar_rect.min.y + y as f32),
                Vec2::new(bar_rect.width(), 1.0),
            );
            ui.painter().rect_filled(
                line_rect,
                0.0,
                Color32::from_rgba_unmultiplied(0, 0, 0, 30),
            );
        }

        // Enhanced glow effect
        for i in 1..=3 {
            let glow_color = self.palette.pulse_color(color, self.neon_pulse, (100 / i) as u8);
            ui.painter().rect_stroke(
                bar_rect.expand(i as f32 * 1.0),
                5.0,
                Stroke::new(1.0, glow_color),
            );
        }

        // Text with shadow
        let text_pos = Pos2::new(
            bar_rect.center().x,
            bar_rect.center().y,
        );
        ui.painter().text(
            text_pos + Vec2::new(1.0, 1.0),
            Align2::CENTER_CENTER,
            text.clone(),
            FontId::monospace(14.0),
            Color32::from_rgba_unmultiplied(0, 0, 0, 180),
        );
        ui.painter().text(
            text_pos,
            Align2::CENTER_CENTER,
            text,
            FontId::monospace(14.0),
            Color32::WHITE,
        );
    }

    fn draw_info_panel(&self, ui: &mut egui::Ui, title: &str, value: &str, color: Color32) {
        let rect = ui.available_rect_before_wrap();
        let panel_height = 50.0;
        let panel_width = rect.width().min(300.0);
        let panel_rect = Rect::from_min_size(
            Pos2::new(rect.min.x + (rect.width() - panel_width) * 0.5, rect.min.y),
            Vec2::new(panel_width, panel_height),
        );

        // Background
        ui.painter().rect_filled(
            panel_rect,
            5.0,
            Color32::from_rgba_unmultiplied(10, 10, 20, 230),
        );

        // Border glow
        for i in 1..=2 {
            let glow_color = self.palette.pulse_color(color, self.neon_pulse, (80 / i) as u8);
            ui.painter().rect_stroke(
                panel_rect.expand(i as f32 * 1.0),
                5.0,
                Stroke::new(1.0, glow_color),
            );
        }

        // Title
        ui.painter().text(
            Pos2::new(panel_rect.center().x, panel_rect.min.y + 15.0),
            Align2::CENTER_CENTER,
            title,
            FontId::monospace(12.0),
            Color32::from_rgba_unmultiplied(200, 200, 200, 255),
        );

        // Value
        ui.painter().text(
            Pos2::new(panel_rect.center().x, panel_rect.max.y - 15.0),
            Align2::CENTER_CENTER,
            value,
            FontId::monospace(16.0),
            color,
        );
    }

    fn check_system_warnings(&mut self) {
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        let memory_used = self.system.used_memory() as f32 / self.system.total_memory() as f32;
        
        // CPU warning (every 30 seconds)
        if cpu_usage > self.settings_cpu_threshold {
            if self.last_cpu_warning
                .map_or(true, |last| last.elapsed().as_secs() > 30)
            {
                self.last_cpu_warning = Some(Instant::now());
                self.alert_glitch = Some(("HIGH CPU USAGE DETECTED".to_string(), Instant::now()));
                if let Some(tts) = &mut self.tts {
                    println!("Attempting to announce CPU warning...");
                    tts.set_volume(self.settings_volume);
                    if let Err(e) = tts.speak(&format!("Warning! CPU usage at {:.0} percent!", cpu_usage)) {
                        println!("Error announcing CPU warning: {}", e);
                    }
                }
            }
        }

        // Memory warning (every 30 seconds)
        if memory_used > 0.9 {
            if self.last_memory_warning
                .map_or(true, |last| last.elapsed().as_secs() > 30)
            {
                self.last_memory_warning = Some(Instant::now());
                self.alert_glitch = Some(("HIGH MEMORY USAGE DETECTED".to_string(), Instant::now()));
                if let Some(tts) = &mut self.tts {
                    println!("Attempting to announce memory warning...");
                    tts.set_volume(self.settings_volume);
                    if let Err(e) = tts.speak(&format!("Warning! Memory usage at {:.0} percent!", memory_used * 100.0)) {
                        println!("Error announcing memory warning: {}", e);
                    }
                }
            }
        }

        // Regular status updates
        if self.last_status_update
            .map_or(true, |last| last.elapsed().as_secs() > self.settings_update_interval as u64)
        {
            self.last_status_update = Some(Instant::now());
            if let Some(tts) = &mut self.tts {
                println!("Attempting to announce status update...");
                tts.set_volume(self.settings_volume);
                let status = format!(
                    "Status report: CPU at {:.0} percent, Memory at {:.0} percent",
                    cpu_usage,
                    memory_used * 100.0
                );
                if let Err(e) = tts.speak(&status) {
                    println!("Error announcing status update: {}", e);
                }
            }
        }
    }
}

impl eframe::App for CyberNinjaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Refresh system stats with proper timing
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.system.refresh_processes();
        
        self.check_system_warnings();
        let elapsed = self.start_time.elapsed().as_secs_f32();
        self.neon_pulse = (elapsed * 2.0).sin() * 0.5 + 0.5;
        
        // Update shuriken
        self.shuriken.update(0.016);

        // Animate logo
        self.logo_offset = (elapsed * 3.0).sin() * 5.0;
        self.logo_scale = 1.0 + (elapsed * 2.0).sin() * 0.05;

        // Set dark theme
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgb(13, 17, 23);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            
            // Clean, dark background
            ui.painter().rect_filled(
                rect,
                0.0,
                Color32::from_rgb(13, 17, 23),
            );

            // Top bar with minimal design
            let top_bar_height = 50.0;
            let top_bar_rect = Rect::from_min_size(
                rect.min,
                Vec2::new(rect.width(), top_bar_height),
            );

            // Subtle top bar separator
            ui.painter().line_segment(
                [
                    Pos2::new(rect.min.x, top_bar_rect.max.y),
                    Pos2::new(rect.max.x, top_bar_rect.max.y),
                ],
                Stroke::new(1.0, Color32::from_rgb(30, 34, 40)),
            );

            // Title with clean typography
            ui.painter().text(
                top_bar_rect.center(),
                Align2::CENTER_CENTER,
                "CYBER NINJA",
                FontId::monospace(24.0),
                Color32::from_rgb(200, 200, 200),
            );

            // Settings button
            let settings_btn_rect = Rect::from_min_size(
                Pos2::new(rect.max.x - 100.0, top_bar_rect.min.y + 10.0),
                Vec2::new(80.0, 30.0),
            );

            if ui.put(
                settings_btn_rect,
                egui::Button::new(
                    RichText::new(if self.show_settings { "CLOSE" } else { "SETTINGS" })
                        .color(Color32::from_rgb(0, 255, 170))
                        .monospace()
                )
            ).clicked() {
                self.show_settings = !self.show_settings;
            }

            // Main content area with clean grid
            let content_rect = Rect::from_min_max(
                Pos2::new(rect.min.x, top_bar_rect.max.y + 20.0),
                rect.max,
            );

            // Grid layout for monitoring panels
            let grid_margin = 15.0;
            let panel_width = (content_rect.width() - grid_margin * 3.0) / 2.0;
            let panel_height = (content_rect.height() - grid_margin * 3.0) / 2.0;

            // Function to create a clean panel background with ambient glow
            let draw_panel = |ui: &mut egui::Ui, rect: Rect, glow_color: Color32| {
                // Ambient background glow
                for i in (0..=4).rev() {
                    let alpha = (20 - i * 4) as u8;
                    let offset = i as f32 * 2.0;
                    ui.painter().rect_filled(
                        rect.expand(offset),
                        8.0,
                        Color32::from_rgba_unmultiplied(
                            glow_color.r(),
                            glow_color.g(),
                            glow_color.b(),
                            alpha,
                        ),
                    );
                }

                // Panel background
                ui.painter().rect_filled(
                    rect,
                    6.0,
                    Color32::from_rgb(18, 22, 28),
                );
                ui.painter().rect_stroke(
                    rect,
                    6.0,
                    Stroke::new(1.0, Color32::from_rgb(30, 34, 40)),
                );
            };

            // Calculate average CPU usage across all cores
            let cpu_usage = self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / 
                           self.system.cpus().len() as f32;
            let cpu_usage = cpu_usage / 100.0; // Normalize to 0.0-1.0 range

            // CPU Panel with green glow
            let cpu_rect = Rect::from_min_size(
                content_rect.min,
                Vec2::new(panel_width, panel_height),
            );
            draw_panel(ui, cpu_rect, Color32::from_rgb(0, 255, 170));
            ui.allocate_ui_at_rect(cpu_rect.shrink(20.0), |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("CPU USAGE").color(Color32::from_rgb(150, 150, 150)).monospace());
                    ui.add_space(10.0);
                    let progress_rect = ui.available_rect_before_wrap();
                    let progress_height = 8.0;
                    let progress_pos = progress_rect.min.y + (progress_rect.height() - progress_height) / 2.0;
                    
                    // Progress bar background
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            Pos2::new(progress_rect.min.x, progress_pos),
                            Vec2::new(progress_rect.width(), progress_height),
                        ),
                        4.0,
                        Color32::from_rgb(30, 34, 40),
                    );

                    // Progress bar fill with clamped value
                    let clamped_usage = cpu_usage.clamp(0.0, 1.0);
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            Pos2::new(progress_rect.min.x, progress_pos),
                            Vec2::new(progress_rect.width() * clamped_usage, progress_height),
                        ),
                        4.0,
                        Color32::from_rgb(0, 255, 170),
                    );

                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("{:.1}%", cpu_usage * 100.0))
                        .size(32.0)
                        .color(Color32::from_rgb(0, 255, 170))
                        .monospace());

                    // Add core count info
                    ui.add_space(5.0);
                    ui.label(RichText::new(format!("{} Cores", self.system.cpus().len()))
                        .size(14.0)
                        .color(Color32::from_rgb(150, 150, 150))
                        .monospace());
                });
            });

            // Memory Panel with pink glow
            let mem_rect = Rect::from_min_size(
                Pos2::new(cpu_rect.max.x + grid_margin, content_rect.min.y),
                Vec2::new(panel_width, panel_height),
            );
            draw_panel(ui, mem_rect, Color32::from_rgb(255, 70, 130));
            let total_mem = self.system.total_memory();
            let used_mem = self.system.used_memory();
            let mem_percent = used_mem as f32 / total_mem as f32;
            ui.allocate_ui_at_rect(mem_rect.shrink(20.0), |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("MEMORY USAGE").color(Color32::from_rgb(150, 150, 150)).monospace());
                    ui.add_space(10.0);
                    let progress_rect = ui.available_rect_before_wrap();
                    let progress_height = 8.0;
                    let progress_pos = progress_rect.min.y + (progress_rect.height() - progress_height) / 2.0;
                    
                    // Progress bar background
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            Pos2::new(progress_rect.min.x, progress_pos),
                            Vec2::new(progress_rect.width(), progress_height),
                        ),
                        4.0,
                        Color32::from_rgb(30, 34, 40),
                    );

                    // Progress bar fill
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            Pos2::new(progress_rect.min.x, progress_pos),
                            Vec2::new(progress_rect.width() * mem_percent, progress_height),
                        ),
                        4.0,
                        Color32::from_rgb(255, 70, 130),
                    );

                    ui.add_space(10.0);
                    ui.label(RichText::new(format!("{:.1}%", mem_percent * 100.0))
                        .size(32.0)
                        .color(Color32::from_rgb(255, 70, 130))
                        .monospace());
                });
            });

            // Network Panel with cyan glow
            let net_rect = Rect::from_min_size(
                Pos2::new(content_rect.min.x, cpu_rect.max.y + grid_margin),
                Vec2::new(panel_width, panel_height),
            );
            draw_panel(ui, net_rect, Color32::from_rgb(70, 200, 255));
            let networks = self.system.networks();
            let net_in: u64 = networks.iter().map(|(_,n)| n.total_received()).sum();
            let net_out: u64 = networks.iter().map(|(_,n)| n.total_transmitted()).sum();
            ui.allocate_ui_at_rect(net_rect.shrink(20.0), |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("NETWORK").color(Color32::from_rgb(150, 150, 150)).monospace());
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new("IN")
                                .color(Color32::from_rgb(150, 150, 150))
                                .monospace());
                            ui.label(RichText::new(format!("{} KB/s", net_in / 1024))
                                .size(24.0)
                                .color(Color32::from_rgb(70, 200, 255))
                                .monospace());
                        });
                        
                        ui.add_space(40.0);
                        
                        ui.vertical(|ui| {
                            ui.label(RichText::new("OUT")
                                .color(Color32::from_rgb(150, 150, 150))
                                .monospace());
                            ui.label(RichText::new(format!("{} KB/s", net_out / 1024))
                                .size(24.0)
                                .color(Color32::from_rgb(70, 200, 255))
                                .monospace());
                        });
                    });
                });
            });

            // System Info / Settings Panel with purple glow
            let info_rect = Rect::from_min_size(
                Pos2::new(net_rect.max.x + grid_margin, mem_rect.max.y + grid_margin),
                Vec2::new(panel_width, panel_height),
            );
            draw_panel(ui, info_rect, Color32::from_rgb(170, 70, 255));
            ui.allocate_ui_at_rect(info_rect.shrink(20.0), |ui| {
                if self.show_settings {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("SETTINGS")
                            .color(Color32::from_rgb(150, 150, 150))
                            .monospace());
                        ui.add_space(20.0);

                        ui.label(RichText::new("TTS Volume").monospace());
                        if ui.add(egui::Slider::new(&mut self.settings_volume, 0.0..=1.0)
                            .text(""))
                            .changed() {
                            if let Some(tts) = &mut self.tts {
                                tts.set_volume(self.settings_volume);
                            }
                        }

                        ui.add_space(10.0);
                        ui.label(RichText::new("CPU Warning Threshold").monospace());
                        ui.add(egui::Slider::new(&mut self.settings_cpu_threshold, 50.0..=100.0)
                            .text("%"));

                        ui.add_space(10.0);
                        ui.label(RichText::new("Update Interval").monospace());
                        ui.add(egui::Slider::new(&mut self.settings_update_interval, 60.0..=600.0)
                            .text("s"));

                        ui.add_space(20.0);
                        if ui.button(RichText::new("Test TTS")
                            .color(Color32::from_rgb(0, 255, 170))
                            .monospace())
                            .clicked() {
                            if let Some(tts) = &mut self.tts {
                                tts.set_volume(self.settings_volume);
                                if let Err(e) = tts.speak("Testing text to speech system.") {
                                    println!("Error testing TTS: {}", e);
                                }
                            }
                        }
                    });
                } else {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("SYSTEM INFO")
                            .color(Color32::from_rgb(150, 150, 150))
                            .monospace());
                        ui.add_space(20.0);
                        
                        ui.label(RichText::new("Uptime")
                            .color(Color32::from_rgb(150, 150, 150))
                            .monospace());
                        ui.label(RichText::new(format!("{:.1}s", self.start_time.elapsed().as_secs_f32()))
                            .size(24.0)
                            .color(Color32::from_rgb(255, 255, 255))
                            .monospace());
                            
                        ui.add_space(10.0);
                        ui.label(RichText::new("Processes")
                            .color(Color32::from_rgb(150, 150, 150))
                            .monospace());
                        ui.label(RichText::new(format!("{}", self.system.processes().len()))
                            .size(24.0)
                            .color(Color32::from_rgb(255, 255, 255))
                            .monospace());
                    });
                }
            });

            // Draw warning overlay if active
            if let Some((text, start_time)) = &self.alert_glitch {
                let elapsed = start_time.elapsed().as_secs_f32();
                if elapsed < 3.0 {
                    let alpha = ((1.0 - elapsed / 3.0) * 255.0) as u8;
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(13, 17, 23, 200),
                    );
                    
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        text,
                        FontId::monospace(32.0),
                        Color32::from_rgba_unmultiplied(255, 70, 70, alpha),
                    );
                } else {
                    self.alert_glitch = None;
                }
            }
        });

        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_decorations(true)
            .with_transparent(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "CyberNinja System Monitor",
        options,
        Box::new(|cc| Box::new(CyberNinjaApp::new(cc))),
    )
} 