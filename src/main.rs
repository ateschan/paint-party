pub mod state;
pub mod ui;

use macroquad::prelude::*;
use state::brush::{Brush, Dot};
use state::networking::{get, put};
use std::vec::Vec;
use ui::toolbar::render_gui;

//Global object for state
pub static mut BRUSH: Brush = Brush {
    r: 0.01,
    g: 0.01,
    b: 0.01,
    size: 15.0,
    sw: true,
    room: 0000,
    ip: String::new(),
    apikey: String::new(),
    frame_counter: -999999,
};

#[macroquad::main("Paint Party")]
async fn main() {
    let mut lines: Vec<Dot> = Vec::new();
    let mut cache: Vec<Dot> = Vec::new();
    lines.extend(get(&mut cache).await);

    let mut frame_count = 0;

    loop {
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
            } else if !cache.is_empty() {
                lines.extend(cache.clone());
                put(&mut cache, &mut frame_count).await;
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
            let current_room = BRUSH.room;
            render_gui(&mut lines);

            //recieve data from server
            if (BRUSH.room != current_room || BRUSH.frame_counter >= 600)
                && !is_mouse_button_down(MouseButton::Left)
                && (!BRUSH.apikey.eq("") || !BRUSH.ip.eq(""))
            {
                lines = get(&mut Vec::new()).await;
                BRUSH.frame_counter = 0
            }
            BRUSH.frame_counter += 1;
        }

        next_frame().await;
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
