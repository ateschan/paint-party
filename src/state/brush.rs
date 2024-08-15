use macroquad::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

pub struct Brush  {
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub size : f32,
    pub sw : bool,
    pub room : i32
}
impl Brush {
    pub fn swapcolor(&self, new_color : [f32; 3]) -> Self {
        Self {
            r : new_color[0],
            g : new_color[1],
            b : new_color[2],
            size : self.size,
            sw : self.sw,
            room : self.room
        }
    }
}



#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dot {
    pub x : f32,
    pub y : f32,
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub size : f32
}
