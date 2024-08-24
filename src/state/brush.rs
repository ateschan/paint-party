use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub struct Brush {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a : u8,
    pub size: f32,
    pub sw: bool,
    pub room: i32,
    pub ip : String,
    pub apikey : String,
    pub frame_counter : i32
}
impl Brush {
    pub fn swapcolor(&self, new_color: [u8; 4]) -> Self {
        Self {
            r: new_color[0],
            g: new_color[1],
            b: new_color[2],
            a: new_color[3],
            size: self.size,
            sw: self.sw,
            room: self.room,
            ip : self.ip.clone(),
            apikey : self.apikey.clone(),
            frame_counter : self.frame_counter
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Dot {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub size: f32,
}
