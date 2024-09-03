use crate::networking::networking_io::remove;
use crate::state::dot::Dot;
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

impl super::super::canvas::Canvas {
    pub async fn eraser(&mut self, socket: &mut WebSocket) {
        self.brush.render_eraser();

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
            self.brush.eraser_update(5.0);
        } else {
            let comp: Vec<String> = Vec::new();
            if self.garbage != comp {
                remove(socket, &self.user, &self.garbage).await.unwrap();
                self.garbage.clear();
            }
        }
        self.brush.eraser_update(1.0);
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
