pub mod intro;
pub mod networking;
pub mod state;
pub mod ui;

use crate::state::canvas::Canvas;
use crate::ui::ui_driver::render_gui;

use intro::intro_loop;
use macroquad::prelude::*;
use networking::ws::WsClient;
use quad_net::web_socket::WebSocket;

// HACK:
//
// *) TODO: Polish notification system with colors or some shit
//
// HACK:
//
//
//Driver
#[macroquad::main("Paint Party")]
async fn main() {
    #[allow(clippy::await_holding_lock)]
    let mut storage = quad_storage::STORAGE.lock().unwrap();

    intro_loop::enter_intro(&mut storage).await;

    //Raw connection, BUG: Needs error handling
    let connsocket = WebSocket::connect(storage.get("socket").unwrap())
        .expect("ERROR: Failed to connect to websocket, validate address");

    //DEF GUI CANVAS WEBSOCKET
    let mut gui = crate::ui::ui_driver::tray_builder();
    let mut canvas: Canvas = Canvas::default();
    let mut wsc = WsClient {
        socket: connsocket,
        user: crate::networking::user::User {
            uuid: 0,
            room: 0,
            apikey: storage.get("apikey").unwrap(),
        },
        chats_inc: Vec::new(),
        chats_out: Vec::new(),
        notification_flags: Vec::new(),
        players_online : 0
    };

    //Check for socket disconnect
    while !wsc.socket.connected() {}
    std::mem::drop(storage);

    //Canvas directly handles rendering paint state.
    //Web socket takes in a mutable reference to canvas
    //Gui takes in a mutable reference to canvas

    loop {
        //Reset camera
        set_default_camera();

        //Set background color
        clear_background(WHITE);

        //Render lines, cache, particles to frame
        canvas.render_paint();

        //Render current brush
        canvas.brush_handler(&mut wsc).await;

        //Render gui, in handler is handled per module (chat)
        render_gui(&mut gui, &mut canvas, &mut wsc).await;

        //Handle incoming & outgoing canvas websocket requests
        wsc.in_handler(&mut canvas).await;
        wsc.canvas_out_handler(&mut canvas).await;

        //println!("{:?}",canvas);

        //Pass frame render data to macroquad
        next_frame().await;
    }
}
