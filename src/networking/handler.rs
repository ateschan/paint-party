use crate::networking::networking_io::{delete, get, put};
use crate::state::canvas::Canvas;
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

pub async fn ws_rq_handler(canvas: &mut Canvas, socket: &mut WebSocket) {
    // PUT REQUEST TO WEBSOCKET
    if !canvas.cache.is_empty() && !is_mouse_button_down(MouseButton::Left) {
        canvas.lines.extend(canvas.cache.clone());
        println!("EXTENDING LINES");
        match put(
            &canvas.cache.clone(),
            &mut canvas.frame_count,
            socket,
            &canvas.user,
        )
        .await
        {
            Ok(res) => {
                println!("{:?}", res);
                canvas.cache.clear();
            }
            Err(e) => println!("{:?}", e),
        }
        println!("CLEARING CACHE");
    }

    // DEL REQUEST TO WEBSOCKET
    if canvas.clear_flag {
        canvas.lines = Vec::new();
        match delete(socket, &canvas.user).await {
            Ok(l) => {
                println!("{l}");
            }
            Err(e) => println!("ERROR {e}"),
        }
        canvas.clear_flag = false;
    }

    // GET REQUEST TO WEBSOCKET
    if canvas.refresh_flag {
        match get(socket, &canvas.user).await {
            Ok(res) => println!("{}", res),
            Err(e) => println!("ERROR {e}"),
        }
        canvas.refresh_flag = false;
    }
}