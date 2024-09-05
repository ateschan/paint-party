//Chat tray aligh TOP RIGHT
use crate::state::canvas::Canvas;
use crate::ui::ui_driver::Render;
use egui::Align2;
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};

//WIP
pub struct ChatTray {
    chats: Vec<Chat>,
    limit: usize,
}

impl Default for ChatTray {
    fn default() -> Self {
        ChatTray {
            chats: Vec::new(),
            limit: 10,
        }
    }
}

#[derive(Clone)]
pub struct Chat {
    message: String,
    user: String,
}

impl Render for ChatTray {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas) {
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
                if !canvas.chats.is_empty() {
                    self.chats.extend(canvas.chats.clone());
                    canvas.chats.clear();
                }
                ui.vertical(|ui| {
                    for chat in self.chats.iter_mut() {
                        chat.chat_module(ui);
                    }
                });

                ui.vertical(|ui| {
                    self.chat_entryfield(ui);
                });
            });
        self.check_size();
    }
}
impl ChatTray {
    //Should have single line with a send button
    //Optionally seperate into another file?
    pub fn chat_entryfield(&self, ui: &mut egui_macroquad::egui::Ui) {}

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
    pub fn chat_module(&self, ui: &mut egui_macroquad::egui::Ui) {}
}
