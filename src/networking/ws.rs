use super::user::User;
use crate::ui::chat::chat_tray::Chat;
use crate::ui::notifications::notification_tray::NotificationFlag;
use quad_net::web_socket::WebSocket;

//canvas_handler = canvas out
//gui_handler = gui_out
pub struct WsClient {
    pub socket: WebSocket,
    pub user: User,

    pub chats_inc: Vec<Chat>,
    pub chats_out: Vec<Chat>,

    pub notification_flags: Vec<NotificationFlag>,

    //TODO: UserCount update will go here as well
    pub players_online : u32
}
