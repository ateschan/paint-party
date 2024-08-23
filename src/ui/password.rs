pub fn password_ui(ui: &mut egui_macroquad::egui::Ui, password: &mut String) -> egui_macroquad::egui::Response {
    let state_id = ui.id().with("show_plaintext");

    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    let result = ui.with_layout(egui_macroquad::egui::Layout::right_to_left(egui_macroquad::egui::Align::Center), |ui| {
        let response = ui
            .add(egui_macroquad::egui::SelectableLabel::new(show_plaintext, "👁"))
            .on_hover_text("Show/hide server password");

        if response.clicked() {
            show_plaintext = !show_plaintext;
        }
        // Show the password field:
        ui.add_sized(
            ui.available_size(),
            egui_macroquad::egui::TextEdit::singleline(password).password(!show_plaintext),
        ).on_hover_text("Server password");


    });

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    result.response
}

pub fn password(password: &mut String) -> impl egui_macroquad::egui::Widget + '_ {
    move |ui: &mut egui_macroquad::egui::Ui| password_ui(ui, password)
}

