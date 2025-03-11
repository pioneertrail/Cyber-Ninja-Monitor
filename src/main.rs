use eframe::NativeOptions;
use eframe::egui::{
    self, Color32, Pos2, RichText, Rect, vec2, TextureHandle, Vec2, Align2, FontId, Stroke, pos2,
    Rounding, ViewportBuilder,
};
use crate::message_system::{SystemData, generate_message, MessagePart, PersonalitySettings};
use crate::theme::{SCAN_LINE_SPEED, HOLOGRAM_FLICKER_SPEED, BLOOM_INTENSITY, FOG_DENSITY, HOLOGRAM_OPACITY, CyberTheme};
use crate::system_monitor::SystemMonitor;
use crate::ai_personality::AIPersonality;
use crate::tts::TTSManager;
use tokio::runtime::Runtime;
use egui::Context;
use std::time::{Instant, Duration};
use sysinfo::{System, SystemExt, CpuExt};
use dotenv::dotenv;
use rand::Rng;
use crate::particles::ParticleSystem;
use usvg::TreeParsing;

mod tts;
mod system_monitor;
mod theme;
mod ai_personality;
mod particles;
mod message_system;

const CPU_ICON: &[u8] = include_bytes!("../assets/cpu_icon.svg");
const MEMORY_ICON: &[u8] = include_bytes!("../assets/memory_icon.svg");
const DISK_ICON: &[u8] = include_bytes!("../assets/disk_icon.svg");

// Network statistics tracking
struct NetworkStats {
    last_update: Instant,
    bytes_received: u64,
    bytes_sent: u64,
    receive_rate: f64,
    send_rate: f64,
}

impl NetworkStats {
    fn new() -> Self {
        Self {
            last_update: Instant::now(),
            bytes_received: 0,
            bytes_sent: 0,
            receive_rate: 0.0,
            send_rate: 0.0,
        }
    }

    fn update(&mut self, new_received: u64, new_sent: u64) {
        let elapsed = self.last_update.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.receive_rate = (new_received as f64 - self.bytes_received as f64) / elapsed;
            self.send_rate = (new_sent as f64 - self.bytes_sent as f64) / elapsed;
        }
        self.bytes_received = new_received;
        self.bytes_sent = new_sent;
        self.last_update = Instant::now();
    }
}

// Main application state
pub struct CyberNinjaApp {
    system: System,
    start_time: Instant,
    neon_pulse: f32,
    tts: Option<TTSManager>,
    last_cpu_warning: Option<Instant>,
    last_memory_warning: Option<Instant>,
    last_status_update: Instant,
    show_settings: bool,
    settings_volume: f32,
    settings_cpu_threshold: f32,
    settings_update_interval: f32,
    network_stats: NetworkStats,
    cpu_icon: Option<TextureHandle>,
    memory_icon: Option<TextureHandle>,
    disk_icon: Option<TextureHandle>,
    alert_glitch: Option<Instant>,
    monitor: SystemMonitor,
    personality: AIPersonality,
    editing_catchphrase: String,
    theme: theme::CyberTheme,
    shurikens: Vec<theme::Shuriken>,
    last_frame_time: Instant,
    warp_effect_intensity: f32,
    particle_system: ParticleSystem,
    hologram_phase: f32,
    runtime: Runtime,
}

impl CyberNinjaApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        println!("Initializing CyberNinjaApp");
        
        // Set up custom fonts if needed
        egui_extras::install_image_loaders(&cc.egui_ctx);
        
        // Set up dark visuals by default
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let theme = theme::CyberTheme::default();
        
        let mut app = Self {
            system: System::new_all(),
            start_time: Instant::now(),
            neon_pulse: 0.0,
            tts: None,
            last_cpu_warning: None,
            last_memory_warning: None,
            last_status_update: Instant::now(),
            show_settings: false,
            settings_volume: 0.8,
            settings_cpu_threshold: 80.0,
            settings_update_interval: 300.0,
            network_stats: NetworkStats::new(),
            cpu_icon: None,
            memory_icon: None,
            disk_icon: None,
            alert_glitch: None,
            monitor: SystemMonitor::new(),
            personality: AIPersonality::default(),
            editing_catchphrase: String::new(),
            theme: theme.clone(),
            shurikens: Vec::new(),
            last_frame_time: Instant::now(),
            warp_effect_intensity: 0.0,
            particle_system: ParticleSystem::new(theme),
            hologram_phase: 0.0,
            runtime,
        };
        
        // Print current working directory and environment variables for debugging
        println!("Current working directory: {:?}", std::env::current_dir().unwrap_or_default());
        println!("OPENAI_API_KEY exists: {:?}", std::env::var("OPENAI_API_KEY").is_ok());
        
        println!("Initializing TTS system...");
        if let Some(mut tts) = TTSManager::new() {
            println!("TTS system initialized successfully");
            let startup_message = vec![
                MessagePart::Static("CyberNinja Monitor initialized.".to_string())
            ];
            let settings = app.personality.to_settings();
            println!("Attempting to speak startup message...");
            app.runtime.block_on(async {
                if let Err(e) = tts.speak(startup_message, &settings).await {
                    eprintln!("Failed to speak startup message: {}", e);
                }
            });
            app.tts = Some(tts);
        } else {
            eprintln!("Failed to initialize TTS system");
        }
        
        // Load icons
        let ctx = &cc.egui_ctx;
        app.cpu_icon = Some(load_svg_icon(ctx, CPU_ICON));
        app.memory_icon = Some(load_svg_icon(ctx, MEMORY_ICON));
        app.disk_icon = Some(load_svg_icon(ctx, DISK_ICON));

        // Initialize shurikens
        app.shurikens.push(theme::Shuriken::new(
            Pos2::new(50.0, 50.0),
            app.theme.neon_primary
        ));
        app.shurikens.push(theme::Shuriken::new(
            Pos2::new(974.0, 50.0),
            app.theme.neon_secondary
        ));
        
        println!("CyberNinjaApp initialization complete");
        app
    }

    fn generate_message(&self, base_message: &str) -> String {
        let mut message = base_message.to_string();
        let mut prefix = String::new();
        let mut suffix = String::new();

        // Convert numerical values to qualitative descriptions
        message = message
            .replace(|c: char| c.is_numeric(), "")
            .replace("%", "")
            .replace("MB/s", "")
            .replace("GB", "")
            .replace("  ", " ");

        // Replace numerical descriptions with qualitative ones
        message = message
            .replace("at ", "is ")
            .replace("using about", "at")
            .replace("running at", "running");
        
        // Add drunk effects
        if self.personality.drunk_level > 0.3 {
            message = message.replace("s", "sh")
                           .replace("r", "rr")
                           .replace(".", "...")
                           .replace("!", "!!");
        }
        
        // Add catchphrases based on sass level
        if self.personality.sass_level > 0.5 && !self.personality.catchphrases.is_empty() {
            if rand::random::<f32>() < self.personality.sass_level {
                let idx = rand::random::<usize>() % self.personality.catchphrases.len();
                suffix.push_str(&format!(" {}", self.personality.catchphrases[idx]));
            }
        }
        
        // Add Grand Pappi references
        if self.personality.grand_pappi_references > 0.3 && rand::random::<f32>() < self.personality.grand_pappi_references {
            let pappi_quotes = [
                "Grand Pappi would be proud!",
                "Just like Grand Pappi's old quantum bike...",
                "Grand Pappi always said this was the way.",
                "Reminds me of Grand Pappi's workshop...",
            ];
            let quote = pappi_quotes[rand::random::<usize>() % pappi_quotes.len()];
            suffix.push_str(&format!(" {}", quote));
        }

        // Add enthusiasm effects
        if self.personality.enthusiasm > 0.7 {
            message = message.replace(".", "!");
            prefix.push_str("🎉 ");
            suffix.push_str(" 🚀");
        }

        // Add anxiety effects
        if self.personality.anxiety_level > 0.7 {
            message = message.replace(".", "...");
            prefix.push_str("*nervously* ");
            suffix.push_str(" *fidgets*");
        }
        
        format!("{}{}{}", prefix, message, suffix)
    }

    fn check_system_warnings(&mut self) {
        if let Some(tts) = &mut self.tts {
            let data = SystemData {
                cpu_usage: self.system.global_cpu_info().cpu_usage(),
                memory_used: self.system.used_memory(),
                memory_total: self.system.total_memory(),
                disk_usage: 0.0, // We'll update this when needed
                network_rx: 0,
                network_tx: 0,
            };

            // CPU warning (every 30 seconds)
            if data.cpu_usage > self.settings_cpu_threshold {
                if self.last_cpu_warning
                    .map_or(true, |last| last.elapsed().as_secs() > 30)
                {
                    self.last_cpu_warning = Some(Instant::now());
                    self.alert_glitch = Some(Instant::now());
                    
                    let parts = generate_message(&data);
                    
                    self.runtime.block_on(async {
                        if let Err(e) = tts.speak(parts, &self.personality.to_settings()).await {
                            eprintln!("Failed to speak CPU warning: {}", e);
                        }
                    });
                }
            }

            // Memory warning (every 30 seconds)
            let memory_used_pct = data.memory_used as f32 / data.memory_total as f32;
            if memory_used_pct > 0.9 {
                if self.last_memory_warning
                    .map_or(true, |last| last.elapsed().as_secs() > 30)
                {
                    self.last_memory_warning = Some(Instant::now());
                    self.alert_glitch = Some(Instant::now());
                    
                    let parts = generate_message(&data);
                    
                    self.runtime.block_on(async {
                        if let Err(e) = tts.speak(parts, &self.personality.to_settings()).await {
                            eprintln!("Failed to speak memory warning: {}", e);
                        }
                    });
                }
            }

            // Regular status updates
            if self.last_status_update.elapsed() >= Duration::from_secs(self.settings_update_interval as u64) {
                self.last_status_update = Instant::now();
                
                let parts = generate_message(&data);
                
                self.runtime.block_on(async {
                    if let Err(e) = tts.speak(parts, &self.personality.to_settings()).await {
                        eprintln!("Failed to speak status update: {}", e);
                    }
                });
            }
        }
    }

    fn show_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("AI Personality Settings")
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                // Voice Settings Section
                ui.heading("Voice Settings");
                egui::Frame::none()
                    .fill(self.theme.background_light)
                    .rounding(Rounding::same(4.0))
                    .show(ui, |ui| {
                        // Voice type dropdown
                        ui.horizontal(|ui| {
                            ui.label("Voice Type:");
                            egui::ComboBox::from_id_source("voice_type")
                                .selected_text(&self.personality.voice_type)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut self.personality.voice_type, "alloy".to_string(), "Alloy");
                                    ui.selectable_value(&mut self.personality.voice_type, "echo".to_string(), "Echo");
                                    ui.selectable_value(&mut self.personality.voice_type, "fable".to_string(), "Fable");
                                    ui.selectable_value(&mut self.personality.voice_type, "nova".to_string(), "Nova");
                                    ui.selectable_value(&mut self.personality.voice_type, "onyx".to_string(), "Onyx");
                                    ui.selectable_value(&mut self.personality.voice_type, "shimmer".to_string(), "Shimmer");
                                });
                            
                            if ui.button("Apply Voice").clicked() && self.tts.is_some() {
                                if let Some(tts) = &mut self.tts {
                                    tts.set_voice_type(self.personality.voice_type.clone());
                                }
                            }
                        });

                        // Audio Controls
                        ui.add_space(8.0);
                        ui.heading("Audio Controls");
                        
                        // Audio test and mute buttons
                        ui.horizontal(|ui| {
                            if ui.button("🔊 Test Audio").clicked() {
                                if let Some(tts) = &mut self.tts {
                                    // Create a test message that will demonstrate personality traits
                                    let test_message = if self.personality.drunk_level > 0.5 {
                                        "Hey there, let's test these awesome settings!"
                                    } else if self.personality.sass_level > 0.5 {
                                        "Oh great, another test? Fine, let's do this."
                                    } else if self.personality.anxiety_level > 0.5 {
                                        "Um... testing the voice settings... if that's okay?"
                                    } else if self.personality.enthusiasm > 0.5 {
                                        "WOW! Time to test these AMAZING voice settings!"
                                    } else {
                                        "Testing personality and voice settings."
                                    };

                                    let message = vec![MessagePart::Static(test_message.to_string())];
                                    let settings = self.personality.to_settings();
                                    
                                    // Update TTS settings before speaking
                                    tts.set_voice_type(self.personality.voice_type.clone());
                                    tts.set_volume(self.personality.volume);
                                    tts.set_speech_rate(self.personality.speech_rate);
                                    
                                    self.runtime.block_on(async {
                                        if let Err(e) = tts.speak(message, &settings).await {
                                            eprintln!("Audio test error: {}", e);
                                        }
                                    });
                                }
                            }
                            
                            if ui.button(if self.personality.audio_enabled { "🔊 Mute" } else { "🔈 Unmute" }).clicked() {
                                self.personality.audio_enabled = !self.personality.audio_enabled;
                                if let Some(tts) = &mut self.tts {
                                    tts.set_audio_enabled(self.personality.audio_enabled);
                                    let message = vec![MessagePart::Static("Audio toggled".to_string())];
                                    let settings = self.personality.to_settings();
                                    self.runtime.block_on(async {
                                        if let Err(e) = tts.speak(message, &settings).await {
                                            eprintln!("Failed to speak: {}", e);
                                        }
                                    });
                                }
                            }
                        });

                        ui.add_space(4.0);
                        if ui.add(egui::Slider::new(&mut self.personality.volume, 0.0..=1.0)
                            .text("Volume")
                            .clamp_to_range(true)).changed() && self.tts.is_some() {
                            if let Some(tts) = &mut self.tts {
                                tts.set_volume(self.personality.volume);
                            }
                        }
                        
                        if ui.add(egui::Slider::new(&mut self.personality.speech_rate, 0.5..=2.0)
                            .text("Speech Rate")
                            .clamp_to_range(true)).changed() && self.tts.is_some() {
                            if let Some(tts) = &mut self.tts {
                                tts.set_speech_rate(self.personality.speech_rate);
                            }
                        }
                    });

                ui.add_space(8.0);

                // Personality Traits Section
                ui.heading("Personality Traits");
                egui::Frame::none()
                    .fill(self.theme.background_light)
                    .rounding(Rounding::same(4.0))
                    .show(ui, |ui| {
                        ui.add(egui::Slider::new(&mut self.personality.drunk_level, 0.0..=1.0)
                            .text("Drunk Level")
                            .clamp_to_range(true));
                        ui.add(egui::Slider::new(&mut self.personality.sass_level, 0.0..=1.0)
                            .text("Sass Level")
                            .clamp_to_range(true));
                        ui.add(egui::Slider::new(&mut self.personality.tech_expertise, 0.0..=1.0)
                            .text("Tech Expertise")
                            .clamp_to_range(true));
                        ui.add(egui::Slider::new(&mut self.personality.grand_pappi_references, 0.0..=1.0)
                            .text("Grand Pappi References")
                            .clamp_to_range(true));
                        ui.add(egui::Slider::new(&mut self.personality.enthusiasm, 0.0..=1.0)
                            .text("Enthusiasm")
                            .clamp_to_range(true));
                        ui.add(egui::Slider::new(&mut self.personality.anxiety_level, 0.0..=1.0)
                            .text("Anxiety Level")
                            .clamp_to_range(true));
                        
                        // Test personality button
                        if ui.button("Test Personality").clicked() {
                            if let Some(tts) = &mut self.tts {
                                let message = vec![MessagePart::Static("Testing personality settings".to_string())];
                                let settings = self.personality.to_settings();
                                self.runtime.block_on(async {
                                    if let Err(e) = tts.speak(message, &settings).await {
                                        eprintln!("Failed to test personality: {}", e);
                                    }
                                });
                            }
                        }
                    });

                ui.add_space(8.0);

                // Catchphrases Section
                ui.heading("Catchphrases");
                egui::Frame::none()
                    .fill(self.theme.background_light)
                    .rounding(Rounding::same(4.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.editing_catchphrase);
                            if ui.button("Add").clicked() && !self.editing_catchphrase.is_empty() {
                                self.personality.catchphrases.push(self.editing_catchphrase.clone());
                                self.editing_catchphrase.clear();
                            }
                        });

                        // Show catchphrases with delete buttons
                        let mut to_remove = None;
                        for (idx, catchphrase) in self.personality.catchphrases.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(catchphrase).color(self.theme.text_dim));
                                if ui.button("🗑").clicked() {
                                    to_remove = Some(idx);
                                }
                            });
                        }

                        // Remove selected catchphrase
                        if let Some(idx) = to_remove {
                            self.personality.catchphrases.remove(idx);
                        }
                    });

                // Exit button at the bottom
                ui.add_space(16.0);
                ui.separator();
                if ui.button("🚪 Exit Application").clicked() {
                    let exit_message = self.personality.get_exit_message();
                    if let Some(tts) = &mut self.tts {
                        let message = vec![MessagePart::Static(exit_message)];
                        let settings = self.personality.to_settings();
                        self.runtime.block_on(async {
                            let _ = tts.speak(message, &settings).await;
                        });
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
    }

    fn draw_grid(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let theme = &self.theme;
        let grid_size = theme::GRID_SIZE;
        
        // Draw horizontal grid lines
        for y in (rect.min.y as i32..rect.max.y as i32).step_by(grid_size as usize) {
            let y = y as f32;
            let alpha = ((y + self.start_time.elapsed().as_secs_f32() * theme::SCAN_LINE_SPEED).sin() * 0.5 + 0.5) * 0.2;
            let color = Color32::from_rgba_premultiplied(
                theme.grid_line.r(),
                theme.grid_line.g(),
                theme.grid_line.b(),
                (theme.grid_line.a() as f32 * alpha) as u8,
            );
            ui.painter().line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, color),
            );
        }
        
        // Draw vertical grid lines
        for x in (rect.min.x as i32..rect.max.x as i32).step_by(grid_size as usize) {
            let x = x as f32;
            let alpha = ((x + self.start_time.elapsed().as_secs_f32() * theme::SCAN_LINE_SPEED).sin() * 0.5 + 0.5) * 0.2;
            let color = Color32::from_rgba_premultiplied(
                theme.grid_line.r(),
                theme.grid_line.g(),
                theme.grid_line.b(),
                (theme.grid_line.a() as f32 * alpha) as u8,
            );
            ui.painter().line_segment(
                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                egui::Stroke::new(1.0, color),
            );
        }
    }

    fn draw_neon_frame(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let theme = &self.theme;
        let pulse = (self.start_time.elapsed().as_secs_f32() * theme::PULSE_SPEED).sin() * 0.5 + 0.5;
        let neon_color = Color32::from_rgba_premultiplied(
            theme.neon_primary.r(),
            theme.neon_primary.g(),
            theme.neon_primary.b(),
            (theme.neon_primary.a() as f32 * pulse) as u8,
        );
        
        // Draw neon border
        ui.painter().rect_stroke(
            rect,
            theme::WINDOW_ROUNDING,
            egui::Stroke::new(theme::BORDER_WIDTH, neon_color),
        );
        
        // Add corner accents
        let corner_size = 10.0;
        let corners = [
            (rect.min, [1.0, 1.0]),
            (pos2(rect.max.x, rect.min.y), [-1.0, 1.0]),
            (rect.max, [-1.0, -1.0]),
            (pos2(rect.min.x, rect.max.y), [1.0, -1.0]),
        ];
        
        for (pos, dir) in corners.iter() {
            ui.painter().line_segment(
                [
                    pos2(pos.x, pos.y),
                    pos2(pos.x + corner_size * dir[0], pos.y),
                ],
                egui::Stroke::new(2.0, neon_color),
            );
            ui.painter().line_segment(
                [
                    pos2(pos.x, pos.y),
                    pos2(pos.x, pos.y + corner_size * dir[1]),
                ],
                egui::Stroke::new(2.0, neon_color),
            );
        }
    }

    fn draw_section_header(&self, ui: &mut egui::Ui, text: &str, color: Color32) {
        ui.horizontal(|ui| {
            ui.heading(RichText::new(text).color(color));
            ui.add_space(4.0);
        });
    }

    fn lerp_color(&self, a: Color32, b: Color32, t: f32) -> Color32 {
        Color32::from_rgba_premultiplied(
            ((1.0 - t) * a.r() as f32 + t * b.r() as f32) as u8,
            ((1.0 - t) * a.g() as f32 + t * b.g() as f32) as u8,
            ((1.0 - t) * a.b() as f32 + t * b.b() as f32) as u8,
            ((1.0 - t) * a.a() as f32 + t * b.a() as f32) as u8,
        )
    }

    fn draw_value_bar(&self, ui: &mut egui::Ui, value: f32, color: Color32) {
        let rect = ui.available_rect_before_wrap();
        let bar_height = 18.0;
        let bar_width = rect.width() * value.clamp(0.0, 1.0);
        
        let painter = ui.painter();
        
        // Background
        painter.rect_filled(
            rect.shrink(1.0),
            4.0,
            self.theme.background_light,
        );
        
        // Progress bar
        if bar_width > 0.0 {
            painter.rect_filled(
                Rect::from_min_size(rect.min, vec2(bar_width, bar_height)),
                4.0,
                color,
            );
        }
        
        ui.allocate_rect(rect, egui::Sense::hover());
    }

    fn draw_shurikens(&mut self, ui: &mut egui::Ui) {
        let now = std::time::Instant::now();
        let dt = (now - self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        for shuriken in &mut self.shurikens {
            shuriken.update(dt);
            let points = shuriken.get_points();
            
            // Draw shuriken body
            ui.painter().add(egui::Shape::convex_polygon(
                points.clone(),
                shuriken.color,
                Stroke::new(1.0, shuriken.color),
            ));
            
            // Draw neon trail
            let trail_color = Color32::from_rgba_premultiplied(
                shuriken.color.r(),
                shuriken.color.g(),
                shuriken.color.b(),
                50,
            );
            
            for i in 0..3 {
                let trail_points: Vec<Pos2> = points.iter().map(|p| {
                    let offset = shuriken.angle - (i as f32 * 0.2);
                    Pos2::new(
                        p.x - offset.cos() * 5.0,
                        p.y - offset.sin() * 5.0,
                    )
                }).collect();
                
                ui.painter().add(egui::Shape::convex_polygon(
                    trail_points,
                    trail_color,
                    Stroke::NONE,
                ));
            }
        }
    }

    fn show_audio_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Updated 1337 mode button with warp drive styling
            let warp_btn = egui::Button::new(
                RichText::new(if self.personality.is_1337_mode {
                    "🚫 Disengage Warp"
                } else {
                    "🚀 Engage Warp"
                })
                .color(if self.personality.is_1337_mode {
                    self.theme.neon_alert
                } else {
                    self.theme.neon_secondary
                })
            );

            if ui.add(warp_btn).clicked() {
                if let Some(tts) = &mut self.tts {
                    let message = vec![MessagePart::Static("Warp mode activated".to_string())];
                    let personality = PersonalitySettings {
                        drunk_level: 0,
                        sass_level: 0,
                        enthusiasm: 0,
                        anxiety_level: 0,
                        grand_pappi_refs: 0,
                        voice_type: "alloy".to_string(),
                    };
                    self.runtime.block_on(async {
                        let _ = tts.speak(message, &personality).await;
                    });
                }
            }

            if ui.button(if self.personality.audio_enabled { "🔊 Mute" } else { "🔈 Unmute" }).clicked() {
                if let Some(tts) = &mut self.tts {
                    let message = vec![MessagePart::Static("Audio toggled".to_string())];
                    let personality = PersonalitySettings {
                        drunk_level: 0,
                        sass_level: 0,
                        enthusiasm: 0,
                        anxiety_level: 0,
                        grand_pappi_refs: 0,
                        voice_type: "alloy".to_string(),
                    };
                    self.runtime.block_on(async {
                        let _ = tts.speak(message, &personality).await;
                    });
                }
            }
        });
    }

    fn draw_holographic_overlay(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        
        // Calculate hologram flicker
        let flicker = (self.hologram_phase * HOLOGRAM_FLICKER_SPEED).sin() * 0.5 + 0.5;
        let hologram_color = Color32::from_rgba_premultiplied(
            self.theme.hologram.r(),
            self.theme.hologram.g(),
            self.theme.hologram.b(),
            (self.theme.hologram.a() as f32 * flicker * HOLOGRAM_OPACITY) as u8,
        );

        // Draw scanlines
        for y in (rect.min.y as i32..rect.max.y as i32).step_by(4) {
            let y = y as f32;
            let alpha = ((y + self.start_time.elapsed().as_secs_f32() * SCAN_LINE_SPEED).sin() * 0.5 + 0.5) * 0.2;
            
            painter.line_segment(
                [pos2(rect.min.x, y), pos2(rect.max.x, y)],
                Stroke::new(1.0, Color32::from_rgba_premultiplied(
                    hologram_color.r(),
                    hologram_color.g(),
                    hologram_color.b(),
                    (hologram_color.a() as f32 * alpha) as u8,
                )),
            );
        }

        // Draw holographic interface elements
        let interface_rect = rect.shrink(20.0);
        painter.rect_stroke(
            interface_rect,
            4.0,
            Stroke::new(2.0, hologram_color),
        );

        // Add corner decorations
        let corner_size = 10.0;
        for corner in &[
            (interface_rect.min, (1.0, 1.0)),
            (pos2(interface_rect.max.x, interface_rect.min.y), (-1.0, 1.0)),
            (interface_rect.max, (-1.0, -1.0)),
            (pos2(interface_rect.min.x, interface_rect.max.y), (1.0, -1.0)),
        ] {
            painter.line_segment(
                [
                    corner.0,
                    pos2(corner.0.x + corner_size * corner.1.0, corner.0.y),
                ],
                Stroke::new(2.0, hologram_color),
            );
            painter.line_segment(
                [
                    corner.0,
                    pos2(corner.0.x, corner.0.y + corner_size * corner.1.1),
                ],
                Stroke::new(2.0, hologram_color),
            );
        }
    }

    fn draw_bloom_effect(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        let center = rect.center();
        
        // Create a radial bloom effect
        for i in 0..5 {
            let radius = 100.0 + i as f32 * 50.0;
            let alpha = (1.0 - i as f32 * 0.2) * BLOOM_INTENSITY;
            
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(
                    2.0,
                    Color32::from_rgba_premultiplied(
                        self.theme.neon_primary.r(),
                        self.theme.neon_primary.g(),
                        self.theme.neon_primary.b(),
                        (self.theme.neon_primary.a() as f32 * alpha) as u8,
                    ),
                ),
            );
        }
    }

    fn draw_volumetric_fog(&self, ui: &mut egui::Ui, rect: Rect) {
        let painter = ui.painter();
        let mut rng = rand::thread_rng();
        
        // Create volumetric fog effect
        for _ in 0..50 {
            let x = rng.gen_range(rect.min.x..rect.max.x);
            let y = rng.gen_range(rect.min.y..rect.max.y);
            let size = rng.gen_range(20.0..100.0);
            let alpha = rng.gen_range(0.0..FOG_DENSITY);
            
            painter.circle_filled(
                pos2(x, y),
                size,
                Color32::from_rgba_premultiplied(
                    self.theme.volumetric_fog.r(),
                    self.theme.volumetric_fog.g(),
                    self.theme.volumetric_fog.b(),
                    (self.theme.volumetric_fog.a() as f32 * alpha) as u8,
                ),
            );
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate delta time
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        
        // Update hologram phase
        self.hologram_phase += dt;
        
        // Update particle system
        let rect = ctx.available_rect();
        self.particle_system.update(dt, rect);
        
        // Refresh all monitoring systems
        self.monitor.refresh();
        self.system.refresh_cpu();
        self.system.refresh_memory();
        
        // Update network stats
        let network_info = self.monitor.get_network_info();
        if let Some((_, rx, tx)) = network_info.first() {
            self.network_stats.update(*rx, *tx);
        }

        self.check_system_warnings();
        let elapsed = self.start_time.elapsed().as_secs_f32();
        self.neon_pulse = (elapsed * 2.0).sin() * 0.5 + 0.5;
        
        // Set dark theme
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = Color32::from_rgb(13, 17, 23);
        ctx.set_visuals(visuals);

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            
            // Apply a subtle background
            ui.painter().rect_filled(
                rect,
                0.0,
                self.theme.background
            );
            
            // Top bar with minimalist design
            let top_bar_height = 48.0;
            let top_bar_rect = Rect::from_min_size(
                rect.min,
                Vec2::new(rect.width(), top_bar_height),
            );
            
            // Draw elegant frame for top bar
            egui::Frame::none()
                .fill(self.theme.background_light)
                .rounding(Rounding::same(4.0))
                .stroke(Stroke::new(1.0, self.theme.neon_primary))
                .show(ui, |ui| {
                    ui.allocate_rect(top_bar_rect, egui::Sense::hover());
                });
            
            // Title with balanced typography
            ui.painter().text(
                top_bar_rect.center(),
                Align2::CENTER_CENTER,
                "System Monitor",
                FontId::proportional(24.0),
                self.theme.text_bright,
            );

            // Audio and settings controls in top bar with golden ratio spacing
            let controls_width = rect.width() * 0.382; // Golden ratio
            let audio_controls_rect = Rect::from_min_size(
                Pos2::new(10.0, top_bar_rect.min.y + 8.0),
                Vec2::new(controls_width * 0.618, 32.0), // Nested golden ratio
            );
            
            let settings_btn_rect = Rect::from_min_size(
                Pos2::new(rect.max.x - 90.0, top_bar_rect.min.y + 8.0),
                Vec2::new(80.0, 32.0),
            );

            // Audio controls with clean layout
            let mut audio_ui = ui.child_ui(audio_controls_rect, egui::Layout::left_to_right(egui::Align::Center));
            self.show_audio_controls(&mut audio_ui);

            // Settings button with consistent styling
            if ui.put(
                settings_btn_rect,
                egui::Button::new(RichText::new("⚙ Settings").color(self.theme.text_bright))
            ).clicked() {
                self.show_settings = !self.show_settings;
            }

            // Main content area with balanced proportions
            let content_rect = rect.shrink2(Vec2::new(20.0, top_bar_height + 20.0));
            let mut content_ui = ui.child_ui(content_rect, egui::Layout::top_down(egui::Align::LEFT));

            // Left column for system info and CPU
            content_ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(content_rect.width() * 0.382); // Golden ratio
                    
                    // System Info Card
                    egui::Frame::none()
                        .fill(self.theme.background_light)
                        .rounding(Rounding::same(8.0))
                        .stroke(Stroke::new(1.0, self.theme.neon_primary))
                        .show(ui, |ui| {
                            self.draw_system_info_section(ui);
                        });
                    
                    ui.add_space(10.0);
                    
                    // CPU Usage Card
                    egui::Frame::none()
                        .fill(self.theme.background_light)
                        .rounding(Rounding::same(8.0))
                        .stroke(Stroke::new(1.0, self.theme.neon_secondary))
                        .show(ui, |ui| {
                            self.draw_cpu_section(ui);
                        });
                });

                ui.add_space(10.0);

                // Right column for memory, disk, and network
                ui.vertical(|ui| {
                    // Memory Usage Card
                    egui::Frame::none()
                        .fill(self.theme.background_light)
                        .rounding(Rounding::same(8.0))
                        .stroke(Stroke::new(1.0, self.theme.neon_primary))
                        .show(ui, |ui| {
                            self.draw_memory_section(ui);
                        });
                    
                    ui.add_space(10.0);
                    
                    // Disk Usage Card
                    egui::Frame::none()
                        .fill(self.theme.background_light)
                        .rounding(Rounding::same(8.0))
                        .stroke(Stroke::new(1.0, self.theme.neon_primary))
                        .show(ui, |ui| {
                            self.draw_disk_section(ui);
                        });
                    
                    ui.add_space(10.0);
                    
                    // Network Usage Card
                    egui::Frame::none()
                        .fill(self.theme.background_light)
                        .rounding(Rounding::same(8.0))
                        .stroke(Stroke::new(1.0, self.theme.neon_primary))
                        .show(ui, |ui| {
                            self.draw_network_section(ui);
                        });
                });
            });

            // Settings window with clean design
            if self.show_settings {
                self.show_settings_window(ctx);
            }
        });

        // Request continuous updates for animations
        ctx.request_repaint();
    }
}

impl CyberNinjaApp {
    fn draw_system_info_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("System Information").color(self.theme.text_bright));
            ui.add_space(4.0);
            
            let (name, kernel, os_version, hostname) = self.monitor.get_system_info();
            ui.label(RichText::new(format!("OS: {}", name)).color(self.theme.text_bright));
            ui.label(RichText::new(format!("Kernel: {}", kernel)).color(self.theme.text_dim));
            ui.label(RichText::new(format!("Version: {}", os_version)).color(self.theme.text_dim));
            ui.label(RichText::new(format!("Hostname: {}", hostname)).color(self.theme.text_dim));
            ui.add_space(8.0);
        });
    }

    fn draw_cpu_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("CPU Usage").color(self.theme.text_bright));
            ui.add_space(4.0);
            
            for (name, usage) in self.monitor.get_cpu_usage() {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&name).color(self.theme.text_dim));
                    self.draw_value_bar(ui, usage / 100.0, self.theme.neon_secondary);
                    ui.label(RichText::new(format!("{:.1}%", usage)).color(self.theme.text_bright));
                });
            }
            ui.add_space(8.0);
        });
    }

    fn draw_memory_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("Memory Usage").color(self.theme.text_bright));
            ui.add_space(4.0);
            
            let (total, used, usage) = self.monitor.get_memory_info();
            ui.label(RichText::new(format!("Total: {:.1} GB", total as f64 / 1024.0 / 1024.0 / 1024.0)).color(self.theme.text_bright));
            ui.label(RichText::new(format!("Used: {:.1} GB", used as f64 / 1024.0 / 1024.0 / 1024.0)).color(self.theme.text_dim));
            self.draw_value_bar(ui, usage / 100.0, self.theme.neon_primary);
            ui.label(RichText::new(format!("Usage: {:.1}%", usage)).color(self.theme.text_bright));
            ui.add_space(8.0);
        });
    }

    fn draw_disk_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("Disk Usage").color(self.theme.text_bright));
            ui.add_space(4.0);
            
            for (mount_point, total, available) in self.monitor.get_disk_info() {
                let used = total - available;
                let usage = (used as f64 / total as f64) * 100.0;
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&mount_point).color(self.theme.text_dim));
                    self.draw_value_bar(ui, usage as f32 / 100.0, self.theme.neon_primary);
                    ui.label(RichText::new(format!("{:.1}%", usage)).color(self.theme.text_bright));
                });
                ui.label(RichText::new(format!("{:.1} GB free of {:.1} GB",
                    available as f64 / 1024.0 / 1024.0 / 1024.0,
                    total as f64 / 1024.0 / 1024.0 / 1024.0
                )).color(self.theme.text_dim));
            }
            ui.add_space(8.0);
        });
    }

    fn draw_network_section(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("Network Usage").color(self.theme.text_bright));
            ui.add_space(4.0);
            
            for (interface, rx, tx) in self.monitor.get_network_info() {
                ui.label(RichText::new(&interface).color(self.theme.text_bright));
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Download:").color(self.theme.text_dim));
                    ui.label(RichText::new(format!("{:.2} MB/s", rx as f64 / 1024.0 / 1024.0)).color(self.theme.text_bright));
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Upload:").color(self.theme.text_dim));
                    ui.label(RichText::new(format!("{:.2} MB/s", tx as f64 / 1024.0 / 1024.0)).color(self.theme.text_bright));
                });
            }
            ui.add_space(8.0);
        });
    }
}

fn load_svg_icon(ctx: &egui::Context, svg_data: &[u8]) -> egui::TextureHandle {
    let svg_str = std::str::from_utf8(svg_data).unwrap();
    
    // Parse SVG with proper options
    let opt = usvg::Options::default();
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    
    let tree = usvg::Tree::from_str(svg_str, &opt).unwrap();
    let view_box = tree.view_box.rect;
    let pixmap_size = view_box.size();
    
    let mut pixmap = tiny_skia::Pixmap::new(
        pixmap_size.width() as u32,
        pixmap_size.height() as u32
    ).unwrap();

    // Create the transform for rendering
    let transform = tiny_skia::Transform::default();
    
    // Render using resvg
    let rtree = resvg::Tree::from_usvg(&tree);
    rtree.render(transform, &mut pixmap.as_mut());

    // Convert to RGBA
    let pixels = pixmap.data();
    let rgba: Vec<u8> = pixels.chunks(4).flat_map(|p| [p[2], p[1], p[0], p[3]]).collect();

    ctx.load_texture(
        format!("icon-{}", svg_str.len()), // Unique ID for each icon
        egui::ColorImage::from_rgba_unmultiplied(
            [pixmap_size.width() as usize, pixmap_size.height() as usize],
            &rgba,
        ),
        egui::TextureOptions::default(),
    )
}

impl eframe::App for CyberNinjaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update(ctx, _frame);
    }
}

fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    println!("Environment variables loaded from .env file");

    let native_options = NativeOptions {
        renderer: eframe::Renderer::Glow,
        multisampling: 0,
        depth_buffer: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        vsync: true,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        window_builder: Some(Box::new(|builder| {
            builder
                .with_min_inner_size([800.0, 600.0])
                .with_inner_size([1024.0, 768.0])
        })),
        ..Default::default()
    };

    // Initialize window with error handling
    println!("Initializing window...");
    match eframe::run_native(
        "Cyber Ninja Monitor",
        native_options,
        Box::new(|cc| {
            println!("Creating application instance...");
            Box::new(CyberNinjaApp::new(cc))
        })
    ) {
        Ok(_) => println!("Application closed successfully"),
        Err(e) => {
            eprintln!("Error running application: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test NetworkStats struct
    mod network_stats_tests {
        use super::*;

        #[test]
        fn test_network_stats_new() {
            let stats = NetworkStats::new();
            assert_eq!(stats.bytes_received, 0);
            assert_eq!(stats.bytes_sent, 0);
            assert_eq!(stats.receive_rate, 0.0);
            assert_eq!(stats.send_rate, 0.0);
        }

        #[test]
        fn test_network_stats_update() {
            let mut stats = NetworkStats::new();
            
            // Test initial update
            stats.update(1000, 500);
            assert_eq!(stats.bytes_received, 1000);
            assert_eq!(stats.bytes_sent, 500);
            
            // Test subsequent update
            stats.update(2000, 1000);
            assert_eq!(stats.bytes_received, 2000);
            assert_eq!(stats.bytes_sent, 1000);
        }
    }
}

#[cfg(test)]
mod window_tests {
    use super::*;
    use eframe::{Frame, NativeOptions};
    use egui::Context;

    pub fn create_test_app() -> CyberNinjaApp {
        let theme = theme::CyberTheme::default();
        CyberNinjaApp {
            system: System::new_all(),
            start_time: Instant::now(),
            neon_pulse: 0.0,
            tts: None,
            last_cpu_warning: None,
            last_memory_warning: None,
            last_status_update: Instant::now(),
            show_settings: false,
            settings_volume: 0.8,
            settings_cpu_threshold: 80.0,
            settings_update_interval: 1.0,
            network_stats: NetworkStats::new(),
            cpu_icon: None,
            memory_icon: None,
            disk_icon: None,
            alert_glitch: None,
            monitor: SystemMonitor::new(),
            personality: AIPersonality::default(),
            editing_catchphrase: String::new(),
            theme: theme.clone(),
            shurikens: Vec::new(),
            last_frame_time: Instant::now(),
            warp_effect_intensity: 0.0,
            particle_system: ParticleSystem::new(theme),
            hologram_phase: 0.0,
            runtime: Runtime::new().unwrap(),
        }
    }

    pub fn create_mock_frame() -> Frame {
        unsafe { std::mem::zeroed() }
    }

    #[test]
    fn test_window_creation() {
        let native_options = NativeOptions {
            renderer: eframe::Renderer::Glow,
            multisampling: 0,
            depth_buffer: 0,
            hardware_acceleration: eframe::HardwareAcceleration::Required,
            vsync: true,
            follow_system_theme: false,
            default_theme: eframe::Theme::Dark,
            ..Default::default()
        };
        // ... existing code ...
    }

    #[test]
    fn test_window_settings() {
        let ctx = Context::default();
        let mut app = create_test_app();
        let mut frame = create_mock_frame();
        // ... existing code ...
    }

    #[test]
    fn test_ui_layout() {
        let ctx = Context::default();
        let mut app = create_test_app();
        let mut frame = create_mock_frame();
        // ... existing code ...
    }
} 