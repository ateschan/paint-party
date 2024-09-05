use crate::networking::ws::WsClient;
use crate::state::canvas::Canvas;
use crate::state::dot::Dot;
use crate::ui::chat::chat_tray::Chat;
use crate::ui::notifications::notification_tray::NotificationFlag::*;
use std::str::from_utf8;

impl WsClient {
    pub async fn in_handler(&mut self, canvas: &mut Canvas) {
        if let Some(res) = self.socket.try_recv() {
            let res_text: &str = from_utf8(&res).unwrap();
            let message: Vec<&str> = res_text.split(' ').collect();

            match message[0] {
                // Server response abstractions
                "GET_RES" => {
                    println!("SERVER GET RES RECIEVED");
                    let new: Vec<Dot> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                    canvas.lines.clear();
                    canvas.lines.extend(new);
                    self.notification_flags.push(GetSuccess);
                }
                "UPD_RES" => {
                    println!("SERVER UPD RES RECIEVED");
                    if message[1] == self.user.room.to_string() {
                        let new: Vec<Dot> =
                            nanoserde::DeJson::deserialize_json(message[2]).unwrap();
                        canvas.lines.extend(new.clone());
                    }
                    self.notification_flags.push(UpdSuccess);
                }
                "CLR_RES" => {
                    println!("SERVER CLR RES RECIEVED");
                    canvas.lines.clear();
                    self.notification_flags.push(ClrSuccess);
                }
                "PUT_RES" => {
                    println!("SERVER PUT RES RECIEVED: {}", message[1]);
                    self.notification_flags.push(UpdSuccess);
                }
                "DEL_RES" => {
                    println!("SERVER DEL RES RECIEVED: {}", message[1]);
                    self.notification_flags.push(DelSuccess);
                }
                "RMV_RES" => {
                    println!("SERVER RMV RES RECIEVED: {}", message[1]);
                    let ids: Vec<String> = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                    canvas.remove_dots_by_id(&ids);
                    self.notification_flags.push(RmvSuccess);
                }
                //TODO
                "INV_API" => {
                    self.notification_flags.push(InvApi);
                }
                //TODO
                //
                //Take in chats one by one
                "CHT_RES" => {
                    let chat: Chat = nanoserde::DeJson::deserialize_json(message[1]).unwrap();
                    self.chats_inc.push(chat);
                }

                "ERR_RES " => {
                    println!("SERVER ERR RES RECIEVED: {}", message[1]);
                    self.notification_flags.push(Fail(message[1].to_owned()));
                }

                "CHT_SELF_RES" => {
                    self.notification_flags.push(ChtSuccess);
                }
                _ => println!("UNDEFINED RES {:?}", message),
            }
        }
    }
}
