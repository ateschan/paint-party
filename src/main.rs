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
    clear_flag: false,
    room: 0000,
    apikey: String::new(),
    refresh_flag: false,
};

#[macroquad::main("Paint Party")]
async fn main() {
    unsafe {
        let mut socket = WebSocket::connect(
            env::var("PARTY_SERVER")
                .expect("ERROR: Failed to find environment variable, ensure it is set"),
        )
        .expect("ERROR: Failed to connect to websocket, validate address");
        while !socket.connected() {}
        let mut lines = Vec::new();
        let mut cache: Vec<Dot> = Vec::new();
        let mut frame_count = 0;

        loop {
            let current_room = BRUSH.room;
            web_socket_handler(&mut socket, &mut lines).await;
            clear_background(WHITE);
            render_paint(&lines);
            render_paint(&cache);
            render_cursor();
            render_gui(&mut lines).await;

            //PAINT FUNC
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
            if BRUSH.clear_flag && socket.connected() {
                lines = Vec::new();
                match delete(&mut socket).await {
                    Ok(l) => {
                        println!("{l}");
                    }
                    Err(e) => println!("ERROR {e}"),
                }
                BRUSH.clear_flag = !BRUSH.clear_flag;
            }

            // GET REFRESH BUTTON
            if BRUSH.refresh_flag || BRUSH.room != current_room {
                match get(&mut socket).await {
                    Ok(res) => println!("{}", res),
                    Err(e) => println!("ERROR {e}"),
                }
                BRUSH.refresh_flag = !BRUSH.refresh_flag;
            }
            next_frame().await;
        }
    }
}

fn render_cursor() {
    unsafe {
        draw_circle(
            mouse_position().0,
            mouse_position().1,
            BRUSH.size,
            macroquad::color::Color::from_rgba(BRUSH.r, BRUSH.g, BRUSH.b, BRUSH.a),
        );
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
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
