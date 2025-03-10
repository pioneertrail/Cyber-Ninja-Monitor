use eframe::egui::{Color32, Pos2};

pub struct CyberTheme {
    pub background: Color32,
    pub background_light: Color32,
    pub background_dark: Color32,
    pub foreground: Color32,
    pub foreground_dim: Color32,
    pub accent: Color32,
    pub text_bright: Color32,
    pub text_dim: Color32,
    pub neon_primary: Color32,
    pub neon_secondary: Color32,
    pub neon_alert: Color32,
    pub grid_line: Color32,
}

impl Default for CyberTheme {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(16, 20, 28),
            background_light: Color32::from_rgb(24, 28, 36),
            background_dark: Color32::from_rgb(12, 16, 24),
            foreground: Color32::from_rgb(200, 210, 220),
            foreground_dim: Color32::from_rgb(140, 150, 160),
            accent: Color32::from_rgb(64, 128, 255),
            text_bright: Color32::from_rgb(220, 230, 240),
            text_dim: Color32::from_rgb(160, 170, 180),
            neon_primary: Color32::from_rgb(0, 255, 196),
            neon_secondary: Color32::from_rgb(255, 64, 128),
            neon_alert: Color32::from_rgb(255, 64, 64),
            grid_line: Color32::from_rgba_premultiplied(64, 128, 255, 48),
        }
    }
}

// UI constants
pub const WINDOW_ROUNDING: f32 = 4.0;
pub const WINDOW_SHADOW: f32 = 8.0;
pub const PANEL_SPACING: f32 = 8.0;
pub const BORDER_WIDTH: f32 = 1.0;
pub const GRID_SIZE: i32 = 32;
pub const HEADER_HEIGHT: f32 = 32.0;

// Animation constants
pub const PULSE_SPEED: f32 = 2.0;
pub const GLITCH_INTERVAL: f32 = 20.0;
pub const SCAN_LINE_SPEED: f32 = 1.0;

// Shuriken constants
const SHURIKEN_SIZE: f32 = 20.0;
const SHURIKEN_SPIN_SPEED: f32 = 3.0;

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