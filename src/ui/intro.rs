use egui_macroquad::egui::{self, Align2, TextEdit};
use egui_macroquad::egui::{epaint::Shadow, Color32, RichText};
use quad_storage::LocalStorage;

pub async fn render_intro(storage: &mut LocalStorage) {
    let mut tmp_socket = storage.get("socket").unwrap();
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new(RichText::new("PAINT PARTY").size(60.0).strong())
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::DARK_RED)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::LIGHT_GRAY)),
            )
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(TextEdit::singleline(&mut tmp_socket));

                    if ui
                        .add(egui_macroquad::egui::Button::new("→"))
                        .on_hover_text("Connect to server")
                        .clicked()
                    {
                        storage.set("intro_complete", "true");
                    }
                });
            });
    });

    storage.set("socket", &tmp_socket);
    egui_macroquad::draw();
}
