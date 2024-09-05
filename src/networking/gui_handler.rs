use crate::networking::ws::WsClient;
use crate::ui::chat::chat_tray::Chat;

impl WsClient {
    pub async fn gui_chat(&self, msg: &Vec<Chat>) -> Result<String, Box<dyn std::error::Error>> {
        //This will be run after cooldown
        for chat in msg {
            let request = format!(
                "{} {} {} {}",
                "CHT",
                //Sending out room in case of local chat room
                self.user.room,
                self.user.apikey,
                nanoserde::SerJson::serialize_json(&chat.message)
            );
            self.socket.send_text(&request);
        }
        Ok(String::from("RMV Sent!"))
    }
}
