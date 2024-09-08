use super::toolbar_tray::ToolbarTray;
use crate::networking::ws::WsClient;

impl ToolbarTray {
    pub fn server_util(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        wsc: &mut WsClient,
    ) -> egui_macroquad::egui::Response {
        let result = ui.horizontal(|ui| {
            if ui
                .add(
                    egui_macroquad::egui::DragValue::new(&mut self.tmp_room)
                        .update_while_editing(false)
                        .speed(1.00)
                        .clamp_range(0.0..=9999.0),
                )
                .on_hover_text("Server room")
                .on_hover_cursor(egui::CursorIcon::Default)
                .lost_focus()
                && ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter))
            {
                wsc.user.room = self.tmp_room;
                self.refresh_flag = true;
            }
            if ui.button("CLEAR").on_hover_text("Erase All").clicked() {
                self.clear_flag = true;
            }
            if ui.button("↺").on_hover_text("Refresh").clicked() {
                self.refresh_flag = true;
            }

            //ui.add_sized(ui.available_size(), password(&mut self.tmp_pass));
        });
        result.response
    }
}
