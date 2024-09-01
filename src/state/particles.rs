use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig, BlendMode, Curve};

pub fn paint_seep() -> particles::EmitterConfig {
    particles::EmitterConfig {
        shape: macroquad_particles::ParticleShape::Circle { subdivisions: (20) },
        one_shot: true,
        emitting: true,
        lifetime: 3.0,
        lifetime_randomness: 0.8,
        explosiveness: 0.96,
        amount: 5,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 16.0,
        gravity: vec2(0.0, 30.0),
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        blend_mode: BlendMode::Alpha,
        local_coords: false,
        ..Default::default()
    }
}

pub fn explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        shape: macroquad_particles::ParticleShape::Circle { subdivisions: (20) },
        one_shot: true,
        emitting: true,
        size_randomness: 0.6,
        lifetime: 900.0,
        lifetime_randomness: 0.8,

        explosiveness : 0.999,
        initial_velocity: 150.0,
        amount: 2,
        initial_direction_spread: 1.5* std::f32::consts::PI,
        gravity: vec2(0.0, 200.0),
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        blend_mode: BlendMode::Alpha,
        local_coords: false,
        ..Default::default()
    }
}
