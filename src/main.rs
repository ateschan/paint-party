pub mod intro;
pub mod networking;
pub mod state;
pub mod ui;

use crate::networking::networking_io::web_socket_handler;
use crate::state::canvas::Canvas;
use crate::ui::ui_driver::render_gui;

use intro::intro_loop;
use macroquad::prelude::*;
use networking::handler::ws_rq_handler;
use quad_net::web_socket::WebSocket;

//Driver
#[macroquad::main("Paint Party")]
async fn main() {
    let mut canvas: Canvas = Canvas::default();

    let mut storage = quad_storage::STORAGE.lock().unwrap();
    intro_loop::enter_intro(&mut storage).await;

    let mut socket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");

    std::mem::drop(storage);

    loop {
        //Check for socket disconnect
        while !socket.connected() {}

        //Reset camera
        set_default_camera();

        //Set background color
        clear_background(WHITE);

        //Render lines, cache, particles
        canvas.render_paint();

        //Handle user canvas input
        canvas.brush_handler(&mut socket).await;

        //Render gui
        render_gui(&mut canvas).await;

        //Handle incoming websocket requests
        web_socket_handler(&mut socket, &mut canvas).await;

        //Handle outgoing websocket requests
        ws_rq_handler(&mut canvas, &mut socket).await;

        next_frame().await;
    }
}
