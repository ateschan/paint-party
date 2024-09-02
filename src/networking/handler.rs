use crate::networking::networking_io::{delete, get, put};
use crate::state::canvas::Canvas;
use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;
use quad_storage::LocalStorage;

pub async fn handle_ws_flags(
    canvas: &mut Canvas,
    storage: &mut LocalStorage,
    socket: &mut WebSocket,
    current_room: i32,
) {
    // PUT REQUEST TO WEBSOCKET
    if !canvas.cache.is_empty() && !is_mouse_button_down(MouseButton::Left) {
        canvas.lines.extend(canvas.cache.clone());
        println!("EXTENDING LINES");
        match put(
            &canvas.cache.clone(),
            &mut canvas.frame_count,
            socket,
            storage,
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
    if storage
        .get("clear_local_flag")
        .unwrap()
        .parse::<bool>()
        .unwrap()
    {
        canvas.lines = Vec::new();
        match delete(socket, storage).await {
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
        match get(socket, storage).await {
            Ok(res) => println!("{}", res),
            Err(e) => println!("ERROR {e}"),
        }
        storage.set("refresh_flag", "false");
    }
}
