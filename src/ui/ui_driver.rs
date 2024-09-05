use crate::{networking::ws::WsClient, state::canvas::Canvas};
use async_trait::async_trait;

#[allow(unused_imports)]
use super::{
    chat::chat_tray::ChatTray, notifications::notification_tray::NotificationTray,
    toolbar::toolbar_tray::ToolbarTray,
};

#[async_trait]
pub trait GuiModule {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas, wsc: &mut WsClient);
    async fn handle_ws(&mut self, wsc: &mut WsClient);
}

//Not just rendering, but calling all nessesary functions
#[allow(clippy::ptr_arg)]
pub async fn render_gui(
    modules: &mut Vec<Box<dyn GuiModule>>,
    canvas: &mut Canvas,
    wsc: &mut WsClient,
) {
    egui_macroquad::ui(|egui_ctx| {
        for item in modules.iter_mut() {
            item.render(egui_ctx, canvas, wsc);
        }
    });
    for item in modules.iter_mut() {
        item.handle_ws(wsc).await;
    }
    //Call handlers individually fucker
    egui_macroquad::draw();
}

pub fn tray_builder() -> Vec<Box<dyn GuiModule>> {
    let modules: Vec<Box<dyn GuiModule>> = vec![
        Box::new(ChatTray::default()),
        Box::new(ToolbarTray::default()),
        Box::new(NotificationTray::default()),
    ];
    modules
}
