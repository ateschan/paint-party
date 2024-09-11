use std::any::Any;

use crate::networking::ws::WsClient;
use crate::state::canvas::Canvas;
use crate::state::dot::Dot;
use macroquad::prelude::*;
use macroquad::ui::KeyCode;

//I feel like user and apikey should be instansiated on websockets instead of canvas

impl WsClient {
    pub async fn canvas_get(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request = format!("{} {} {}", "GET", self.user.room, self.user.apikey);
        self.socket.send_text(&request);
        Ok(String::from("GET Sent!"))
    }

    pub async fn canvas_put(
        &self,
        cache: &Vec<Dot>,
        ct: &mut i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = format!(
            "{} {} {} {}",
            "PUT",
            self.user.room,
            self.user.apikey,
            nanoserde::SerJson::serialize_json(cache)
        );
        self.socket.send_text(&request);

        *ct = 0;
        Ok(String::from("PUT Sent!"))
    }

    pub async fn canvas_delete(&self) -> Result<String, Box<dyn std::error::Error>> {
        let request = format!("{} {} {}", "DEL", self.user.room, self.user.apikey,);
        self.socket.send_text(&request);
        Ok(String::from("DEL Sent!"))
    }

    pub async fn canvas_remove(
        &self,
        garbage: &Vec<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = format!(
            "{} {} {} {}",
            "RMV",
            self.user.room,
            self.user.apikey,
            nanoserde::SerJson::serialize_json(garbage)
        );
        self.socket.send_text(&request);
        Ok(String::from("RMV Sent!"))
    }
}

impl WsClient {
    pub async fn canvas_out_handler(&self, canvas: &mut Canvas) {
        // PUT REQUEST TO WEBSOCKET
        if !canvas.cache.is_empty() && !is_key_down(miniquad::KeyCode::LeftControl) && !is_mouse_button_down(MouseButton::Left) {
            canvas.lines.extend(canvas.cache.clone());
            #[cfg(test)]
            println!("EXTENDING LINES");
            match self
                .canvas_put(&canvas.cache.clone(), &mut canvas.frame_count)
                .await
            {
                Ok(res) => {
                    println!("{:?}", res);
                    canvas.cache.clear();
                }
                Err(e) => println!("{:?}", e),
            }
            #[cfg(test)]
            println!("CLEARING CACHE");
        }
    }
}
// DEL REQUEST TO WEBSOCKET
//     if canvas.clear_flag {
//         canvas.lines = Vec::new();
//         match self.canvas_delete(&canvas.user).await {
//             #[allow(unused)]
//             Ok(l) => {
//                 #[cfg(test)]
//                 println!("{l}");
//             }
//             Err(e) => println!("ERROR {e}"),
//         }
//         canvas.clear_flag = false;
//     }

// if canvas.refresh_flag {
//     match self.canvas_get(&canvas.user).await {
//         #[allow(unused)]
//         Ok(res) => {
//             #[cfg(test)]
//             println!("{}", res)
//         }
//         Err(e) => println!("ERROR {e}"),
//     }
//     canvas.refresh_flag = false;
// }
