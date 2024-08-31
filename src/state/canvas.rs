use super::paintbrush::PaintBrush;
use crate::networking::networking::remove;
use crate::state::dot::Dot;
use crate::LocalStorage;
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

#[derive(Clone)]
pub struct Canvas {
    pub lines: Vec<Dot>,
    pub cache: Vec<Dot>,
    pub garbage: Vec<String>,
    pub frame_count: i32,
    pub brush: PaintBrush,
}

impl Canvas {
    pub fn render_paint(&self) {
        for circle in self.cache.iter() {
            circle.render();
        }
        for circle in self.lines.iter() {
            circle.render();
        }
    }

    pub async fn brush_handler(&mut self, storage: &mut LocalStorage, socket: &mut WebSocket) {
        match storage.get("brush_state").unwrap().as_str() {
            "On" => {
                self.brush.render_paintbrush(storage);
                if is_mouse_button_down(MouseButton::Left)
                    && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
                    && storage
                        .get("brush_hamper")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap()
                {
                    // one_shot_emitter.config.emitting = true;
                    let dot = Dot {
                        x: mouse_position().0,
                        y: mouse_position().1,
                        r: storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                        g: storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                        b: storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                        a: storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                        size: storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
                        id: nanoid::nanoid!(),
                    };

                    self.cache.push(dot);
                }
            }
            "Off" => {}
            "Erase" => {
                self.brush.render_eraser(storage);

                if is_mouse_button_down(MouseButton::Left)
                    && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
                    && storage
                        .get("brush_hamper")
                        .unwrap()
                        .parse::<bool>()
                        .unwrap()
                {
                    let dot = Dot {
                        x: mouse_position().0,
                        y: mouse_position().1,
                        r: storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                        g: storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                        b: storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                        a: storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                        size: storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
                        id: "0".to_string(),
                    };

                    self.garbage.extend(is_overlapping(&dot, &self.lines));
                    self.lines.retain(|dot| !self.garbage.contains(&dot.id));
                    if self.brush.eraser_rot <= 360.0 {
                        self.brush.eraser_rot += 5.0;
                    } else {
                        self.brush.eraser_rot = 0.0;
                    }
                } else {
                    let comp: Vec<String> = Vec::new();
                    if self.garbage != comp {
                        remove(socket, storage, &self.garbage).await.unwrap();
                        self.garbage.clear();
                    }
                }
                if self.brush.eraser_rot <= 360.0 {
                    self.brush.eraser_rot += 1.0;
                } else {
                    self.brush.eraser_rot = 0.0;
                }
            }
            &_ => {
                println!("UNABLE TO PARSE {} ", storage.get("brush_state").unwrap())
            }
        }
    }
    pub fn init_state(&self, storage: &mut LocalStorage) {
        //Brush
        storage.set("brush_r", "255");
        storage.set("brush_g", "255");
        storage.set("brush_b", "255");
        storage.set("brush_a", "255");
        storage.set("brush_size", "15.0");

        //On Off Erase
        storage.set("brush_state", "On");
        storage.set("brush_hamper", "true");

        //Networking
        storage.set("room", "0");
        storage.set("apikey", "");
        storage.set("socket", "");

        //State flags
        storage.set("clear_local_flag", "false");
        storage.set("refresh_flag", "false");
        storage.set("intro_complete_flag", "false");
    }
}

pub fn is_overlapping(circle1: &Dot, circles: &[Dot]) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for circle2 in circles {
        let distance_squared = (circle1.x - circle2.x).powi(2) + (circle1.y - circle2.y).powi(2);
        if distance_squared <= (circle1.size + circle2.size).powi(2) {
            res.push(circle2.id.to_owned());
        }
    }
    res
}
