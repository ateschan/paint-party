use crate::state::canvas::Canvas;

use super::toolbar::toolbar_tray::toolbar;
use quad_storage::LocalStorage;

pub async fn render_gui(storage: &mut LocalStorage, canvas : &mut Canvas) {
    toolbar(storage, canvas);
}
