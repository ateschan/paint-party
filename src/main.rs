pub mod state;
pub mod ui;
use crate::state::networking::{delete, get, put, web_socket_handler};
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;
use state::brush::{init_state, Dot};
use std::vec::Vec;
use ui::{intro::render_intro, toolbar::render_gui};

//TODO Implement Hashmap instead of Vec<Dot>


#[macroquad::main("Paint Party")]
async fn main() {

    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    init_state(storage);
    let mut cam = Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    let mut orbit_angle: f32 = 0.0;
    let party_logo = load_texture("assets/party.png").await.unwrap();
    let mut frame_accel = 0.0;

    loop {
        render_intro(storage, &mut cam, &mut orbit_angle, &party_logo, &mut frame_accel).await;

        if storage
            .get("intro_complete")
            .unwrap()
            .parse::<bool>()
            .unwrap() {
            break;
        }
        next_frame().await
    }

    macroquad::window::set_fullscreen(true);
    let mut socket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");

    let mut lines = Vec::new();
    let mut cache: Vec<Dot> = Vec::new();
    let mut frame_count = 0;
    loop {

        while !socket.connected() {}
        //RENDER & INIT
        set_default_camera();
        clear_background(WHITE);

        render_paint(&lines);
        render_paint(&cache);
        render_cursor(storage);

        web_socket_handler(&mut socket, &mut lines, storage).await;

        let current_room = storage.get("room").unwrap().parse::<i32>().unwrap();
        render_gui(storage).await;

        //INPUT
        if is_mouse_button_down(MouseButton::Left)
            && storage.get("brush_state").unwrap().parse::<bool>().unwrap()
            && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
        {
            let dot = Dot {
                x: mouse_position().0,
                y: mouse_position().1,
                r: storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                g: storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                b: storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                a: storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                size: storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
                id : nanoid::nanoid!()
            };
            cache.push(dot);
        } else if !cache.is_empty() {
            lines.extend(cache.clone());
            println!("EXTENDING LINES");
            match put(&cache.clone(), &mut frame_count, &mut socket, storage).await {
                Ok(res) => {
                    println!("{:?}", res);
                    cache.clear();
                }
                Err(e) => println!("{:?}", e),
            }
            println!("CLEARING CACHE");
        }

        // DEL REQUEST TO WEBSOCKET
        if storage
            .get("clear_local_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            lines = Vec::new();
            match delete(&mut socket, storage).await {
                Ok(l) => {
                    println!("{l}");
                }
                Err(e) => println!("ERROR {e}"),
            }
            storage.set("clear_local_flag", "false");
        }

        // GET REFRESH BUTTON
        if storage
            .get("refresh_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap()
            || storage.get("room").unwrap().parse::<i32>().unwrap() != current_room
        {
            match get(&mut socket, storage).await {
                Ok(res) => println!("{}", res),
                Err(e) => println!("ERROR {e}"),
            }
            storage.set("refresh_flag", "false");
        }

        next_frame().await;
    }
}

fn render_cursor(storage: &mut quad_storage::LocalStorage) {
    draw_circle(
        mouse_position().0,
        mouse_position().1,
        storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
        macroquad::color::Color::from_rgba(
            storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
            storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
            storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
            storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
        ),
    );
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
