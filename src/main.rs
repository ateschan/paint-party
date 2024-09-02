pub mod networking;
pub mod state;
pub mod ui;
pub mod intro;

use crate::ui::ui_driver::render_gui;
use crate::networking::networking_io::web_socket_handler;
use crate::state::canvas::Canvas;
use crate::intro::intro_loop::render_intro;

use macroquad::prelude::*;
use networking::handler::ws_flags_handler;
use quad_net::web_socket::WebSocket;

#[macroquad::main("Paint Party")]
async fn main() {
    //Init for sub main menu
    //Storage singleton
    let mut canvas: Canvas = Canvas::default();

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

    let mut storage = quad_storage::STORAGE.lock().unwrap();
    canvas.init_state(&mut storage);
    loop {
        render_intro(
            &mut storage,
            &mut cam,
            &mut orbit_angle,
            &party_logo,
            &mut frame_accel,
        )
        .await;

        if storage.get("intro_complete_flag").unwrap().parse::<bool>().unwrap() {
            break;
        }
        next_frame().await
    }

    let mut socket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");

    loop {
        let current_room = canvas.user.room;
        while !socket.connected() {}
        set_default_camera();
        clear_background(WHITE);

        canvas.render_paint();
        canvas.brush_handler(&mut socket).await;
        web_socket_handler(&mut socket, &mut canvas).await;
        render_gui(&mut storage, &mut canvas).await;
        ws_flags_handler(&mut canvas, &mut storage, &mut socket, current_room).await;
        next_frame().await;
    }
}
