use crate::state::brush::Dot;
use crate::ui::password::password;
use crate::BRUSH;
use egui_macroquad::egui::{self};
use egui_macroquad::egui::{epaint::Shadow, Color32, FontFamily, Frame, RichText};

pub async fn render_gui(lines: &mut Vec<Dot>) {
    unsafe {
        let mut tmp_room = BRUSH.room;

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new(RichText::new("PAINT PARTY").size(15.0).strong())
                .frame(
                    egui::Frame::default()
                        .inner_margin(4.0)
                        .shadow(Shadow::NONE)
                        .fill(Color32::TRANSPARENT)
                        .stroke(egui_macroquad::egui::Stroke::new(4.0, Color32::BLACK)),
                )
                .show(egui_ctx, |ui| {
                    ui.vertical(|ui| {
                        BRUSH.sw = !egui_ctx.is_using_pointer() && !egui_ctx.is_pointer_over_area();
                        egui_ctx.set_visuals(egui::Visuals::light());

                        let mut color_button: egui_macroquad::egui::Color32 =
                            egui_macroquad::egui::Color32::from_rgba_unmultiplied(
                                BRUSH.r, BRUSH.g, BRUSH.b, BRUSH.a,
                            );
                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut color_button);

                            if ui.button("↺").clicked() {
                                BRUSH.refresh_flag = true;
                            }

                            if ui.button("CLEAR").on_hover_text("Erase All").clicked() {
                                *lines = Vec::new();
                                BRUSH.clear_flag = true;
                            }

                            ui.add_sized(
                                ui.available_size(),
                                egui::Slider::new(&mut BRUSH.size, 0.0..=300.0),
                            )
                            .on_hover_text("Brush Size");
                        });

                        ui.horizontal(|ui| {
                            if ui
                                .add(
                                    egui_macroquad::egui::DragValue::new(&mut tmp_room)
                                        .speed(0.01)
                                        .clamp_range(0.0..=9999.0),
                                )
                                .lost_focus()
                                && ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter))
                            {
                                BRUSH.room = tmp_room;
                            }

                            ui.add(password(&mut BRUSH.apikey));
                        });

                        BRUSH = BRUSH.swapcolor(
                            egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button),
                        );
                    });
                });
        });
        egui_macroquad::draw();
    }
}
