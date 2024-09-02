use macroquad::prelude::*;
use crate::state::brush::BrushState::Paint;
use macroquad_particles::Emitter;

    pub enum BrushState {
        Off,
        Paint,
        Erase
    }

//COLOR SIZE
pub struct Brush {
    pub eraser_rot: f32,
    //pub emitter: Emitter,
    pub emitters: Vec<(Emitter, Vec2)>,
    pub size : f32,
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8,
    pub hamper_self : bool,
    pub hamper_particles : bool,
    pub state : BrushState
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            eraser_rot: 0.0,
            emitters: Vec::new(), //emitter: Emitter::new(EmitterConfig { ..explosion() }),
            size : 0.0,
            r : 0,
            g : 0,
            b : 0,
            a : 255,
            hamper_self : false,
            hamper_particles : false,
            state : Paint
        }
    }
}

impl Brush {
    

    pub fn render_paintbrush(&self) {
        draw_circle(
            mouse_position().0,
            mouse_position().1,
            self.size,
            macroquad::color::Color::from_rgba(
                self.r, self.g, self.b, self.a
            ),
        );
    }

    pub fn render_eraser(&self) {
        draw_poly_lines(
            mouse_position().0,
            mouse_position().1,
            10,
            self.size,
            self.eraser_rot,
            5.0,
            macroquad::color::Color::from_rgba(
                self.r, self.g, self.b, self.a
            ),
        );
    }

    pub fn eraser_update(&mut self, num : f32) {
        if self.eraser_rot <= 360.0 {
            self.eraser_rot += num;
        }
        else {
            self.eraser_rot = 0.0;
        }
    }

    pub fn spawn_emitter(&mut self, emitter: Emitter, Vec2 { x, y }: Vec2) {
        self.emitters.push((emitter, vec2(x, y)));
    }

    pub fn render_emitters(&mut self) {
        for emitter in self.emitters.iter_mut() {
            emitter.0.draw(emitter.1);
        }
        self.emitters.retain(|(emitter, _)| emitter.config.emitting);
    }
}
