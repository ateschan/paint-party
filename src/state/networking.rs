use crate::state::brush::Dot;
use quad_net::web_socket::WebSocket;
use quad_storage::LocalStorage;
use std::str::from_utf8;

pub async fn get(
    socket: &mut WebSocket,
    storage: &mut LocalStorage,
) -> Result<String, Box<dyn std::error::Error>> {
    // Attempt to receive bytes from the socket
    let request = format!(
        "{} {} {}",
        "GET",
        storage.get("room").unwrap().parse::<i32>().unwrap(),
        &storage.get("apikey").unwrap()
    );
    socket.send_text(&request);
    Ok(String::from("GET Sent!"))
}

pub async fn put(
    cache: &Vec<Dot>,
    ct: &mut i32,
    socket: &mut WebSocket,
    storage: &mut LocalStorage,
) -> Result<String, Box<dyn std::error::Error>> {
    // Attempt to receive bytes from the socket
    let request = format!(
        "{} {} {} {}",
        "PUT",
        storage.get("room").unwrap().parse::<i32>().unwrap(),
        &storage.get("apikey").unwrap(),
        nanoserde::SerJson::serialize_json(cache)
    );
    socket.send_text(&request);

    *ct = 0;
    Ok(String::from("PUT Sent!"))
}

pub async fn delete(
    socket: &mut WebSocket,
    storage: &mut LocalStorage,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!(
        "{} {} {}",
        "DEL",
        storage.get("room").unwrap().parse::<i32>().unwrap(),
        &storage.get("apikey").unwrap(),
    );
    socket.send_text(&request);
    Ok(String::from("DEL Sent!"))
}

// pub async fn chat_out(
//     socket: &mut WebSocket,
//     chat: String,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let request = format!("{} {}", "CHAT", chat);
//     socket.send_text(&request);
//     Ok(String::from("CHAT Success!"))
// }

pub async fn web_socket_handler(
    socket: &mut WebSocket,
    lines: &mut Vec<Dot>,
    storage: &mut LocalStorage,
) {
    if let Some(res) = socket.try_recv() {
        let res_text: &str = from_utf8(&res).unwrap();
        let message: Vec<&str> = res_text.split(' ').collect();

        match message[0] {




            // Server response abstractions
            "GET_RES" => {
                let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                //println!("THIS SHOULD BE VEC DOT {:?}", new);
                lines.clear();
                lines.extend(new);
            }
            "UPD_RES" => {
                if message[1] == storage.get("room").unwrap() {
                    let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[2]).unwrap();
                    lines.extend(new);
                }
            }
            "PUT_RES" => println!("SERVER PUT RES: {}", message[1]),
            "DEL_RES" => println!("SERVER DEL RES: {}", message[1]),
            "RMV_RES" => {
                lines.clear();
            },
            _ => println!("UNDEFINED RES"),
        }
    }
}
