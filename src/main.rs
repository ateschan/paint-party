use egui_macroquad::egui;
use macroquad::prelude::*;
use std::vec::Vec;
pub mod state;
use state::brush::{Brush, Dot};

pub static mut BRUSH: Brush = Brush {
    r: 0.01,
    g: 0.01,
    b: 0.01,
    size: 15.0,
    sw: true,
    room: 0000,
    ip : String::new(),
    apikey : String::new()
};

use crate::state::networking::{delete, get, put};

#[macroquad::main("Paint Party")]
async fn main() {
    let mut lines: Vec<Dot> = Vec::new();
    let mut cache: Vec<Dot> = Vec::new();
    for val in get(&mut cache).await {
        lines.push(val);
    }
    let mut ct = 0;
    loop {
        //Weird data race between the await and the timer.
        if ct > 200 && (!cache.is_empty()) {
            lines.extend(cache.clone());
            put(&mut cache, &mut ct).await;
        }

        egui_macroquad::draw();
        clear_background(WHITE);
        render_paint(&mut lines[..]);
        render_paint(&mut cache[..]);

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
            }

            ct += 1;
            let currentrm = BRUSH.room;
            render_gui(&mut lines);
            if BRUSH.room != currentrm {
                lines = get(&mut Vec::new()).await
            }
            next_frame().await
        }
    }
}

pub fn render_gui(lines: &mut Vec<Dot>) {
    unsafe {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("PAINT PARTY").show(egui_ctx, |ui| {
                ui.vertical(|ui| {

                    //Switch brush off if egui is using it 
                    BRUSH.sw = match (egui_ctx.is_using_pointer(), egui_ctx.is_pointer_over_area()) {
                        (true, _) | (_, true) => false,
                        (false, false) => true,
                    };

                    egui_ctx.set_visuals(egui::Visuals::light());
                    let mut color = [BRUSH.r, BRUSH.g, BRUSH.b];
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut color);
                        let roomnumber = ui.add(
                            egui::DragValue::new(&mut BRUSH.room)
                                .speed(0.5)
                                .clamp_range(0.0..=100.0),
                        );
                        let clearbtn = ui.button("CLEAR");
                        if clearbtn.clicked() {
                            delete();
                            *lines = Vec::new()
                        }
                        let apikey = ui.add(egui::TextEdit::singleline(&mut BRUSH.apikey));
                        
                        clearbtn.on_hover_text("Erase All");
                        apikey.on_hover_text("Server Password");
                        roomnumber.on_hover_text("Room #");
                    });

                    ui.horizontal(|ui| {
                        let slider = ui.add(egui::Slider::new(&mut BRUSH.size, 0.0..=100.0));
                        let server = ui.add(egui::TextEdit::singleline(&mut BRUSH.ip));
                        server.on_hover_text("IP:Port/Hostname");
                        slider.on_hover_text("Brush Size");
                    });

                    BRUSH = BRUSH.swapcolor(color);
                });
            });
        });
        egui_macroquad::draw();
    }
}

pub fn render_paint(lines: &mut [Dot]) {
    for circle in lines.iter_mut() {
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
