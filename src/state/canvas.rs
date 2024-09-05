use super::{brush::Brush, /* particles::explosion */};
use crate::state::brush::BrushState::{Eraser, Off, Paintbrush};
use crate::state::dot::Dot;
use crate::ui::chat::chat_tray::Chat;
use crate::ui::notifications::notification_tray::NotificationFlag;
use macroquad::input::KeyCode;
use macroquad::prelude::*;
use quad_storage::LocalStorage;
use crate::networking::ws::WsClient;
//Order of inheritence for drawing goes Canvas -> Brush -> Dot
//
//Canvas serves as the interface for screeen state, and is used by UI and the websocket client
//
//Canvas sits between the websockets and the UI acting as a middleman

#[derive(Default)]
pub struct Canvas {
    pub lines: Vec<Dot>,
    pub cache: Vec<Dot>,
    pub garbage: Vec<String>,
    pub frame_count: i32,
    pub brush: Brush,

    //Why does this exist? Canvas -> UI
    //Canvas -> Websocket Handler

    //  BUG: |||||||||||||||||||||||||||||||||||||||||||||||||\
    //  What I need
    //  Ui is able to change canvas settings Canvas -> Ui
    //  check for changes to ui WSHANDLER CLASS -> Ui
    //  check for changes to canvas Canvas-> WShandler class
    //
    //  Ws class will have functions and will be passed into ws handler functions in gui and in
    //  canvas
    //  BUG: |||||||||||||||||||||||||||||||||||||||||||||||||\

    //WS glue

    //Ui glue
    pub inc_chats: Vec<Chat>,
    pub out_chats: Vec<Chat>,
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

    //Definitions in ./tools/
    //Entry point for user input
    pub async fn brush_handler(&mut self, wsc : &mut WsClient) {
        self.hotkey_handler().await;

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
