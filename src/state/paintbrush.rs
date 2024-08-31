use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PaintBrush {
    pub eraser_rot: f32,
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
}
