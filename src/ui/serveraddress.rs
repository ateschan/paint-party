use crate::BRUSH;
pub fn server_address_ui(ui: &mut egui_macroquad::egui::Ui, address : &mut String, room : &mut i32) -> egui_macroquad::egui::Response {

    let result = ui.with_layout(egui_macroquad::egui::Layout::right_to_left(egui_macroquad::egui::Align::Center), |ui| {
        let room_button = ui.add(
                egui_macroquad::egui::DragValue::new(room)
                .speed(0.01)
                .clamp_range(0.0..=9999.0),
            );
        unsafe{
            if room_button.lost_focus() && ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter)) {
                BRUSH.room = *room;
            }
        }

        ui.add_sized(
            ui.available_size(),
            egui_macroquad::egui::TextEdit::singleline(address)
        ).on_hover_text("IP/HOSTNAME : PORT");

    });

    result.response
}

pub fn server_address<'a>(address : &'a mut String, room : &'a mut i32) -> impl egui_macroquad::egui::Widget + 'a {
    move |ui: &mut egui_macroquad::egui::Ui| server_address_ui(ui, address, room)
}

