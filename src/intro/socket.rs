//
//
//
//
//
//
//
//
//WIP
pub fn socket_ui(
    ui: &mut egui_macroquad::egui::Ui,
    socket: &mut String,
) -> egui_macroquad::egui::Response {
    let state_id = ui.id().with("show_plaintext");

    let mut show_plaintext = ui.data_mut(|d| d.get_temp::<bool>(state_id).unwrap_or(false));

    let result = ui.with_layout(
        egui_macroquad::egui::Layout::right_to_left(egui_macroquad::egui::Align::Center),
        |ui| {
            if ui
                .add(egui_macroquad::egui::SelectableLabel::new(
                    show_plaintext,
                    "👁",
                ))
                .on_hover_text("Show/hide server socket")
                .clicked()
            {
                show_plaintext = !show_plaintext;
            }
            // Show the socket field:
            ui.add(egui_macroquad::egui::TextEdit::singleline(socket))
                .highlight()
                .on_hover_text("Server socket");
        },
    );

    // Store the (possibly changed) state:
    ui.data_mut(|d| d.insert_temp(state_id, show_plaintext));

    result.response
}

pub fn socket(socket: &mut String) -> impl egui_macroquad::egui::Widget + '_ {
    move |ui: &mut egui_macroquad::egui::Ui| socket_ui(ui, socket)
}
