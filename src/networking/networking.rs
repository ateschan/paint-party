use crate::state::dot::Dot;
use crate::Canvas;
use quad_net::web_socket::WebSocket;
use quad_storage::LocalStorage;
use std::str::from_utf8;
// use crate::state::particles::paint_seep;
// use macroquad_particles::{EmitterConfig, Emitter, ColorCurve};
// use macroquad::math::Vec2;

pub async fn get(
    socket: &mut WebSocket,
    storage: &mut LocalStorage,
) -> Result<String, Box<dyn std::error::Error>> {
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

pub async fn remove(
    socket: &mut WebSocket,
    storage: &mut LocalStorage,
    garbage: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let request = format!(
        "{} {} {} {}",
        "RMV",
        storage.get("room").unwrap().parse::<i32>().unwrap(),
        &storage.get("apikey").unwrap(),
        nanoserde::SerJson::serialize_json(garbage)
    );
    socket.send_text(&request);
    Ok(String::from("RMV Sent!"))
}

pub async fn web_socket_handler(
    socket: &mut WebSocket,
    canvas: &mut Canvas,
    storage: &mut LocalStorage,
) {
    if let Some(res) = socket.try_recv() {
        let res_text: &str = from_utf8(&res).unwrap();
        let message: Vec<&str> = res_text.split(' ').collect();

        match message[0] {
            // Server response abstractions
            "GET_RES" => {
                println!("SERVER GET RES RECIEVED");
                let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                //println!("THIS SHOULD BE VEC DOT {:?}", new);
                canvas.lines.clear();
                canvas.lines.extend(new);
            }
            "UPD_RES" => {
                println!("SERVER UPD RES RECIEVED");
                if message[1] == storage.get("room").unwrap() {
                    let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[2]).unwrap();
                    canvas.lines.extend(new.clone());
                    // if storage
                    //     .get("brush_particles")
                    //     .unwrap()
                    //     .parse::<bool>()
                    //     .unwrap()
                    // {
                    // for dot in new.iter() {
                    //     canvas.brush.spawn_emitter(
                    //         Emitter::new(EmitterConfig {
                    //             size: dot.size,
                    //             colors_curve: ColorCurve {
                    //                 start: macroquad::color::Color::from_rgba(
                    //                     dot.r, dot.g, dot.b, dot.a,
                    //                 ),
                    //                 mid: macroquad::color::Color::from_rgba(
                    //                     dot.r, dot.g, dot.b, dot.a,
                    //                 ),
                    //                 end: macroquad::color::Color::from_rgba(
                    //                     dot.r, dot.g, dot.b, dot.a,
                    //                 ),
                    //             },
                    //             ..paint_seep()
                    //         }),
                    //         Vec2 { x: dot.x, y: dot.y },
                    //     );
                    //     }
                    //}
                }
            }
            "CLR_RES" => {
                println!("SERVER CLR RES RECIEVED");
                canvas.lines.clear();
            }
            "PUT_RES" => println!("SERVER PUT RES RECIEVED: {}", message[1]),
            "DEL_RES" => println!("SERVER DEL RES RECIEVED: {}", message[1]),
            "RMV_RES" => {
                println!("SERVER RMV RES RECIEVED: {}", message[1]);
                let ids: Vec<String> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                remove_dots_by_id(&mut canvas.lines, &ids);
            }
            _ => println!("UNDEFINED RES"),
        }
    }
}

fn remove_dots_by_id(dots: &mut Vec<Dot>, ids_to_remove: &[String]) {
    dots.retain(|dot| !ids_to_remove.contains(&dot.id));
}
