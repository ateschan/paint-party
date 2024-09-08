use crate::networking::ws::WsClient;
use crate::state::dot::Dot;
use macroquad::prelude::*;

impl super::super::canvas::Canvas {
    pub async fn eraser(&mut self, socket: &mut WsClient) {


        if is_mouse_button_down(MouseButton::Left)
            && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
            && !self.brush.hamper_self
        {
            let dot = Dot {
                x: mouse_position().0,
                y: mouse_position().1,
                r: self.brush.r,
                g: self.brush.g,
                b: self.brush.b,
                a: self.brush.a,
                size: self.brush.size,
                ..Dot::default()
            };

            self.garbage.extend(self.is_overlapping(&dot));
            if !self.garbage.is_empty() {
                self.lines.retain(|dot| {
                    if !self.garbage.contains(&dot.id) {
                        true
                    } else {
                        // Unimplemnted emitter config
                        // self.brush.spawn_emitter(
                        //     Emitter::new(EmitterConfig {
                        //         size: dot.size,
                        //         colors_curve: ColorCurve {
                        //             start: macroquad::color::Color::from_rgba(
                        //                 dot.r, dot.g, dot.b, dot.a,
                        //             ),
                        //             mid: macroquad::color::Color::from_rgba(
                        //                 dot.r, dot.g, dot.b, dot.a,
                        //             ),
                        //             end: macroquad::color::Color::from_rgba(
                        //                 dot.r, dot.g, dot.b, dot.a,
                        //             ),
                        //         },
                        //         ..explosion()
                        //     }),
                        //     Vec2 { x: dot.x, y: dot.y },
                        // );
                        false
                    }
                });
            }
            // Counter at 99 for chunking
            if self.garbage.len() >= 99 {
                self.clear_and_del(socket).await;
            }
            self.brush.eraser_update(5.0);
        } else {
            self.clear_and_del(socket).await;
        }
    }
    pub fn remove_dots_by_id(&mut self, ids_to_remove: &[String]) {
        self.lines.retain(|dot| !ids_to_remove.contains(&dot.id));
    }

    fn is_overlapping(&self, circle1: &Dot) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for dot in self.lines.iter() {
            let distance_squared = (circle1.x - dot.x).powi(2) + (circle1.y - dot.y).powi(2);
            if distance_squared <= (circle1.size + dot.size).powi(2) {
                res.push(dot.id.to_owned());
            }
        }
        res
    }

    async fn clear_and_del(&mut self, wsc: &mut WsClient) {
        if !self.garbage.is_empty() {
            wsc.canvas_remove(&self.garbage).await.unwrap();
            self.garbage.clear();
        }
    }
}

impl super::super::brush::Brush {
    pub fn render_eraser(&self) {
        draw_poly_lines(
            mouse_position().0,
            mouse_position().1,
            10,
            self.size,
            self.eraser_rot,
            5.0,
            macroquad::color::Color::from_rgba(self.r, self.g, self.b, self.a),
        );
    }
}
