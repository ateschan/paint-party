use crate::state::canvas::Canvas;

use super::{
    chat::chat_tray::ChatTray, notifications::notification_tray::NotificationTray,
    toolbar::toolbar_tray::ToolbarTray,
};

pub trait Render {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas);
}

#[allow(clippy::ptr_arg)]
pub async fn render_gui(modules: &mut Vec<Box<dyn Render>>, canvas: &mut Canvas) {
    egui_macroquad::ui(|egui_ctx| {
        for item in modules.iter_mut() {
            item.render(egui_ctx, canvas);
        }
    });
    egui_macroquad::draw();
}

pub fn tray_builder() -> Vec<Box<dyn Render>> {
    let modules: Vec<Box<dyn Render>> = vec![
        Box::new(ChatTray::default()),
        Box::new(ToolbarTray::default()),
        Box::new(NotificationTray::default()),
    ];
    modules
}
