use egui::TextEdit;
use quad_storage::LocalStorage;
pub fn socket_ui(
    ui: &mut egui_macroquad::egui::Ui,
    socket: &mut String,
    storage: &mut LocalStorage,
) -> egui_macroquad::egui::Response {
    let result = ui.with_layout(
        egui_macroquad::egui::Layout::right_to_left(egui_macroquad::egui::Align::LEFT),
        |ui| {
            if ui
                .add(egui_macroquad::egui::Button::new("connect"))
                .on_hover_text("Connect to server")
                .clicked()
            {
                storage.set("intro_complete_flag", "true");
            }
            // Show the socket field:
            ui.add(TextEdit::singleline(socket).hint_text("Server Address"))
                .highlight();
        },
    );

    result.response
}

pub fn socket<'a>(
    socket: &'a mut String,
    storage: &'a mut LocalStorage,
) -> impl egui_macroquad::egui::Widget + 'a {
    move |ui: &mut egui_macroquad::egui::Ui| socket_ui(ui, socket, storage)
}
