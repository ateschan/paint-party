use crate::networking::ws::WsClient;
use crate::state::user::User;
use crate::ui::chat::chat_tray::Chat;

//TODO: REDO TO INCORPORATE GUI OBJECT RATHER THAN CANVAS


impl WsClient {
    pub async fn gui_chat(
        &self,
        user: &User,
        msg: &Vec<Chat>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        for chat in msg {
            let request = format!(
                "{} {} {} {}",
                "CHT",
                //Sending out room in case of local chat room
                user.room,
                user.apikey,
                nanoserde::SerJson::serialize_json(&chat.message)
            );
            self.socket.send_text(&request);
        }
        Ok(String::from("RMV Sent!"))
    }
}
