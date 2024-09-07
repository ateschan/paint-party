use crate::ui::chat::chat_tray::Chat;
use egui::TextEdit;

pub fn chat_entryfield<'a>(
    entry: &'a mut String,
    chats: &'a mut Vec<Chat>,
) -> impl egui_macroquad::egui::Widget + 'a {
    move |ui: &mut egui_macroquad::egui::Ui| chat_entryfield_ui(ui, entry, chats)
}

//Bar module for entry bar on bottom
pub fn chat_entryfield_ui(
    ui: &mut egui_macroquad::egui::Ui,
    entry: &mut String,
    chats: &mut Vec<Chat>,
) -> egui_macroquad::egui::Response {
    let result = ui.with_layout(
        egui_macroquad::egui::Layout::right_to_left(egui::Align::LEFT),
        |ui| {
            if ui
                .add(egui_macroquad::egui::Button::new("Send"))
                .on_hover_text("Send chat")
                .clicked() && !entry.is_empty()
            {
                chats.push(Chat {
                    user: "Self".to_owned(),
                    message: entry.clone(),
                    color: (255, 144, 144),
                });
                entry.clear();
                //crate::networking::networking_io::chat(socket, user, msg)
            }
            ui.add_space(5.0);
            ui.add(TextEdit::singleline(entry).desired_width(300.0).char_limit(60).hint_text("ENTER CHAT"))
                .highlight();
            ui.add_space(5.0);
        },
    );
    result.response
}
