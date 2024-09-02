use macroquad::{prelude::*, rand::gen_range};
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

impl Default for Dot {
    fn default() -> Self {
        Dot {
            x: gen_range(0.0, 1920.0),
            y: gen_range(0.0, 1080.0),
            r: gen_range(0, 255),
            g: gen_range(0, 255),
            b: gen_range(0, 255),
            a: gen_range(0, 255),
            size: rand::gen_range(0.0, 300.0),
            id: "0".to_owned(),
        }
    }
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
