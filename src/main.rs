pub mod state;
pub mod ui;
use crate::state::networking::{delete, get, put, remove, web_socket_handler};
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;
use state::brush::{init_state, Dot};
use std::vec::Vec;
use ui::{intro::render_intro, toolbar::render_gui};

//TODO Implement Hashmap instead of Vec<Dot>


#[macroquad::main("Paint Party")]
async fn main() {
    //Init for sub main menu
    //

    //Storage singleton
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
        render_intro(
            storage,
            &mut cam,
            &mut orbit_angle,
            &party_logo,
            &mut frame_accel,
        )
        .await;

        if storage
            .get("intro_complete_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            break;
        }
        next_frame().await
    }

    //Init for main loop
    //

    macroquad::window::set_fullscreen(true);
    let mut socket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");
    let mut lines = Vec::new();
    let mut cache: Vec<Dot> = Vec::new();
    let mut garbage : Vec<String> = Vec::new();
    let mut frame_count = 0;
    let mut eraser_rot = 0.0;

    loop {
        while !socket.connected() {}
        //RENDER & INIT
        set_default_camera();
        clear_background(WHITE);

        render_paint(&lines);
        render_paint(&cache);

        match storage.get("brush_state").unwrap().as_str() {
            "On" => {
                render_paintbrush(storage);
                if is_mouse_button_down(MouseButton::Left)
                    && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
                    && storage.get("brush_hamper").unwrap().parse::<bool>().unwrap()
                {
                    let dot = Dot {
                        x: mouse_position().0,
                        y: mouse_position().1,
                        r: storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                        g: storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                        b: storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                        a: storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                        size: storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
                        id: nanoid::nanoid!(),
                    };
                    cache.push(dot);
                }
            }
            "Off" => {}
            "Erase" => {
                render_eraser(storage, &mut eraser_rot);

                if is_mouse_button_down(MouseButton::Left)
                    && mouse_delta_position() != macroquad::math::Vec2::new(0.0, 0.0)
                    && storage.get("brush_hamper").unwrap().parse::<bool>().unwrap()
                {
                    let dot = Dot {
                        x: mouse_position().0,
                        y: mouse_position().1,
                        r: storage.get("brush_r").unwrap().parse::<u8>().unwrap(),
                        g: storage.get("brush_g").unwrap().parse::<u8>().unwrap(),
                        b: storage.get("brush_b").unwrap().parse::<u8>().unwrap(),
                        a: storage.get("brush_a").unwrap().parse::<u8>().unwrap(),
                        size: storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
                        id: "0".to_string(),
                    };
                    garbage.extend(is_overlapping(&dot, &lines));
                    lines.retain(|dot| !garbage.contains(&dot.id));
                    eraser_rot +=15.0;
                }
                else {
                    let comp : Vec<String> = Vec::new();
                    if garbage != comp {
                        remove(&mut socket, storage, &garbage).await.unwrap();
                        garbage.clear();
                    }
                }
        
                if eraser_rot <= 360.0 {
                    eraser_rot +=3.0;
                }
                else {
                    eraser_rot = 0.0;
                }
            }
            &_ => {println!("UNABLE TO PARSE {} ", storage.get("brush_state").unwrap())}
        }

        web_socket_handler(&mut socket, &mut lines, storage).await;

        let current_room = storage.get("room").unwrap().parse::<i32>().unwrap();
        render_gui(storage).await;

        //INPUT

        // PUT REQUEST TO WEBSOCKET
        if !cache.is_empty() && !is_mouse_button_down(MouseButton::Left) {
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

        // GET REQUEST TO WEBSOCKET
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

fn render_paintbrush(storage: &mut quad_storage::LocalStorage) {
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

fn render_eraser(storage: &mut quad_storage::LocalStorage, eraser_rot : &mut f32) {
    draw_poly_lines(
        mouse_position().0,
        mouse_position().1,
        10,
        storage.get("brush_size").unwrap().parse::<f32>().unwrap(),
        *eraser_rot,
        5.0,
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



fn is_overlapping(circle1: &Dot, circles: &[Dot]) -> Vec<String> {
    let mut res :Vec<String> = Vec::new();
    for circle2 in circles {
        let distance_squared = (circle1.x - circle2.x).powi(2) + (circle1.y - circle2.y).powi(2);
        if distance_squared <= (circle1.size + circle2.size).powi(2) {
            res.push(circle2.id.to_owned());
        }
    }
    res
}
