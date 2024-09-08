use macroquad::prelude::*;

//No behavior becuase canvas is mutated direclty, uses paintbrush for behavior
//SELF AS CURSOR
impl super::super::brush::Brush {
    pub fn render_size_oscillator(&self) {
        draw_circle(
            mouse_position().0,
            mouse_position().1,
            self.size,
            macroquad::color::Color::from_rgba(self.r, self.g, self.b, self.a),
        );
    }
}
