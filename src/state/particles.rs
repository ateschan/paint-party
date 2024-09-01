use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig, BlendMode};

pub fn paint_seep() -> particles::EmitterConfig {
    particles::EmitterConfig {
        shape: macroquad_particles::ParticleShape::Circle { subdivisions: (20) },
        one_shot: true,
        emitting: true,
        lifetime: 3.0,
        lifetime_randomness: 0.8,
        explosiveness: 0.70,
        amount: 2,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 6.0,
        gravity: vec2(0.0, 10.0),
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        blend_mode: BlendMode::Alpha,
        local_coords: false,
        ..Default::default()
    }
}
