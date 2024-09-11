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

    pub async fn brush_handler(&mut self, wsc: &mut WsClient) {
        self.hotkey_handler().await;

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



        if self.brush.active && !self.brush.mark_cease {
            match self.brush.state {
                Paintbrush => {
                    self.paintbrush().await;
                }

                Eraser => {
                    self.eraser(wsc).await;
                }

                Off => {}
            }
        }

        if !self.brush.mark_cease {
            match self.brush.state {
                Paintbrush => {
                    self.brush.render_paintbrush();
                }

                Eraser => {
                    self.brush.render_eraser();
                }

                Off => {}
            }

            if !self.brush.is_using_mouse {
                self.brush.render_etch();
            }
        }
    }

    pub async fn hotkey_handler(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.brush.state = Paintbrush;
        }
        if is_key_pressed(KeyCode::E) {
            self.brush.state = Eraser;
        }
        if is_key_down(KeyCode::LeftControl) {
            self.brush.active = true;
            self.brush.is_using_mouse = false;
            
            if is_key_down(KeyCode::Up) {
                self.brush.pos.1 -= 10.0;
            }
            if is_key_down(KeyCode::Down) {
                self.brush.pos.1 += 10.0;
            }
            if is_key_down(KeyCode::Left) {
                self.brush.pos.0 -= 10.0;
            }
            if is_key_down(KeyCode::Right) {
                self.brush.pos.0 += 10.0;
            }
        }
        else {
            self.brush.active = is_mouse_button_down(MouseButton::Left);
        }

        if !self.brush.is_using_mouse && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0) {
            self.brush.is_using_mouse = true;
        }

        if self.brush.is_using_mouse {
            self.brush.pos = mouse_position();
        }


        if is_key_released(KeyCode::LeftControl) || is_mouse_button_released(MouseButton::Left){
            if self.brush.add_mark {
                self.brush.size = self.brush.size_osc_minmax.1;
            }
            if self.brush.add_rev_mark {
                self.brush.size = self.brush.size_osc_minmax.0;
            }
            self.brush.mark_cease = false;
        }
    }

    pub fn calulate_delta_pos(&mut self) -> (f32, f32) {
        (self.brush.pos_last.0 - self.brush.pos.0, self.brush.pos_last.1 - self.brush.pos.1)
    }

    pub fn init_state(&self, storage: &mut LocalStorage) {
        //Networking
        storage.set("socket", "");
        storage.set("apikey", "");

        //State flags
        storage.set("intro_complete_flag", "false");
    }
}
