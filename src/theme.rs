use eframe::egui::{Color32, Pos2};

#[derive(Clone)]
pub struct CyberTheme {
    pub accent: egui::Color32,
    pub background: egui::Color32,
    pub background_light: egui::Color32,
    pub foreground: egui::Color32,
    pub text_bright: egui::Color32,
    pub text_dim: egui::Color32,
    pub neon_primary: egui::Color32,
    pub neon_secondary: egui::Color32,
    pub neon_alert: egui::Color32,
    pub grid_line: egui::Color32,
    pub hologram: egui::Color32,
    pub volumetric_fog: egui::Color32,
}

impl Default for CyberTheme {
    fn default() -> Self {
        Self {
            accent: egui::Color32::from_rgb(0, 255, 255),
            background: egui::Color32::from_rgb(0, 0, 20),
            background_light: egui::Color32::from_rgb(10, 10, 30),
            foreground: egui::Color32::from_rgb(200, 200, 200),
            text_bright: egui::Color32::from_rgb(255, 255, 255),
            text_dim: egui::Color32::from_rgb(150, 150, 150),
            neon_primary: egui::Color32::from_rgb(0, 255, 255),
            neon_secondary: egui::Color32::from_rgb(255, 0, 255),
            neon_alert: egui::Color32::from_rgb(255, 0, 0),
            grid_line: egui::Color32::from_rgba_premultiplied(0, 255, 255, 100),
            hologram: egui::Color32::from_rgba_premultiplied(0, 255, 255, 150),
            volumetric_fog: egui::Color32::from_rgba_premultiplied(0, 255, 255, 50),
        }
    }
}

// Core colors
pub const ACCENT_COLOR: Color32 = Color32::from_rgb(0, 255, 136);
pub const BACKGROUND_COLOR: Color32 = Color32::from_rgb(16, 24, 32);
pub const BACKGROUND_DARK: Color32 = Color32::from_rgb(8, 12, 16);
pub const FOREGROUND_COLOR: Color32 = Color32::from_rgb(200, 255, 200);
pub const FOREGROUND_DIM: Color32 = Color32::from_rgb(100, 128, 100);

// UI constants
pub const WINDOW_ROUNDING: f32 = 4.0;
pub const WINDOW_SHADOW: f32 = 8.0;
pub const PANEL_SPACING: f32 = 8.0;
pub const BORDER_WIDTH: f32 = 2.0;
pub const GRID_SIZE: i32 = 32;
pub const HEADER_HEIGHT: f32 = 48.0;

// Animation constants
pub const PULSE_SPEED: f32 = 1.0;
pub const GLITCH_INTERVAL: f32 = 20.0;
pub const SCAN_LINE_SPEED: f32 = 2.0;
pub const HOLOGRAM_FLICKER_SPEED: f32 = 5.0;
pub const PARTICLE_SPEED: f32 = 3.0;
pub const DIGITAL_RAIN_SPEED: f32 = 2.0;

// Particle system constants
pub const MAX_PARTICLES: usize = 1000;
pub const PARTICLE_LIFETIME: f32 = 2.0;
pub const SPARK_SIZE: f32 = 2.0;
pub const RAIN_CHAR_SIZE: f32 = 14.0;

// Effect intensities
pub const BLOOM_INTENSITY: f32 = 0.5;
pub const FOG_DENSITY: f32 = 0.3;
pub const GLITCH_INTENSITY: f32 = 0.2;
pub const HOLOGRAM_OPACITY: f32 = 0.6;

// Shuriken constants
pub const SHURIKEN_SIZE: f32 = 20.0;
pub const SHURIKEN_SPIN_SPEED: f32 = 3.0;

pub struct Shuriken {
    pub pos: Pos2,
    pub angle: f32,
    pub size: f32,
    pub color: Color32,
    pub speed: f32,
}

impl Shuriken {
    pub fn new(pos: Pos2, color: Color32) -> Self {
        Self {
            pos,
            angle: 0.0,
            size: SHURIKEN_SIZE,
            color,
            speed: SHURIKEN_SPIN_SPEED,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.angle += self.speed * dt;
        if self.angle > 2.0 * std::f32::consts::PI {
            self.angle -= 2.0 * std::f32::consts::PI;
        }
    }

    pub fn get_points(&self) -> Vec<Pos2> {
        let mut points = Vec::with_capacity(4);
        for i in 0..4 {
            let point_angle = self.angle + (i as f32) * std::f32::consts::PI / 2.0;
            points.push(Pos2::new(
                self.pos.x + self.size * point_angle.cos(),
                self.pos.y + self.size * point_angle.sin(),
            ));
        }
        points
    }
}

pub fn pulse_color(color: Color32, intensity: f32) -> Color32 {
    let r = (color.r() as f32 * intensity) as u8;
    let g = (color.g() as f32 * intensity) as u8;
    let b = (color.b() as f32 * intensity) as u8;
    let a = color.a();
    Color32::from_rgba_unmultiplied(r, g, b, a)
} 