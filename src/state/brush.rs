use macroquad::prelude::*;
use macroquad_particles::Emitter;
use rand::gen_range;
use BrushState::Paintbrush;

//Brush handles what lies behind the cursor, paint color, and particles
pub enum BrushState {
    Off,
    Paintbrush,
    Eraser,
}

//COLOR SIZE
pub struct Brush {
    pub emitters: Vec<(Emitter, Vec2)>,
    pub size: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub hamper_self: bool,
    pub hamper_particles: bool,
    pub state: BrushState,

    //SPECIAL
    pub eraser_rot: f32,
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            emitters: Vec::new(), //emitter: Emitter::new(EmitterConfig { ..explosion() }),
            size: gen_range(15.0, 300.0),
            r: gen_range(0, 255),
            g: gen_range(0, 255),
            b: gen_range(0, 255),
            a: gen_range(0, 255),
            hamper_self: false,
            hamper_particles: false,
            state: Paintbrush,
            eraser_rot: 0.0,
        }
    }
}

impl Brush {
    pub fn eraser_update(&mut self, num: f32) {
        if self.eraser_rot <= 360.0 {
            self.eraser_rot += num;
        } else {
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
