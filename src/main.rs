pub mod state;
pub mod ui;

use macroquad::prelude::*;
use std::vec::Vec;
use state::brush::{Brush, Dot};
use ui::toolbar::render_gui;
use state::networking::{get, put};

//Global object for state
pub static mut BRUSH: Brush = Brush {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
    size: 15.0,
    sw: true,
    room: 0000,
    ip: String::new(),
    apikey: String::new(),
    frame_counter : -999999
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
                    a: BRUSH.a,
                    size: BRUSH.size,
                };
                cache.push(dot);
            }
            else if !cache.is_empty() {
                lines.extend(cache.clone());
                put(&mut cache, &mut frame_count).await;
            }
            draw_circle(
                    mouse_position().0,
                    mouse_position().1,
                    BRUSH.size,
                    macroquad::color::Color::from_rgba(
                        BRUSH.r,
                        BRUSH.g,
                        BRUSH.b,
                        BRUSH.a,
                    ),
                );
            let current_room = BRUSH.room;
            render_gui(&mut lines);
            
            //recieve data from server
            if (BRUSH.room != current_room || BRUSH.frame_counter >= 600 ) && !is_mouse_button_down(MouseButton::Left) && (!BRUSH.apikey.eq("") || !BRUSH.ip.eq("")){
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
                circle.r,
                circle.g,
                circle.b,
                circle.a,
            ),
        );
    }
}
