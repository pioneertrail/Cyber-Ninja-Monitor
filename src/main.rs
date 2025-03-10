use eframe::egui::{
    self, Color32, Pos2, RichText, Rect, vec2, TextureHandle, Vec2, Align2, FontId, Stroke, pos2,
    ViewportBuilder,
};
use sysinfo::{System, SystemExt, CpuExt};
use std::time::{Duration, Instant};
use dotenv::dotenv;
use crate::tts::TTSManager;
use crate::system_monitor::SystemMonitor;
use usvg::TreeParsing;
use crate::theme::{CyberTheme, GRID_SIZE, HEADER_HEIGHT, PULSE_SPEED, SCAN_LINE_SPEED, GLITCH_INTERVAL};
use crate::ai_personality::AIPersonality;
use crate::audio_manager::AudioManager;

mod tts;
mod system_monitor;
mod theme;
mod audio_manager;
mod ai_personality;

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
    tts_manager: Option<TTSManager>,
    last_cpu_warning: Option<Instant>,
    last_memory_warning: Option<Instant>,
    last_status_update: Option<Instant>,
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
    last_frame_time: std::time::Instant,
    warp_effect_intensity: f32,
}

impl CyberNinjaApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        println!("Initializing CyberNinjaApp...");
        dotenv().ok();
        
        let mut sys = System::new();
        sys.refresh_cpu();
        
        let mut tts_manager = TTSManager::new();
        if let Some(ref mut tts) = tts_manager {
            if let Err(e) = tts.speak(
                "Yo! Systems are lit and running sweet as candy! Grand Pappi Scotty would be proud - he always said a well-tuned system purrs like his old quantum bike. Let's get this party started, Captain!",
                "startup"
            ) {
                eprintln!("TTS Error: {}", e);
            }
        }

        // Load icons
        let ctx = &cc.egui_ctx;
        let cpu_icon = load_svg_icon(ctx, CPU_ICON);
        let memory_icon = load_svg_icon(ctx, MEMORY_ICON);
        let disk_icon = load_svg_icon(ctx, DISK_ICON);

        // Initialize shurikens
        let mut shurikens = Vec::new();
        let theme = theme::CyberTheme::default();
        
        // Add decorative shurikens
        shurikens.push(theme::Shuriken::new(
            Pos2::new(50.0, 50.0),
            theme.neon_primary
        ));
        shurikens.push(theme::Shuriken::new(
            Pos2::new(974.0, 50.0),
            theme.neon_secondary
        ));
        
        Self {
            system: sys,
            start_time: Instant::now(),
            neon_pulse: 0.0,
            tts_manager,
            last_cpu_warning: None,
            last_memory_warning: None,
            last_status_update: None,
            show_settings: false,
            settings_volume: 0.8,
            settings_cpu_threshold: 80.0,
            settings_update_interval: 300.0,
            network_stats: NetworkStats::new(),
            cpu_icon: Some(cpu_icon),
            memory_icon: Some(memory_icon),
            disk_icon: Some(disk_icon),
            alert_glitch: None,
            monitor: SystemMonitor::new(),
            personality: AIPersonality::default(),
            editing_catchphrase: String::new(),
            shurikens,
            last_frame_time: std::time::Instant::now(),
            warp_effect_intensity: 0.0,
            theme,
        }
    }

    fn generate_message(&self, base_message: &str) -> String {
        let mut message = base_message.to_string();
        let mut prefix = String::new();
        let mut suffix = String::new();
        
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
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        let memory_used = self.system.used_memory() as f32 / self.system.total_memory() as f32;
        
        // CPU warning (every 30 seconds)
        if cpu_usage > self.settings_cpu_threshold {
            if self.last_cpu_warning
                .map_or(true, |last| last.elapsed().as_secs() > 30)
            {
                self.last_cpu_warning = Some(Instant::now());
                self.alert_glitch = Some(Instant::now());
                
                let base_message = format!(
                    "The processors are running super hot! They're at {}%! Need to cool them down!",
                    cpu_usage
                );
                let message = self.generate_message(&base_message);
                
                if let Some(tts) = &mut self.tts_manager {
                    println!("Attempting to announce CPU warning...");
                    tts.set_volume(self.settings_volume);
                    if let Err(e) = tts.speak(&message, "cpu_warning") {
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
                self.alert_glitch = Some(Instant::now());
                
                let base_message = format!(
                    "Memory banks are stuffed fuller than Grand Pappi's tool shed! We're using about {}% - time to dump some data before this whole rig goes sideways!",
                    memory_used * 100.0
                );
                let message = self.generate_message(&base_message);
                
                if let Some(tts) = &mut self.tts_manager {
                    println!("Attempting to announce memory warning...");
                    tts.set_volume(self.settings_volume);
                    if let Err(e) = tts.speak(&message, "memory_warning") {
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
            
            let cpu_status = if cpu_usage < 30.0 {
                "super chill"
            } else if cpu_usage < 60.0 {
                "cruising along"
            } else {
                "working overtime"
            };

            let mem_status = if memory_used < 0.5 {
                "barely breaking a sweat"
            } else if memory_used < 0.8 {
                "getting cozy"
            } else {
                "packed like a cyber-rave"
            };

            let base_status = format!(
                "System check! Processors are {} at {}%, and memory is {} at {}%",
                cpu_status,
                cpu_usage,
                mem_status,
                memory_used * 100.0
            );
            
            let message = self.generate_message(&base_status);
            
            if let Some(tts) = &mut self.tts_manager {
                tts.set_volume(self.settings_volume);
                if let Err(e) = tts.speak(&message, "status_update") {
                    println!("Error announcing status update: {}", e);
                }
            }
        }
    }

    fn show_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("AI Personality Settings")
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                ui.heading("Voice Settings");
                ui.horizontal(|ui| {
                    ui.label("Voice Type:");
                    ui.text_edit_singleline(&mut self.personality.voice_type);
                });

                // Audio Controls
                ui.heading("Audio Controls");
                ui.horizontal(|ui| {
                    if ui.button(if self.personality.audio_enabled { "🔊 Mute" } else { "🔈 Unmute" }).clicked() {
                        let message = self.personality.toggle_audio();
                        if let Some(tts) = &mut self.tts_manager {
                            let _ = tts.speak(&message, "audio_toggle");
                        }
                    }
                    if ui.button("🔄 Reset Audio").clicked() {
                        let message = self.personality.reset_audio();
                        if let Some(tts) = &mut self.tts_manager {
                            tts.set_volume(self.personality.volume);
                            let _ = tts.speak(&message, "audio_reset");
                        }
                    }
                });

                ui.add(egui::Slider::new(&mut self.personality.volume, 0.0..=1.0)
                    .text("Volume")
                    .clamp_to_range(true));
                ui.add(egui::Slider::new(&mut self.personality.speech_rate, 0.5..=2.0)
                    .text("Speech Rate")
                    .clamp_to_range(true));

                ui.heading("Personality Traits");
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

                ui.heading("Catchphrases");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.editing_catchphrase);
                    if ui.button("Add").clicked() && !self.editing_catchphrase.is_empty() {
                        self.personality.catchphrases.push(self.editing_catchphrase.clone());
                        self.editing_catchphrase.clear();
                    }
                });

                for catchphrase in &self.personality.catchphrases {
                    ui.label(catchphrase);
                }

                // Exit button at the bottom
                ui.separator();
                if ui.button("🚪 Exit Application").clicked() {
                    let exit_message = self.personality.get_exit_message();
                    if let Some(tts) = &mut self.tts_manager {
                        let _ = tts.speak(&exit_message, "exit");
                        // Give it a moment to speak before exiting
                        std::thread::sleep(std::time::Duration::from_secs(2));
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
            if ui.button("🔊 Test Audio").clicked() && self.personality.audio_enabled {
                if let Some(tts) = &mut self.tts_manager {
                    let message = "Systems operational! Testing audio subsystems.";
                    if let Err(e) = tts.speak(message, "test") {
                        println!("Audio test error: {}", e);
                    }
                }
            }

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
                if let Some(tts) = &mut self.tts_manager {
                    let message = self.personality.toggle_1337_mode();
                    tts.set_speech_rate(if self.personality.is_1337_mode { 2.0 } else { 1.0 });
                    if let Err(e) = tts.speak(&message, "warp") {
                        println!("Warp drive error: {}", e);
                    }
                }
            }

            if ui.button(if self.personality.audio_enabled { "🔊 Mute" } else { "🔈 Unmute" }).clicked() {
                let message = self.personality.toggle_audio();
                if let Some(tts) = &mut self.tts_manager {
                    tts.set_audio_enabled(self.personality.audio_enabled);
                    if self.personality.audio_enabled {
                        let _ = tts.speak(&message, "audio_toggle");
                    }
                }
            }
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
            
            // Draw background
            ui.painter().rect_filled(
                rect,
                0.0,
                self.theme.background_dark,
            );
            
            // Draw animated grid with fade effect
            self.draw_grid(ui, rect);
            
            // Draw spinning shurikens
            self.draw_shurikens(ui);
            
            // Top bar with cyberpunk style
            let top_bar_height = theme::HEADER_HEIGHT;
            let top_bar_rect = Rect::from_min_size(
                rect.min,
                Vec2::new(rect.width(), top_bar_height),
            );
            
            // Draw neon frame around top bar
            self.draw_neon_frame(ui, top_bar_rect);
            
            // Title with neon effect
            ui.painter().text(
                top_bar_rect.center(),
                Align2::CENTER_CENTER,
                "CYBER NINJA",
                FontId::monospace(24.0),
                self.theme.neon_primary,
            );

            // Audio controls in top bar
            let audio_controls_rect = Rect::from_min_size(
                Pos2::new(top_bar_rect.min.x + 10.0, top_bar_rect.min.y + 5.0),
                Vec2::new(300.0, top_bar_height - 10.0),
            );
            let mut audio_ui = ui.child_ui(audio_controls_rect, egui::Layout::left_to_right(egui::Align::Center));
            self.show_audio_controls(&mut audio_ui);

            // Settings button (moved to right side)
            let settings_btn_rect = Rect::from_min_size(
                Pos2::new(rect.max.x - 100.0, top_bar_rect.min.y + 10.0),
                Vec2::new(80.0, 30.0),
            );

            if ui.put(
                settings_btn_rect,
                egui::Button::new(
                    RichText::new("Settings")
                        .color(self.theme.text_bright)
                )
            ).clicked() {
                self.show_settings = !self.show_settings;
            }

            // Main content area
            let content_rect = rect.shrink2(Vec2::new(20.0, top_bar_height + 20.0));
            let mut content_ui = ui.child_ui(content_rect, egui::Layout::top_down(egui::Align::LEFT));

            // System Info Section with cyberpunk styling
            content_ui.group(|ui| {
                self.draw_section_header(ui, "System Information", self.theme.neon_primary);
                ui.horizontal(|ui| {
                    if let Some(cpu_icon) = &self.cpu_icon {
                        ui.add(egui::Image::new(cpu_icon).max_size(Vec2::new(24.0, 24.0)));
                    }
                    ui.vertical(|ui| {
                        let (name, kernel, os_version, hostname) = self.monitor.get_system_info();
                        ui.label(RichText::new(format!("OS: {}", name)).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("Kernel: {}", kernel)).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("Version: {}", os_version)).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("Hostname: {}", hostname)).color(self.theme.text_bright));
                    });
                });
            });

            // CPU Section
            content_ui.group(|ui| {
                self.draw_section_header(ui, "CPU Usage", self.theme.neon_secondary);
                ui.horizontal(|ui| {
                    if let Some(cpu_icon) = &self.cpu_icon {
                        ui.add(egui::Image::new(cpu_icon).max_size(Vec2::new(24.0, 24.0)));
                    }
                    ui.vertical(|ui| {
                        for (name, usage) in self.monitor.get_cpu_usage() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&name).color(self.theme.text_dim));
                                self.draw_value_bar(ui, usage / 100.0, self.theme.neon_secondary);
                                ui.label(RichText::new(format!("{:.1}%", usage)).color(self.theme.text_bright));
                            });
                        }
                    });
                });
            });

            // Memory Section
            content_ui.group(|ui| {
                self.draw_section_header(ui, "Memory Usage", self.theme.neon_primary);
                ui.horizontal(|ui| {
                    if let Some(memory_icon) = &self.memory_icon {
                        ui.add(egui::Image::new(memory_icon).max_size(Vec2::new(24.0, 24.0)));
                    }
                    ui.vertical(|ui| {
                        let (total, used, usage) = self.monitor.get_memory_info();
                        ui.label(RichText::new(format!("Total: {} GB", total / 1024 / 1024 / 1024)).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("Used: {} GB", used / 1024 / 1024 / 1024)).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("Usage: {:.1}%", usage)).color(self.theme.text_bright));
                    });
                });
            });

            // Disk Section
            content_ui.group(|ui| {
                self.draw_section_header(ui, "Disk Usage", self.theme.neon_primary);
                ui.horizontal(|ui| {
                    if let Some(disk_icon) = &self.disk_icon {
                        ui.add(egui::Image::new(disk_icon).max_size(Vec2::new(24.0, 24.0)));
                    }
                    ui.vertical(|ui| {
                        for (mount_point, total, available) in self.monitor.get_disk_info() {
                            let used = total - available;
                            let usage = (used as f64 / total as f64) * 100.0;
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(format!("{}: {:.1}% ({:.1} GB free of {:.1} GB)",
                                    mount_point,
                                    usage,
                                    available as f64 / 1024.0 / 1024.0 / 1024.0,
                                    total as f64 / 1024.0 / 1024.0 / 1024.0
                                )).color(self.theme.text_dim));
                            });
                        }
                    });
                });
            });

            // Network Usage Section
            content_ui.group(|ui| {
                self.draw_section_header(ui, "Network Usage", self.theme.neon_primary);
                for (interface, rx, tx) in self.monitor.get_network_info() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&interface).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("↓ {:.2} MB/s", rx as f64 / 1024.0 / 1024.0)).color(self.theme.text_bright));
                        ui.label(RichText::new(format!("↑ {:.2} MB/s", tx as f64 / 1024.0 / 1024.0)).color(self.theme.text_bright));
                    });
                }
            });

            // Top Processes Section
            content_ui.group(|ui| {
                self.draw_section_header(ui, "Top Processes", self.theme.neon_primary);
                let mut processes = self.monitor.get_process_info();
                processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
                for (pid, name, cpu_usage) in processes.iter().take(5) {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("[{}] {}", pid, name)).color(self.theme.text_bright));
                        ui.add(egui::ProgressBar::new(*cpu_usage / 100.0)
                            .text(format!("{:.1}%", cpu_usage))
                            .fill(Color32::from_rgb(
                                (255.0 * cpu_usage / 100.0) as u8,
                                (255.0 * (1.0 - cpu_usage / 100.0)) as u8,
                                0,
                            ))
                        );
                    });
                }
            });

            // Settings window
            if self.show_settings {
                self.show_settings_window(ctx);
            }

            // Draw glitch effect when alerts are active
            if let Some(start_time) = &self.alert_glitch {
                let elapsed = start_time.elapsed().as_secs_f32();
                if elapsed < 3.0 {
                    let glitch_intensity = (1.0 - elapsed / 3.0) * 
                        ((elapsed * theme::GLITCH_INTERVAL).sin() * 0.5 + 0.5);
                    let warning_color = Color32::from_rgba_premultiplied(
                        self.theme.neon_alert.r(),
                        self.theme.neon_alert.g(),
                        self.theme.neon_alert.b(),
                        (255.0 * glitch_intensity) as u8,
                    );
                    
                    // Random offset for glitch effect
                    let offset = (elapsed * 10.0).sin() * 5.0;
                    
                    ui.painter().text(
                        rect.center() + Vec2::new(offset, 0.0),
                        Align2::CENTER_CENTER,
                        "SYSTEM ALERT",
                        FontId::monospace(32.0),
                        warning_color,
                    );
                } else {
                    self.alert_glitch = None;
                }
            }

            // Update warp effect intensity
            if self.personality.is_1337_mode {
                self.warp_effect_intensity = (self.warp_effect_intensity + 0.1).min(1.0);
            } else {
                self.warp_effect_intensity = (self.warp_effect_intensity - 0.1).max(0.0);
            }

            // Apply warp speed visual effects
            if self.warp_effect_intensity > 0.0 {
                let rect = ctx.screen_rect();
                let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("warp_effect")));
                
                // Create warp tunnel effect
                let num_lines = 50;
                for i in 0..num_lines {
                    let t = i as f32 / num_lines as f32;
                    let alpha = (1.0 - t) * self.warp_effect_intensity * 0.5;
                    let color = Color32::from_rgba_premultiplied(
                        self.theme.neon_primary.r(),
                        self.theme.neon_primary.g(),
                        self.theme.neon_primary.b(),
                        (alpha * 255.0) as u8,
                    );
                    
                    let center = rect.center();
                    let radius = t * rect.width() * self.warp_effect_intensity;
                    let points = (0..36).map(|j| {
                        let angle = j as f32 * std::f32::consts::PI * 2.0 / 36.0;
                        let x = center.x + angle.cos() * radius;
                        let y = center.y + angle.sin() * radius;
                        Pos2::new(x, y)
                    }).collect::<Vec<_>>();
                    
                    painter.add(egui::Shape::line(points, Stroke::new(2.0, color)));
                }

                // Add random "stars" streaking effect
                let num_stars = (50.0 * self.warp_effect_intensity) as i32;
                for _ in 0..num_stars {
                    let x = rand::random::<f32>() * rect.width();
                    let y = rand::random::<f32>() * rect.height();
                    let length = 20.0 * self.warp_effect_intensity;
                    
                    painter.line_segment(
                        [
                            Pos2::new(rect.min.x + x, rect.min.y + y),
                            Pos2::new(rect.min.x + x + length, rect.min.y + y),
                        ],
                        Stroke::new(1.0, self.theme.neon_secondary),
                    );
                }
            }
        });

        // Request continuous updates for animations
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    dotenv().ok();
    
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
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