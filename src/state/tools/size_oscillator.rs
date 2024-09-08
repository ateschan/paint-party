use macroquad::prelude::*;

//No behavior becuase canvas is mutated direclty, uses paintbrush for behavior
//SELF AS CURSOR
impl super::super::brush::Brush {
    pub async fn mark(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.size = self.size_osc_minmax.1;
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.size = self.size_osc_minmax.0;
        }
    }

    pub async fn rev_mark(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.size = self.size_osc_minmax.0;
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.size = self.size_osc_minmax.1;
        }
    }

    pub fn render_size_oscillator(&self) {
        draw_circle_lines(
            mouse_position().0,
            mouse_position().1,
            self.size_osc_minmax.1,
            5.0,
            macroquad::color::Color::from_rgba(255 - self.r, 255 - self.g, 255 - self.b, self.a),
        );
        draw_circle(
            mouse_position().0,
            mouse_position().1,
            self.size_osc_minmax.0,
            macroquad::color::Color::from_rgba(255 - self.r, 255 - self.g, 255 - self.b, self.a),
        );
    }
}
