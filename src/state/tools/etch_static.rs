use macroquad::prelude::*;

//SELF AS CURSOR
impl super::super::brush::Brush {
    pub fn render_etch(&self) {
        if !self.hamper_self {
            draw_line(
                self.pos.0 - 30.0,
                self.pos.1 + 0.0,
                self.pos.0 + 30.0,
                self.pos.1 - 0.0,
                1.0,
                macroquad::color::Color::from_rgba(
                    255 - self.r,
                    255 - self.g,
                    255 - self.b,
                    self.a,
                ),
            );
            draw_line(
                self.pos.0 - 0.0,
                self.pos.1 + 30.0,
                self.pos.0 + 0.0,
                self.pos.1 - 30.0,
                1.0,
                macroquad::color::Color::from_rgba(
                    255 - self.r,
                    255 - self.g,
                    255 - self.b,
                    self.a,
                ),
            );
        }
    }
}
