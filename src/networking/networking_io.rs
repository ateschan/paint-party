use crate::state::canvas::Canvas;
use crate::state::dot::Dot;
use crate::state::user::User;
use crate::ui::notifications::notification_tray::NotificationFlag::*;
use quad_net::web_socket::WebSocket;
use std::str::from_utf8;

pub async fn get(
    socket: &mut WebSocket,
    user: &User,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!("{} {} {}", "GET", user.room, user.apikey);
    socket.send_text(&request);
    Ok(String::from("GET Sent!"))
}

pub async fn put(
    cache: &Vec<Dot>,
    ct: &mut i32,
    socket: &mut WebSocket,
    user: &User,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!(
        "{} {} {} {}",
        "PUT",
        user.room,
        user.apikey,
        nanoserde::SerJson::serialize_json(cache)
    );
    socket.send_text(&request);

    *ct = 0;
    Ok(String::from("PUT Sent!"))
}

pub async fn delete(
    socket: &mut WebSocket,
    user: &User,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!("{} {} {}", "DEL", user.room, user.apikey,);
    socket.send_text(&request);
    Ok(String::from("DEL Sent!"))
}

pub async fn remove(
    socket: &mut WebSocket,
    user: &User,
    garbage: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!(
        "{} {} {} {}",
        "RMV",
        user.room,
        user.apikey,
        nanoserde::SerJson::serialize_json(garbage)
    );
    socket.send_text(&request);
    Ok(String::from("RMV Sent!"))
}

pub async fn web_socket_handler(socket: &mut WebSocket, canvas: &mut Canvas) {
    if let Some(res) = socket.try_recv() {
        let res_text: &str = from_utf8(&res).unwrap();
        let message: Vec<&str> = res_text.split(' ').collect();

        match message[0] {
            // Server response abstractions
            "GET_RES" => {
                println!("SERVER GET RES RECIEVED");
                let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                canvas.lines.clear();
                canvas.lines.extend(new);
                canvas.notification_flags.push(GetSuccess);
            }
            "UPD_RES" => {
                println!("SERVER UPD RES RECIEVED");
                if message[1] == canvas.user.room.to_string() {
                    let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[2]).unwrap();
                    canvas.lines.extend(new.clone());
                }
                canvas.notification_flags.push(UpdSuccess);
            }
            "CLR_RES" => {
                println!("SERVER CLR RES RECIEVED");
                canvas.lines.clear();
                canvas.notification_flags.push(ClrSuccess);
            }
            "PUT_RES" => {
                println!("SERVER PUT RES RECIEVED: {}", message[1]);
                canvas.notification_flags.push(UpdSuccess);
            }
            "DEL_RES" => {
                println!("SERVER DEL RES RECIEVED: {}", message[1]);
                canvas.notification_flags.push(DelSuccess);
            }
            "RMV_RES" => {
                println!("SERVER RMV RES RECIEVED: {}", message[1]);
                let ids: Vec<String> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                canvas.remove_dots_by_id(&ids);
                canvas.notification_flags.push(RmvSuccess);
            }
            "ERR_RES " => {
                println!("SERVER ERR RES RECIEVED: {}", message[1]);
                canvas.notification_flags.push(Fail(message[1].to_owned()));
            }
            _ => println!("UNDEFINED RES"),
        }
    }
}
