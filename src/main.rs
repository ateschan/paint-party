use egui_macroquad::egui;
use macroquad::prelude::*;
use std::vec::Vec;

pub mod state;
use state::brush::{Brush, Dot};
use state::networking::{delete, get, put};

pub static mut BRUSH: Brush = Brush {
    r: 0.01,
    g: 0.01,
    b: 0.01,
    size: 15.0,
    sw: true,
    room: 0000,
    ip: String::new(),
    apikey: String::new(),
};

#[macroquad::main("Paint Party")]
async fn main() {
    let mut lines: Vec<Dot> = Vec::new();
    let mut cache: Vec<Dot> = Vec::new();
    lines.extend(get(&mut cache).await);

    let mut frame_count = 0;

    loop {
        if frame_count > 200 && !cache.is_empty() {
            lines.extend(cache.clone());
            put(&mut cache, &mut frame_count).await;
        }

        egui_macroquad::draw();
        clear_background(WHITE);

        render_paint(&lines);
        render_paint(&cache);

        unsafe {
            if is_mouse_button_down(MouseButton::Left) && BRUSH.sw {
                let dot = Dot {
                    x: mouse_position().0,
                    y: mouse_position().1,
                    r: BRUSH.r,
                    g: BRUSH.g,
                    b: BRUSH.b,
                    size: BRUSH.size,
                };
                cache.push(dot);
            }
            draw_circle(
                    mouse_position().0,
                    mouse_position().1,
                    BRUSH.size,
                    macroquad::color::Color::from_rgba(
                        (BRUSH.r * 255.0) as u8,
                        (BRUSH.g * 255.0) as u8,
                        (BRUSH.b * 255.0) as u8,
                        255,
                    ),
                );
            frame_count += 1;
            let current_room = BRUSH.room;
            render_gui(&mut lines);

            if BRUSH.room != current_room {
                lines = get(&mut Vec::new()).await;
            }
        }

        next_frame().await;
    }
}

fn render_gui(lines: &mut Vec<Dot>) {
    unsafe {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("PAINT PARTY").show(egui_ctx, |ui| {
                ui.vertical(|ui| {
                    BRUSH.sw = !egui_ctx.is_using_pointer() && !egui_ctx.is_pointer_over_area();

                    egui_ctx.set_visuals(egui::Visuals::light());

                    let mut color = [BRUSH.r, BRUSH.g, BRUSH.b];
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut color);

                        let room_number = ui.add(
                            egui::DragValue::new(&mut BRUSH.room)
                                .speed(0.5)
                                .clamp_range(0.0..=100.0),
                        );
                        let clear_button = ui.button("CLEAR");
                        if clear_button.clicked() {
                            delete();
                            *lines = Vec::new();
                        }

                        let apikey_input = ui.add(egui::TextEdit::singleline(&mut BRUSH.apikey));
                        clear_button.on_hover_text("Erase All");
                        apikey_input.on_hover_text("Server Password");
                        room_number.on_hover_text("Room #");
                    });

                    ui.horizontal(|ui| {
                        let size_slider = ui.add(egui::Slider::new(&mut BRUSH.size, 0.0..=100.0));
                        let server_input = ui.add(egui::TextEdit::singleline(&mut BRUSH.ip));
                        server_input.on_hover_text("IP:Port/Hostname");
                        size_slider.on_hover_text("Brush Size");
                    });

                    BRUSH = BRUSH.swapcolor(color);
                });
            });
        });
        egui_macroquad::draw();
    }
}

fn render_paint(lines: &[Dot]) {
    for circle in lines.iter() {
        draw_circle(
            circle.x,
            circle.y,
            circle.size,
            macroquad::color::Color::from_rgba(
                (circle.r * 255.0) as u8,
                (circle.g * 255.0) as u8,
                (circle.b * 255.0) as u8,
                255,
            ),
        );
    }
}
