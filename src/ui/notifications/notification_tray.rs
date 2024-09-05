use crate::networking::ws::WsClient;
use crate::state::canvas::Canvas;
use crate::ui::ui_driver::GuiModule;
use async_trait::async_trait;
use egui::Align2;
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};

#[derive(Clone, Debug)]
pub enum NotificationFlag {
    GetSuccess,
    PutSuccess,
    DelSuccess,
    UpdSuccess,
    ClrSuccess,
    RmvSuccess,
    InvApi,
    Fail(String),
}

#[derive(Clone)]
pub struct NotificationTray {
    pub current_notifications: Vec<NotificationFlag>,
    limit: usize,
}

impl Default for NotificationTray {
    fn default() -> Self {
        NotificationTray {
            current_notifications: Vec::new(),
            limit: 5,
        }
    }
}

#[async_trait]
impl GuiModule for NotificationTray {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas,  wsc : &mut WsClient) {
        egui::Window::new(RichText::new("Notifications"))
            //.to_owned() + &storage.get("socket").unwrap()).size(14.0).strong()
            .anchor(Align2::CENTER_TOP, (0.0, 10.0))
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
                egui_ctx.set_visuals(egui::Visuals::light());
                ui.vertical(|ui| {
                    if !canvas.notification_flags.is_empty() {
                        self.current_notifications
                            .extend(canvas.notification_flags.clone());
                        canvas.notification_flags.clear();
                    }
                    for not in self.current_notifications.clone().iter_mut() {
                        self.notificaton_module(ui, not);
                    }
                });
            });
        self.check_size()
    }
    async fn handle_ws(&mut self, wsc: &mut WsClient) {
        //No use for notiication out
    }
}

impl NotificationTray {
    fn notificaton_module(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        not: &mut NotificationFlag,
    ) {
        match not {
            NotificationFlag::GetSuccess => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Get recieved!",
            )),
            NotificationFlag::PutSuccess => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Put recieved!",
            )),
            NotificationFlag::UpdSuccess => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Upd recieved!",
            )),
            NotificationFlag::ClrSuccess => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Clr recieved!",
            )),
            NotificationFlag::RmvSuccess => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Rmv recieved!",
            )),
            NotificationFlag::DelSuccess => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Del recieved!",
            )),
            NotificationFlag::InvApi => ui.add(egui_macroquad::egui::TextEdit::singleline(
                &mut "Inavlid Credentials",
            )),
            NotificationFlag::Fail(e) => ui.add_sized(
                ui.available_size(),
                egui_macroquad::egui::TextEdit::singleline(&mut format!(
                    "Failiure recieved: {}",
                    e
                )),
            ),
        };
    }

    fn check_size(&mut self) {
        if self.current_notifications.len() > self.limit {
            self.current_notifications.reverse();
            self.current_notifications.pop();
            self.current_notifications.reverse();
        }
    }
}
