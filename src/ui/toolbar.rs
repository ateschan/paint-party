use crate::BRUSH;
use crate::state::brush::Dot;
use crate::state::networking::delete;
use crate::ui::password::password;
use crate::ui::serveraddress::server_address;
use egui_macroquad::egui;



pub fn render_gui(lines: &mut Vec<Dot>) {
    unsafe {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("PAINT PARTY").show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    BRUSH.sw = !egui_ctx.is_using_pointer() && !egui_ctx.is_pointer_over_area();

                    egui_ctx.set_visuals(egui::Visuals::light());

                    let mut color : egui_macroquad::egui::Color32 = egui_macroquad::egui::Color32::from_rgba_unmultiplied(BRUSH.r, BRUSH.g, BRUSH.b, BRUSH.a);
                    ui.horizontal(|ui| {

                        ui.color_edit_button_srgba(&mut color);

                        let refresh_button = ui.button("↺");
                        if refresh_button.clicked() {
                            BRUSH.frame_counter = 600;
                        }


                        let clear_button = ui.button("CLEAR");
                        if clear_button.clicked() {
                            *lines = Vec::new();
                            delete();
                        }

                        ui.add(password(&mut BRUSH.apikey));
                        clear_button.on_hover_text("Erase All");
                    });

                    ui.horizontal(|ui| {
                        let mut tmp_room = BRUSH.room;
                        let size_slider = ui.add(egui::Slider::new(&mut BRUSH.size, 0.0..=300.0));
                        
                        ui.add(server_address(&mut BRUSH.ip, &mut tmp_room));

                        size_slider.on_hover_text("Brush Size");
                    });
                    
                    BRUSH = BRUSH.swapcolor(egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color));
                    println!("{:?}",egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color));
                });
            });
        });
        egui_macroquad::draw();
    }
}
