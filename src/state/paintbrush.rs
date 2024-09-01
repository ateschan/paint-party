use macroquad::prelude::*;
use macroquad_particles::Emitter;

pub struct PaintBrush {
    pub eraser_rot: f32,
    //pub emitter: Emitter,
    pub emitters: Vec<(Emitter, Vec2)>,
}

impl Default for PaintBrush {
    fn default() -> Self {
        PaintBrush {
            eraser_rot: 0.0,
            emitters: Vec::new(), //emitter: Emitter::new(EmitterConfig { ..explosion() }),
        }
    }
}

impl PaintBrush {
    pub fn render_paintbrush(&self, storage: &mut quad_storage::LocalStorage) {
        draw_circle(
            mouse_position().0,
            mouse_position().1,
            storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
            macroquad::color::Color::from_rgba(
                storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
            ),
        );
    }

    pub fn render_eraser(&self, storage: &mut quad_storage::LocalStorage) {
        draw_poly_lines(
            mouse_position().0,
            mouse_position().1,
            10,
            storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
            self.eraser_rot,
            5.0,
            macroquad::color::Color::from_rgba(
                storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
            ),
        );
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
