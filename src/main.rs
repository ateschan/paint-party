pub mod networking;
pub mod state;
pub mod ui;

use crate::networking::networking::web_socket_handler;
use crate::state::canvas::Canvas;
use macroquad::prelude::*;
use networking::handler::handle_flags;
use quad_net::web_socket::WebSocket;
use quad_storage::LocalStorage;
use ui::{intro::render_intro, toolbar::render_gui};

//TODO Implement Hashmap instead of Vec<Dot>

#[macroquad::main("Paint Party")]
async fn main() {
    //Init for sub main menu
    //Storage singleton
    let storage = &mut quad_storage::STORAGE.lock().unwrap();
    let mut canvas: Canvas = Canvas::default();
    canvas.init_state(storage);

    let mut cam = Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    let mut orbit_angle: f32 = 0.0;
    let party_logo = load_texture("assets/party.png").await.unwrap();
    let mut frame_accel = 0.0;
    request_new_screen_size(1920.0, 1080.0);

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

    let mut socket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");

    loop {
        let current_room = storage.get("room").unwrap().parse::<i32>().unwrap();
        while !socket.connected() {}
        set_default_camera();
        clear_background(WHITE);

        canvas.render_paint();
        canvas.brush_handler(storage, &mut socket).await;
        web_socket_handler(&mut socket, &mut canvas, storage).await;
        render_gui(storage).await;
        handle_flags(&mut canvas, storage, &mut socket, current_room).await;
        next_frame().await;
    }
}
