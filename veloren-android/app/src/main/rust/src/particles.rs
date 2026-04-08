//! Particle system for visual effects

use vek::Vec3;
use rand::Rng;

/// Particle properties
#[derive(Clone, Copy)]
pub struct Particle {
    pub position: Vec3<f32>,
    pub velocity: Vec3<f32>,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub color: [f32; 4], // RGBA
    pub active: bool,
}

/// Particle emitter
pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    /// Emit particles at position
    pub fn emit(&mut self, position: Vec3<f32>, count: usize, config: &ParticleConfig) {
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            if self.particles.len() >= self.max_particles {
                // Remove oldest inactive particle
                if let Some(pos) = self.particles.iter().position(|p| !p.active) {
                    self.particles.remove(pos);
                } else {
                    break; // No space
                }
            }

            // Random velocity
            let velocity = Vec3::new(
                rng.gen_range(-config.speed..=config.speed),
                rng.gen_range(-config.speed..=config.speed) + config.upward_bias,
                rng.gen_range(-config.speed..=config.speed),
            );

            let particle = Particle {
                position,
                velocity,
                lifetime: config.lifetime,
                max_lifetime: config.lifetime,
                size: config.size,
                color: config.color,
                active: true,
            };

            self.particles.push(particle);
        }
    }

    /// Update all particles
    pub fn update(&mut self, delta_time: f32, gravity: f32) {
        for particle in &mut self.particles {
            if !particle.active {
                continue;
            }

            // Update lifetime
            particle.lifetime -= delta_time;
            if particle.lifetime <= 0.0 {
                particle.active = false;
                continue;
            }

            // Apply gravity
            particle.velocity.y += gravity * delta_time;

            // Update position
            particle.position += particle.velocity * delta_time;

            // Shrink over time
            let life_ratio = particle.lifetime / particle.max_lifetime;
            particle.size *= 0.99; // Slowly shrink
        }

        // Remove inactive particles
        self.particles.retain(|p| p.active);
    }

    /// Get active particles for rendering
    pub fn get_active_particles(&self) -> &[Particle] {
        &self.particles
    }

    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    /// Get particle count
    pub fn count(&self) -> usize {
        self.particles.len()
    }
}

/// Particle configuration
pub struct ParticleConfig {
    pub speed: f32,
    pub upward_bias: f32,
    pub lifetime: f32,
    pub size: f32,
    pub color: [f32; 4],
}

impl ParticleConfig {
    /// Dust particle config
    pub fn dust() -> Self {
        Self {
            speed: 2.0,
            upward_bias: 1.0,
            lifetime: 1.5,
            size: 0.3,
            color: [0.8, 0.7, 0.5, 0.6],
        }
    }

    /// Fire particle config
    pub fn fire() -> Self {
        Self {
            speed: 3.0,
            upward_bias: 5.0,
            lifetime: 0.8,
            size: 0.5,
            color: [1.0, 0.6, 0.1, 0.8],
        }
    }

    /// Smoke particle config
    pub fn smoke() -> Self {
        Self {
            speed: 1.5,
            upward_bias: 3.0,
            lifetime: 2.0,
            size: 0.8,
            color: [0.3, 0.3, 0.3, 0.5],
        }
    }

    /// Spark particle config
    pub fn spark() -> Self {
        Self {
            speed: 8.0,
            upward_bias: 2.0,
            lifetime: 0.5,
            size: 0.2,
            color: [1.0, 1.0, 0.5, 1.0],
        }
    }

    /// Magic particle config
    pub fn magic() -> Self {
        Self {
            speed: 4.0,
            upward_bias: 0.0,
            lifetime: 1.2,
            size: 0.6,
            color: [0.5, 0.8, 1.0, 0.7],
        }
    }
}
