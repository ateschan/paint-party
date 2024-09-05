use crate::state::user::User;
use quad_net::web_socket::WebSocket;

//TODO: I need definiteions for canvas_out(var)
//TODO: I need definiteions for canvas_in(var)
//TODO: I need definiteions for gui_out(var)

pub struct WsClient {
    pub socket: WebSocket,
    pub user: User,
}
