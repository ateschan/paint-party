use crate::state::brush::Dot;
use crate::ui::password::password;
use egui_macroquad::egui::{self};
use egui_macroquad::egui::{epaint::Shadow, Color32, RichText};
use quad_storage::LocalStorage;

pub async fn render_gui(lines: &mut Vec<Dot>, storage: &mut LocalStorage) {
    let mut tmp_room = storage.get("room").unwrap().parse::<i32>().unwrap();
    let mut tmp_pass = storage.get("apikey").unwrap();

    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new(RichText::new("PAINT PARTY").size(15.0).strong())
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::TRANSPARENT)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::TRANSPARENT)),
            )
            .show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    storage.set(
                        "brush_state",
                        &format!(
                            "{}",
                            !egui_ctx.is_using_pointer() && !egui_ctx.is_pointer_over_area()
                        ),
                    );
                    egui_ctx.set_visuals(egui::Visuals::light());

                    let mut color_button: egui_macroquad::egui::Color32 =
                        egui_macroquad::egui::Color32::from_rgba_unmultiplied(
                            storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                            storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                            storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                            storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                        );
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut color_button);

                        if ui.button("↺").clicked() {
                            storage.set("refresh_flag", "true");
                        }

                        if ui.button("CLEAR").on_hover_text("Erase All").clicked() {
                            *lines = Vec::new();
                            storage.set("clear_flag", "true");
                        }

                        ui.add_sized(
                            ui.available_size(),
                            egui::Slider::new(
                                &mut storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
                                0.0..=300.0,
                            )
                            .trailing_fill(true),
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
                            .lost_focus()
                            || ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter))
                        {
                            storage.set("room", &tmp_room.to_string());
                        }

                        ui.add(password(&mut tmp_pass));
                    });

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
