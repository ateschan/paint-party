use macroquad::prelude::*;
use nanoserde::{DeJson, SerJson};

#[derive(Clone, Debug, SerJson, DeJson, PartialEq)]
pub struct Dot {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub size: f32,
    pub id: String,
}

impl Dot {
    pub fn render(&self) {
        draw_circle(
            self.x,
            self.y,
            self.size,
            macroquad::color::Color::from_rgba(self.r, self.g, self.b, self.a),
        );
    }

    pub fn get_unique_dots(&self, dots: Vec<Dot>) -> Vec<Dot> {
        let mut unique_dots: Vec<Dot> = Vec::new();
        for dot in dots.iter() {
            if !unique_dots.contains(dot) {
                unique_dots.push(dot.clone());
            }
        }
        unique_dots
    }
}
