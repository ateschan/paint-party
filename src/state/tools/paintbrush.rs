use crate::state::{dot::Dot, particles::*};
use macroquad::prelude::*;
use macroquad_particles::*;

//BEHAVIOR
impl super::super::canvas::Canvas {
    pub async fn paintbrush(&mut self) {

        if is_mouse_button_down(MouseButton::Left)
            && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
            && !self.brush.hamper_self
            && self.brush.a != 0
        {
            let dot = Dot {
                x: mouse_position().0,
                y: mouse_position().1,
                r: self.brush.r,
                g: self.brush.g,
                b: self.brush.b,
                a: self.brush.a,
                size: self.brush.size,
                id: nanoid::nanoid!(),
            };

            if !self.brush.hamper_particles {
                self.brush.spawn_emitter(
                    Emitter::new(EmitterConfig {
                        size: dot.size,
                        colors_curve: ColorCurve {
                            start: macroquad::color::Color::from_rgba(dot.r, dot.g, dot.b, dot.a),
                            mid: macroquad::color::Color::from_rgba(dot.r, dot.g, dot.b, dot.a),
                            end: macroquad::color::Color::from_rgba(dot.r, dot.g, dot.b, dot.a),
                        },
                        ..paint_seep()
                    }),
                    Vec2 { x: dot.x, y: dot.y },
                );
            }
            self.cache.push(dot);
        }
    }
}

//SELF AS CURSOR
impl super::super::brush::Brush {
    pub fn render_paintbrush(&self) {
        draw_circle(
            mouse_position().0,
            mouse_position().1,
            self.size,
            macroquad::color::Color::from_rgba(self.r, self.g, self.b, self.a),
        );
    }
}
