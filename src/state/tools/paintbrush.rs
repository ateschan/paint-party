use crate::state::dot::Dot;
use macroquad::prelude::*;

//BEHAVIOR
impl super::super::canvas::Canvas {
    pub async fn paintbrush(&mut self) {
        if self.brush.active
            && self.calulate_delta_pos() != (0.0, 0.0)
            && !self.brush.hamper_self
            && self.brush.a != 0
            && !self.brush.mark_cease
        {
            let dot = Dot {
                x: self.brush.pos.0,
                y: self.brush.pos.1,
                r: self.brush.r,
                g: self.brush.g,
                b: self.brush.b,
                a: self.brush.a,
                size: self.brush.size,
                id: nanoid::nanoid!(),
            };

            self.cache.push(dot);
        }
    }
}

//SELF AS CURSOR
impl super::super::brush::Brush {
    pub fn render_paintbrush(&self) {
        if !self.hamper_self {
            draw_circle(
                self.pos.0,
                self.pos.1,
                self.size,
                macroquad::color::Color::from_rgba(self.r, self.g, self.b, self.a),
            );
        }
    }
}
