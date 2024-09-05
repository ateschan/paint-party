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

// HACK:
//
// *) TODO: Work on chat feature
//
// *) TODO: Polish notification system
//
// HACK:
//
//Driver
#[macroquad::main("Paint Party")]
async fn main() {
    let mut canvas: Canvas = Canvas::default();
    #[allow(clippy::await_holding_lock)]
    let mut storage = quad_storage::STORAGE.lock().unwrap();

    intro_loop::enter_intro(&mut storage).await;

    let mut socket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");
    canvas.user.apikey = storage.get("apikey").unwrap();

    let mut gui = crate::ui::ui_driver::tray_builder();
    //std::mem::drop(storage);
    //Main entry point.
    //Canvas directly handles rendering paint state.
    //Web socket takes in a mutable reference to canvas
    //Gui takes in a mutable reference to canvas

    loop {
        //Check for socket disconnect
        while !socket.connected() {}

        //Reset camera
        set_default_camera();

        //Set background color
        clear_background(WHITE);

        //Render lines, cache, particles to frame
        canvas.render_paint();

        //Handle user canvas input
        //Socket is used here for eraser
        canvas.brush_handler(&mut socket).await;

        //Render gui to frame
        render_gui(&mut gui, &mut canvas).await;

        //Handle incoming websocket requests
        web_socket_handler(&mut socket, &mut canvas).await;

        //Handle outgoing websocket requests
        ws_rq_handler(&mut canvas, &mut socket).await;

        //Pass frame render data to macroquad
        next_frame().await;
    }
}
