use macroquad::prelude::*;

pub struct Brush {
    pub r : f32,
    pub g : f32,
    pub b : f32
}
impl Brush {
    pub fn swapcolor(&self, new_color : [f32; 3]) -> Self {
        Self {
            r : new_color[0],
            g : new_color[1],
            b : new_color[2]
        }
    }
}

pub struct Dot {
    pub x : f32,
    pub y : f32,
    pub color : Color,
}
