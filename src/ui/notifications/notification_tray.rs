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
    ChtSndSuccess,
    ChtRcvSuccess,
    RmvSuccess,
    InvApi, //Invalid api access attempt
    Fail(String),
}

#[derive(Clone)]
pub struct NotificationTray {
    pub current_notifications: Vec<NotificationFlag>,
    limit: usize,
    timer: usize,
}

impl Default for NotificationTray {
    fn default() -> Self {
        NotificationTray {
            current_notifications: Vec::new(),
            limit: 5,
            timer: 300,
        }
    }
}

//TODO: Refactor to remove canvas
#[async_trait]
impl GuiModule for NotificationTray {
    fn render(&mut self, egui_ctx: &egui::Context, _canvas: &mut Canvas, wsc: &mut WsClient) {
        egui::Window::new(RichText::new(" Notifications"))
            //.to_owned() + &storage.get("socket").unwrap()).size(14.0).strong()
            .collapsible(true)
            .anchor(Align2::RIGHT_TOP, (-8.0, 8.0))
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .default_open(true)
            .frame(
                egui::Frame::default()
                    .multiply_with_opacity(20.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::TRANSPARENT),
            )
            .show(egui_ctx, |ui| {
                egui_ctx.set_visuals(egui::Visuals::light());
                ui.horizontal(|ui| {
                    if !wsc.notification_flags.is_empty() {
                        self.current_notifications
                            .extend(wsc.notification_flags.clone());
                        wsc.notification_flags.clear();
                    }
                    for not in self.current_notifications.clone().iter_mut() {
                        self.notificaton_module(ui, not);
                    }
                });
            });
        self.check_size();
        self.timer -= 1;
    }
    async fn handle_ws(&mut self, _wsc: &mut WsClient) {
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
            NotificationFlag::GetSuccess => ui.label(
                RichText::new("Get recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::PutSuccess => ui.label(
                RichText::new("Put recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::UpdSuccess => ui.label(
                RichText::new("Upd recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::ClrSuccess => ui.label(
                RichText::new("Clr recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::RmvSuccess => ui.label(
                RichText::new("Rmv recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::DelSuccess => ui.label(
                RichText::new("Del recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::ChtSndSuccess => ui.label(
                RichText::new("Chat sent!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::ChtRcvSuccess => ui.label(
                RichText::new("Chat recieved!")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::TRANSPARENT),
            ),
            NotificationFlag::InvApi => ui.label(
                RichText::new("Inavlid Credentials")
                    .size(16.0)
                    .strong()
                    .background_color(Color32::LIGHT_RED),
            ),

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
        if self.current_notifications.len() > self.limit || self.timer == 0 {
            self.current_notifications.reverse();
            self.current_notifications.pop();
            self.current_notifications.reverse();
            self.timer = 300;
        }
    }
}
