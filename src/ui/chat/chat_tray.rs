//Chat tray aligh TOP RIGHT
use crate::networking::ws::WsClient;
use crate::state::canvas::Canvas;
use crate::ui::ui_driver::GuiModule;
use async_trait::async_trait;
use egui::{Align, Align2};
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};
use nanoserde::{DeJson, SerJson};

//WIP
#[allow(dead_code)]
pub struct ChatTray {
    chats: Vec<Chat>,
    limit: usize,
    current_entry: String,
    cooldown : i32
}

impl Default for ChatTray {
    fn default() -> Self {
        ChatTray {
            chats: Vec::new(),
            limit: 10,
            current_entry: String::new(),
            cooldown : 600
        }
    }
}

#[derive(Clone, SerJson, DeJson, Debug)]
pub struct Chat {
    pub message: String,
    pub user: String,
}

#[async_trait]
impl GuiModule for ChatTray {
    fn render(&mut self, egui_ctx: &egui::Context, _canvas: &mut Canvas, wsc: &mut WsClient) {
        egui::Window::new(RichText::new("Live Chat"))
            //.to_owned() + &storage.get("socket").unwrap()).size(14.0).strong()
            .anchor(Align2::RIGHT_TOP, (-250.0, 10.0))
            .resizable(false)
            .movable(false)
            .default_open(false)
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::WHITE)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::WHITE)),
            )
            .show(egui_ctx, |ui| {
                //Handle current chats
                ui.vertical(|ui| {
                    for chat in self.chats.iter_mut() {
                        chat.chat_module(ui);
                    }
                });

                //Handle entry func
                ui.vertical(|ui| {
                    ui.add(super::entry::chat_entryfield(
                        &mut self.current_entry,
                        &mut wsc.chats_out,
                    ));
                });
            });
        self.check_size();
    }

    //Handle chat out
    async fn handle_ws(&mut self, wsc: &mut WsClient) {
        if !wsc.chats_inc.is_empty() {
            self.chats.extend(wsc.chats_inc.clone());
            wsc.chats_inc.clear();
        }

        if !wsc.chats_out.is_empty() {
            wsc.gui_chat(&wsc.chats_out).await.unwrap();
            self.chats.extend(wsc.chats_out.clone());
            wsc.chats_out.clear();
        }
    }
}

impl ChatTray {
    fn check_size(&mut self) {
        if self.chats.len() > self.limit {
            self.chats.reverse();
            self.chats.pop();
            self.chats.reverse();
        }
    }
}

impl Chat {
    //Render out chats with uuid : message format
    //Single bar
    pub fn chat_module(&mut self, ui: &mut egui_macroquad::egui::Ui) {
        ui.with_layout(
            egui_macroquad::egui::Layout::left_to_right(Align::TOP),
            |ui| {
                ui.add_space(5.0);
                ui.label(&self.user);
                ui.label(&self.message);
                ui.add_space(5.0);
            },
        );
    }
}
