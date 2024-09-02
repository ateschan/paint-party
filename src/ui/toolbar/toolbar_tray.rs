use crate::{state::canvas::{self, Canvas}, ui::toolbar::password::password};
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};
use macroquad::math::bool;
use quad_storage::LocalStorage;
use crate::state::brush::BrushState::*;
    
pub fn toolbar(storage : &mut LocalStorage, canvas : &mut Canvas) {
    let mut tmp_room = canvas.user.room;
    let mut tmp_pass = canvas.user.apikey.clone();
    let mut tmp_size = canvas.brush.size;

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
                    canvas.brush.hamper_self = egui_ctx.is_using_pointer() || egui_ctx.is_pointer_over_area();

                    egui_ctx.set_visuals(egui::Visuals::light());

                    let mut color_button: egui_macroquad::egui::Color32 =
                        egui_macroquad::egui::Color32::from_rgba_unmultiplied(
                            canvas.brush.r,
                            canvas.brush.g,
                            canvas.brush.b,
                            canvas.brush.a,
                        );

                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut color_button)
                            .on_hover_text("Change color");

                        if ui.button("[]").on_hover_text("Eraser").clicked() {
                            canvas.brush.state = Erase;
                        }
                        if ui.button("/").on_hover_text("Paintbrush").clicked() {
                            canvas.brush.state = Paint;
                        }
                        if ui
                            .add(egui_macroquad::egui::SelectableLabel::new(
                                canvas.brush.hamper_particles,
                                "*",
                            ))
                            .on_hover_text("Particles Toggle")
                            .clicked()
                        {
                            canvas.brush.hamper_particles = !canvas.brush.hamper_particles;
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
                            canvas.user.room = tmp_room;
                        }
                        if ui.button("CLEAR").on_hover_text("Erase All").clicked() {
                            storage.set("clear_local_flag", "true");
                        }
                        if ui.button("↺").on_hover_text("Refresh").clicked() {
                            storage.set("refresh_flag", "true");
                        }

                        ui.add_sized(ui.available_size(), password(&mut tmp_pass));
                    });

                    canvas.user.apikey = tmp_pass;
                    canvas.brush.size = tmp_size;
                    canvas.brush.r = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[0];
                    canvas.brush.g = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[1];
                    canvas.brush.b = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[2];
                    canvas.brush.a = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[3];
                });
            });
    });
    egui_macroquad::draw();
}
