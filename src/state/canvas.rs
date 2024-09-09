use super::{brush::Brush /* particles::explosion */};
use crate::networking::ws::WsClient;
use crate::state::brush::BrushState::*;
use crate::state::dot::Dot;
use macroquad::input::KeyCode;
use macroquad::prelude::*;
use quad_storage::LocalStorage;

#[derive(Default)]
pub struct Canvas {
    pub lines: Vec<Dot>,
    pub cache: Vec<Dot>,
    pub garbage: Vec<String>,
    pub frame_count: i32,
    pub brush: Brush,
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

    //Why not put functionalty in brush? Since all dots are being handled by canvas, so will the
    //brush behavior for interop with the outer enviroment.

    pub async fn brush_handler(&mut self, wsc: &mut WsClient) {
        self.hotkey_handler().await;

        //Cease used for pausing all brush rendering and input, independent from hamper_self which
        //is used for gui state
        if !self.brush.cease {
            match self.brush.state {
                Paintbrush => {
                    self.brush.render_paintbrush();
                    self.paintbrush().await;
                }

                Eraser => {
                    self.brush.render_eraser();
                    self.eraser(wsc).await;
                }

                Off => {}
            }
        }

        if self.brush.add_cmodulate {
            if self.brush.r_speed != 0 {
                self.r_modulate();
            }
            if self.brush.g_speed != 0 {
                self.g_modulate();
            }
            if self.brush.b_speed != 0 {
                self.b_modulate();
            }
            if self.brush.a_speed != 0 {
                self.a_modulate();
            }
        }

        if self.brush.add_size_osc {
            self.render_size_oscillator();
            self.size_oscillate();
        }

        if self.brush.add_mark {
            self.mark().await;
            self.render_size_oscillator();
        }

        if self.brush.add_rev_mark {
            self.rev_mark().await;
            self.render_size_oscillator();
        }
        
        self.brush.rotation_update(1.0);
    }

    pub async fn hotkey_handler(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.brush.state = Paintbrush;
        }
        if is_key_pressed(KeyCode::E) {
            self.brush.state = Eraser;
        }
        // if is_key_pressed(KeyCode::Left) && self.user.room > 0 && !self.refresh_flag {
        //     self.user.room -= 1;
        //     self.refresh_flag = true;
        // }
        // if is_key_pressed(KeyCode::Right) && self.user.room < 9999 && !self.refresh_flag{
        //     self.user.room += 1;
        //     self.refresh_flag = true;
        // }
    }

    pub fn init_state(&self, storage: &mut LocalStorage) {
        //Networking
        storage.set("socket", "");
        storage.set("apikey", "");

        //State flags
        storage.set("intro_complete_flag", "false");
    }
}
