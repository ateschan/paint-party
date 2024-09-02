use crate::state::canvas::Canvas;

use super::{
    notifications::notification_tray::NotificationTray, toolbar::toolbar_tray::ToolbarTray,
};

pub trait Render {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas);
}

pub async fn render_gui(canvas: &mut Canvas) {
    egui_macroquad::ui(|egui_ctx| {
        let mut modules: Vec<Box<dyn Render>> = Vec::new();

        modules.push(Box::new(ToolbarTray::default()));
        modules.push(Box::new(NotificationTray::default()));

        for item in modules.iter_mut() {
            item.render(egui_ctx, canvas);
        }
    });
    egui_macroquad::draw();
}
