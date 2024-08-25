pub mod state;
pub mod ui;

use crate::state::networking::{delete, get, put, web_socket_handler};
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;
use state::brush::{Brush, Dot};
use std::env;
use std::vec::Vec;
use ui::toolbar::render_gui;


//Global object for state
pub static mut BRUSH: Brush = Brush {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
    size: 15.0,
    sw: true,
    clear: false,
    room: 0000,
    apikey: String::new(),
    refresh_flag :  false
};

#[macroquad::main("Paint Party")]
async fn main() {

    unsafe {
        let mut socket = WebSocket::connect(env::var("PARTY_SERVER").unwrap()).unwrap();
        let mut lines = Vec::new();
        let mut cache: Vec<Dot> = Vec::new();
        let mut frame_count = 0;

        loop {
            web_socket_handler(&mut socket, &mut lines).await;

            clear_background(WHITE);

            draw_circle(
                mouse_position().0,
                mouse_position().1,
                BRUSH.size,
                macroquad::color::Color::from_rgba(BRUSH.r, BRUSH.g, BRUSH.b, BRUSH.a),
            );

            let current_room = BRUSH.room;
            render_gui(&mut lines).await;
            while !socket.connected() {
                next_frame().await;
            }
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
                cache.push(dot.clone());
            } else if !cache.is_empty() {
                lines.extend(cache.clone());
                println!("EXTENDING LINES");
                match put(&cache.clone(), &mut frame_count, &mut socket).await {
                    Ok(res) => {
                        println!("{:?}", res);
                        cache.clear();
                    }
                    Err(e) => println!("{:?}", e),
                }
                println!("CLEARING CACHE");
            }

            // DEL REQUEST TO WEBSOCKET
            if BRUSH.clear && socket.connected() {
                lines = Vec::new();
                match delete(&mut socket).await {
                    Ok(l) => {
                        println!("{l}");
                    }
                    Err(e) => println!("ERROR {e}"),
                }
                BRUSH.clear = !BRUSH.clear;
            }
            
            if BRUSH.refresh_flag {
                match get(&mut socket).await {
                    Ok(res) => println!("{}", res),
                    Err(e) => println!("ERROR {e}"),
                }
            }

            render_paint(&lines);
            render_paint(&cache);
            next_frame().await;
        }
    }
}

fn render_paint(lines: &[Dot]) {
    for circle in lines.iter() {
        draw_circle(
            circle.x,
            circle.y,
            circle.size,
            macroquad::color::Color::from_rgba(circle.r, circle.g, circle.b, circle.a),
        );
    }
}
