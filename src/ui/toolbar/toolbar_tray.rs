use crate::ui::toolbar::password::password;
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};
use macroquad::math::bool;
use quad_storage::LocalStorage;
    
pub fn toolbar(storage : &mut LocalStorage) {
let mut tmp_room = storage.get("room").unwrap().parse::<i32>().unwrap();
    let mut tmp_pass = storage.get("apikey").unwrap();
    let mut tmp_size = storage.get("brush_size").unwrap().parse::<f32>().unwrap();

    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new(RichText::new("PAINT PARTY"))
            //.to_owned() + &storage.get("socket").unwrap()).size(14.0).strong()
            .resizable(false)
            .default_pos([10.0, 10.0])
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::TRANSPARENT)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::TRANSPARENT)),
            )
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    if egui_ctx.is_using_pointer() || egui_ctx.is_pointer_over_area() {
                        storage.set("brush_hamper", "false");
                    } else {
                        storage.set("brush_hamper", "true")
                    }

                    egui_ctx.set_visuals(egui::Visuals::light());

                    let mut color_button: egui_macroquad::egui::Color32 =
                        egui_macroquad::egui::Color32::from_rgba_unmultiplied(
                            storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                            storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                            storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                            storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                        );

                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut color_button)
                            .on_hover_text("Change color");

                        if ui.button("[]").on_hover_text("Eraser").clicked() {
                            storage.set("brush_state", "Erase");
                        }
                        if ui.button("/").on_hover_text("Paintbrush").clicked() {
                            storage.set("brush_state", "On");
                        }
                        if ui
                            .add(egui_macroquad::egui::SelectableLabel::new(
                                !storage
                                    .get("brush_particles")
                                    .unwrap()
                                    .parse::<bool>()
                                    .unwrap(),
                                "*",
                            ))
                            .on_hover_text("Particles Toggle")
                            .clicked()
                        {
                            storage.set(
                                "brush_particles",
                                &(!storage
                                    .get("brush_particles")
                                    .unwrap()
                                    .parse::<bool>()
                                    .unwrap())
                                .to_string(),
                            );
                        }

                        ui.add_sized(
                            ui.available_size(),
                            egui::Slider::new(&mut tmp_size, 0.0..=300.0).trailing_fill(true),
                        )
                        .on_hover_text("Brush Size");
                    });

                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                egui_macroquad::egui::DragValue::new(&mut tmp_room)
                                    .update_while_editing(false)
                                    .speed(1.00)
                                    .clamp_range(0.0..=9999.0),
                            )
                            .on_hover_text("Server room")
                            .on_hover_cursor(egui::CursorIcon::Default)
                            .lost_focus()
                            || ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter))
                        {
                            storage.set("room", &tmp_room.to_string());
                        }
                        if ui.button("CLEAR").on_hover_text("Erase All").clicked() {
                            storage.set("clear_local_flag", "true");
                        }
                        if ui.button("↺").on_hover_text("Refresh").clicked() {
                            storage.set("refresh_flag", "true");
                        }

                        ui.add_sized(ui.available_size(), password(&mut tmp_pass));
                    });
                    storage.set("brush_size", &tmp_size.to_string());
                    storage.set("apikey", &tmp_pass);
                    storage.set(
                        "brush_r",
                        &egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[0]
                            .to_string(),
                    );
                    storage.set(
                        "brush_g",
                        &egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[1]
                            .to_string(),
                    );
                    storage.set(
                        "brush_b",
                        &egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[2]
                            .to_string(),
                    );
                    storage.set(
                        "brush_a",
                        &egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[3]
                            .to_string(),
                    );
                });
            });
    });
    egui_macroquad::draw();
}
