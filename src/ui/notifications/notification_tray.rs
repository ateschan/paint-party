use crate::state::brush::BrushState::*;
use crate::ui::ui_driver::Render;
use crate::{state::canvas::Canvas, ui::toolbar::password::password};
use egui::Align2;
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};

#[derive(Default)]
pub struct NotificationTray;

impl Render for NotificationTray {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas) {
        egui::Window::new(RichText::new("Notifications"))
            //.to_owned() + &storage.get("socket").unwrap()).size(14.0).strong()
            .resizable(false)
            .anchor(Align2::CENTER_TOP, (0.0, 0.0))
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::TRANSPARENT)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::TRANSPARENT)),
            )
            .show(egui_ctx, |ui| {
                egui_ctx.set_visuals(egui::Visuals::light());
            });
    }
}
