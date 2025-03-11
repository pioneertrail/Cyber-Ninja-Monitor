use eframe::egui::{self, Color32, Pos2, Vec2, Rect};
use rand::Rng;
use std::time::Instant;
use crate::theme::{CyberTheme, MAX_PARTICLES, PARTICLE_LIFETIME, SPARK_SIZE, RAIN_CHAR_SIZE, PARTICLE_SPEED};

pub struct Particle {
    pos: Pos2,
    velocity: Vec2,
    color: Color32,
    lifetime: f32,
    size: f32,
    created: Instant,
}

impl Particle {
    fn new(pos: Pos2, velocity: Vec2, color: Color32, size: f32) -> Self {
        Self {
            pos,
            velocity,
            color,
            lifetime: PARTICLE_LIFETIME,
            size,
            created: Instant::now(),
        }
    }

    fn update(&mut self, dt: f32) -> bool {
        let age = self.created.elapsed().as_secs_f32();
        if age >= self.lifetime {
            return false;
        }

        self.pos += self.velocity * dt;
        true
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
    last_spawn: Instant,
    theme: CyberTheme,
}

impl ParticleSystem {
    pub fn new(theme: CyberTheme) -> Self {
        Self {
            particles: Vec::with_capacity(MAX_PARTICLES),
            last_spawn: Instant::now(),
            theme,
        }
    }

    pub fn update(&mut self, dt: f32, rect: Rect) {
        // Remove dead particles
        self.particles.retain_mut(|p| p.update(dt));

        // Spawn new particles
        if self.last_spawn.elapsed().as_secs_f32() > 0.1 {
            self.spawn_particles(rect);
            self.last_spawn = Instant::now();
        }
    }

    fn spawn_particles(&mut self, rect: Rect) {
        let mut rng = rand::thread_rng();

        // Spawn digital rain
        if self.particles.len() < MAX_PARTICLES {
            let x = rng.gen_range(rect.min.x..rect.max.x);
            let pos = Pos2::new(x, rect.min.y);
            let velocity = Vec2::new(0.0, PARTICLE_SPEED * 50.0);
            
            self.particles.push(Particle::new(
                pos,
                velocity,
                self.theme.digital_rain,
                RAIN_CHAR_SIZE,
            ));
        }

        // Spawn energy sparks
        if self.particles.len() < MAX_PARTICLES {
            for _ in 0..5 {
                let x = rng.gen_range(rect.min.x..rect.max.x);
                let y = rng.gen_range(rect.min.y..rect.max.y);
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let speed = rng.gen_range(50.0..150.0);
                
                let velocity = Vec2::new(
                    angle.cos() * speed,
                    angle.sin() * speed,
                );
                
                self.particles.push(Particle::new(
                    Pos2::new(x, y),
                    velocity,
                    self.theme.energy_spark,
                    SPARK_SIZE,
                ));
            }
        }
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        let painter = ui.painter();

        for particle in &self.particles {
            let age = particle.created.elapsed().as_secs_f32();
            let alpha = 1.0 - (age / particle.lifetime);
            
            let color = Color32::from_rgba_premultiplied(
                particle.color.r(),
                particle.color.g(),
                particle.color.b(),
                (particle.color.a() as f32 * alpha) as u8,
            );

            if particle.size == RAIN_CHAR_SIZE {
                // Draw digital rain character
                let glyph = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F']
                    [((age * 10.0) as usize) % 16];
                
                painter.text(
                    particle.pos,
                    egui::Align2::CENTER_CENTER,
                    glyph.to_string(),
                    egui::FontId::monospace(particle.size),
                    color,
                );
            } else {
                // Draw energy spark
                painter.circle_filled(
                    particle.pos,
                    particle.size,
                    color,
                );
            }
        }
    }
} 