use std::str::from_utf8;
use crate::state::brush::Dot;
use crate::BRUSH;
use quad_net::web_socket::WebSocket;

pub async fn get(socket: &mut WebSocket) -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        // Attempt to receive bytes from the socket
        let request = format!("{} {} {}", "GET", BRUSH.room, BRUSH.apikey);
        socket.send_text(&request);
    }
    Ok(String::from("GET Success!"))
}

pub async fn put(
    cache: &Vec<Dot>,
    ct: &mut i32,
    socket: &mut WebSocket,
) -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        // Attempt to receive bytes from the socket
        let request = format!(
            "{} {} {} {}",
            "PUT",
            BRUSH.room,
            BRUSH.apikey,
            nanoserde::SerJson::serialize_json(cache)
        );
        socket.send_text(&request);

        *ct = 0;
        Ok(String::from("PUT Success!"))
    }
}

pub async fn delete(socket: &mut WebSocket) -> Result<String, Box<dyn std::error::Error>> {
    unsafe {
        let request = format!("{} {} {}", "DEL", BRUSH.room, BRUSH.apikey);
        socket.send_text(&request);
    }
    Ok(String::from("DEL Success!"))
}

// pub async fn chat_out(
//     socket: &mut WebSocket,
//     chat: String,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let request = format!("{} {}", "CHAT", chat);
//     socket.send_text(&request);
//     Ok(String::from("CHAT Success!"))
// }

pub async fn web_socket_handler(socket: &mut WebSocket, lines: &mut Vec<Dot>) {
    unsafe {
        if let Some(res) = socket.try_recv() {
            let res_text: &str = from_utf8(&res).unwrap();
            let message: Vec<&str> = res_text.split(' ').collect();

            match message[0] {
                "GET_RES" => {

                    let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                    //println!("THIS SHOULD BE VEC DOT {:?}", new);
                    lines.clear();
                    lines.extend(new);
                }
                "UPD_RES" => {
                    if message[1] == BRUSH.room.to_string() {
                        println!("REVIEVED UPDATE: {:?}", message[2]);
                        let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[2]).unwrap();
                        lines.extend(new);
                    }
                }
                "PUT_RES" => println!("{}", message[1]),
                "DEL_RES" => println!("{}", message[1]),
                _ => println!("UNDEFINED RES"),
            }
        }
    }
}
