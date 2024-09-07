//Chat tray aligh TOP RIGHT
use crate::networking::ws::WsClient;
use crate::state::canvas::Canvas;
use crate::ui::ui_driver::GuiModule;
use async_trait::async_trait;
use egui::{Align, Align2, Rounding};
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};
use nanoserde::{DeJson, SerJson};

//WIP
#[allow(dead_code)]
pub struct ChatTray {
    chats: Vec<Chat>,
    limit: usize,
    current_entry: String,
    cooldown: i32,
}

impl Default for ChatTray {
    fn default() -> Self {
        ChatTray {
            chats: Vec::new(),
            limit: 10,
            current_entry: String::new(),
            cooldown: 600,
        }
    }
}

#[derive(Clone, SerJson, DeJson, Debug)]
pub struct Chat {
    pub message: String,
    pub user: String,
    pub color: (u8, u8, u8),
}

#[async_trait]
impl GuiModule for ChatTray {
    fn render(&mut self, egui_ctx: &egui::Context, _canvas: &mut Canvas, wsc: &mut WsClient) {
        egui::Window::new(RichText::new("Live Chat"))
            //.to_owned() + &storage.get("socket").unwrap()).size(14.0).strong()
            .anchor(Align2::CENTER_TOP, (0.0,0.0))
            .resizable(false)
            .movable(false)
            .default_open(false)
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::TRANSPARENT)
                    .rounding(Rounding::same(10.0))
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::TRANSPARENT)),
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

        //Implement queuing for internet lag
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
        if self.user.len() > 5 {
            ui.with_layout(
                egui_macroquad::egui::Layout::left_to_right(Align::TOP),
                |ui| {
                    ui.add_space(5.0);
                    ui.label(
                        RichText::new(&self.user[&self.user.len() - 5..]).background_color(
                            Color32::from_rgb(self.color.0, self.color.1, self.color.2),
                        ).strong().text_style(egui::TextStyle::Button),
                    );

                    ui.label(RichText::new(&self.message).background_color(Color32::LIGHT_GRAY));
                    ui.add_space(5.0);
                },
            );
        } else {
            ui.with_layout(
                egui_macroquad::egui::Layout::left_to_right(Align::TOP),
                |ui| {
                    ui.add_space(5.0);
                    ui.label(
                        RichText::new(&self.user).background_color(Color32::from_rgb(
                            self.color.0,
                            self.color.1,
                            self.color.2,
                        )),
                    );
                    ui.label(RichText::new(&self.message).background_color(Color32::LIGHT_GRAY));
                    ui.add_space(5.0);
                },
            );
        }
    }
}
