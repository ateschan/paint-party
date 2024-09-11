use crate::state::{dot::Dot, particles::*};
use macroquad::prelude::*;
use macroquad_particles::*;

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

            //BUG: Memory Leak with spawning emitters using oscillator and modulator tools
            if !self.brush.hamper_particles
                && !self.brush.add_cmodulate
                && !self.brush.add_rev_mark
                && !self.brush.add_mark
                && !self.brush.add_size_osc
            {
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
        self.brush.pos_last = self.brush.pos;
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
