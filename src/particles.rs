use egui::{Pos2, Vec2, Rect, Color32};
use rand::random;
use crate::theme::CyberTheme;

pub struct Particle {
    pub position: Pos2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub initial_lifetime: f32,
    pub color: Color32,
    pub size: f32,
}

impl Particle {
    pub fn new(position: Pos2, velocity: Vec2, lifetime: f32, color: Color32, size: f32) -> Self {
        Self {
            position,
            velocity,
            lifetime,
            initial_lifetime: lifetime,
            color,
            size,
        }
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    theme: CyberTheme,
}

impl ParticleSystem {
    pub fn new(theme: CyberTheme) -> Self {
        Self {
            particles: Vec::new(),
            theme,
        }
    }

    pub fn update(&mut self, dt: f32, bounds: Rect) {
        self.particles.retain_mut(|particle| {
            particle.position += particle.velocity * dt;
            particle.lifetime -= dt;

            // Bounce off the edges of the bounds
            if particle.position.x < bounds.min.x || particle.position.x > bounds.max.x {
                particle.velocity.x = -particle.velocity.x;
            }
            if particle.position.y < bounds.min.y || particle.position.y > bounds.max.y {
                particle.velocity.y = -particle.velocity.y;
            }

            particle.lifetime > 0.0
        });
    }

    pub fn emit(&mut self, position: Pos2) {
        let angle = random::<f32>() * std::f32::consts::TAU;
        let speed = random::<f32>() * 100.0 + 50.0;
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        let lifetime = random::<f32>() * 2.0 + 1.0;
        let color = self.theme.neon_primary;
        let size = random::<f32>() * 10.0 + 5.0;

        self.particles.push(Particle::new(position, velocity, lifetime, color, size));
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        let painter = ui.painter();
        
        for particle in &self.particles {
            let alpha = particle.lifetime / particle.initial_lifetime;
            let color = Color32::from_rgba_unmultiplied(
                particle.color.r(),
                particle.color.g(),
                particle.color.b(),
                (particle.color.a() as f32 * alpha) as u8,
            );

            painter.circle_filled(
                particle.position,
                particle.size,
                color,
            );
        }
    }
} 