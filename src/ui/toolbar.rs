use crate::state::brush::Dot;
use crate::ui::password::password;
use crate::BRUSH;
use egui_macroquad::egui;
use quad_net::web_socket::WebSocket;

pub async fn render_gui(lines: &mut Vec<Dot>) {
    unsafe {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("PAINT PARTY").show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    BRUSH.sw = !egui_ctx.is_using_pointer() && !egui_ctx.is_pointer_over_area();
                    let mut tmp_room = BRUSH.room;

                    egui_ctx.set_visuals(egui::Visuals::light());

                    let mut color: egui_macroquad::egui::Color32 =
                        egui_macroquad::egui::Color32::from_rgba_unmultiplied(
                            BRUSH.r, BRUSH.g, BRUSH.b, BRUSH.a,
                        );
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut color);

                        let refresh_button = ui.button("↺");
                        if refresh_button.clicked() {
                            BRUSH.refresh_flag = true;
                        }

                        let clear_button = ui.button("CLEAR");
                        if clear_button.clicked() {
                            *lines = Vec::new();
                            BRUSH.clear = true;
                        }

                        let size_slider = ui.add(egui::Slider::new(&mut BRUSH.size, 0.0..=300.0));
                        let room_button = ui.add(
                            egui_macroquad::egui::DragValue::new(&mut tmp_room)
                                .speed(0.01)
                                .clamp_range(0.0..=9999.0),
                        );
                        if room_button.lost_focus()
                            && ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter))
                        {
                            BRUSH.room = tmp_room;
                        }

                        size_slider.on_hover_text("Brush Size");
                        clear_button.on_hover_text("Erase All");
                    });

                    ui.horizontal(|ui| {
                        ui.add(password(&mut BRUSH.apikey));
                    });

                    BRUSH = BRUSH
                        .swapcolor(egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color));
                });
            });
        });
        egui_macroquad::draw();
    }
}
