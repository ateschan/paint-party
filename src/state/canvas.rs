use super::{brush::Brush, user::User /* particles::explosion */};
use crate::networking::networking_io::remove;
use crate::state::brush::BrushState::{Erase, Off, Paint};
use crate::state::dot::Dot;
use crate::state::particles::paint_seep;
use macroquad::prelude::*;
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};
use quad_net::web_socket::WebSocket;
use quad_storage::LocalStorage;

use crate::ui::notifications::notification_tray::NotificationFlag;

#[derive(Default)]
pub struct Canvas {
    pub lines: Vec<Dot>,
    pub cache: Vec<Dot>,
    pub garbage: Vec<String>,
    pub frame_count: i32,
    pub brush: Brush,
    pub user: User,

    pub refresh_flag: bool,
    pub clear_flag: bool,
    pub notification_flags: Vec<NotificationFlag>,
}

impl Canvas {
    pub fn render_paint(&mut self) {
        for dot in self.lines.iter() {
            dot.render();
        }
        for dot in self.cache.iter() {
            dot.render();
        }
        self.brush.render_emitters();
    }

    pub async fn brush_handler(&mut self, socket: &mut WebSocket) {
        match self.brush.state {
            Paint => {
                self.brush.render_paintbrush();

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
                        id: nanoid::nanoid!(),
                    };

                    if !self.brush.hamper_particles {
                        self.brush.spawn_emitter(
                            Emitter::new(EmitterConfig {
                                size: dot.size,
                                colors_curve: ColorCurve {
                                    start: macroquad::color::Color::from_rgba(
                                        dot.r, dot.g, dot.b, dot.a,
                                    ),
                                    mid: macroquad::color::Color::from_rgba(
                                        dot.r, dot.g, dot.b, dot.a,
                                    ),
                                    end: macroquad::color::Color::from_rgba(
                                        dot.r, dot.g, dot.b, dot.a,
                                    ),
                                },
                                ..paint_seep()
                            }),
                            Vec2 { x: dot.x, y: dot.y },
                        );
                    }
                    self.cache.push(dot);
                }
            }

            Erase => {
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

            Off => {}
        }
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

    pub fn remove_dots_by_id(&mut self, ids_to_remove: &[String]) {
        self.lines.retain(|dot| !ids_to_remove.contains(&dot.id));
    }

    pub fn init_state(&self, storage: &mut LocalStorage) {
        //Networking
        storage.set("socket", "");
        storage.set("apikey", "");

        //State flags
        storage.set("intro_complete_flag", "false");
    }
}
