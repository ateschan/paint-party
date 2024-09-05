use crate::networking::ws::WsClient;
use crate::state::canvas::Canvas;
use crate::state::dot::Dot;
use macroquad::prelude::*;

use crate::ui::notifications::notification_tray::NotificationFlag::*;
use std::str::from_utf8;
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
        if !canvas.cache.is_empty() && !is_mouse_button_down(MouseButton::Left) {
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

        pub async fn in_handler(&mut self, canvas: &mut Canvas) {
            if let Some(res) = self.socket.try_recv() {
                let res_text: &str = from_utf8(&res).unwrap();
                let message: Vec<&str> = res_text.split(' ').collect();

                match message[0] {
                    // Server response abstractions
                    "GET_RES" => {
                        println!("SERVER GET RES RECIEVED");
                        let new: Vec<Dot> =
                            nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                        canvas.lines.clear();
                        canvas.lines.extend(new);
                        canvas.notification_flags.push(GetSuccess);
                    }
                    "UPD_RES" => {
                        println!("SERVER UPD RES RECIEVED");
                        if message[1] == self.user.room.to_string() {
                            let new: Vec<Dot> =
                                nanoserde::DeJson::deserialize_json(message[2]).unwrap();
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
                        let ids: Vec<String> =
                            nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                        canvas.remove_dots_by_id(&ids);
                        canvas.notification_flags.push(RmvSuccess);
                    }
                    //TODO
                    "INV_API" => {
                        canvas.notification_flags.push(InvApi);
                    }
                    //TODO
                    "ERR_RES " => {
                        println!("SERVER ERR RES RECIEVED: {}", message[1]);
                        canvas.notification_flags.push(Fail(message[1].to_owned()));
                    }
                    _ => println!("UNDEFINED RES"),
                }
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
