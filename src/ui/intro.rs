use egui_macroquad::egui::{self, Align2, TextEdit, epaint::Shadow, Color32};
use macroquad::prelude::*;
use quad_storage::LocalStorage;

pub async fn render_intro(storage: &mut LocalStorage, cam: &mut Camera3D, orbit_angle: &mut f32, party_logo : &Texture2D, frame_accel : &mut f32) {
    let mut tmp_socket = storage.get("socket").unwrap();
    let cube_spin = tmp_socket.clone();
    //intro macroquad instance
    clear_background(WHITE);
    set_camera(cam);
    update_camera(cam, orbit_angle);
    let bytes: Vec<u8> = vec![255, 0, 0, 192, 0, 255, 0, 192, 0, 0, 255, 192, 255, 255, 255, 192];
    let texture = Texture2D::from_rgba8(2, 2, &bytes);
    draw_cube(vec3(0., 5., -0.), vec3(8., 8., 8.),Some(party_logo), WHITE);
    draw_cube(vec3(0., 10., -0.), vec3(9., 1., 9.),Some(&texture),WHITE);
    draw_cube(vec3(0., 0., 0.), vec3(9., 1., 9.),Some(&texture),WHITE);
    //draw_grid(40, 1., BLACK, GRAY);

    //intro gui
    egui_macroquad::ui(|egui_ctx| {
        egui::Window::new("Server")
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .anchor(Align2::CENTER_BOTTOM, [0.0, -50.0])
            .collapsible(false)
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::BLACK)),
            )
            .show(egui_ctx, |ui| {
                egui_ctx.set_visuals(egui::Visuals::light());
                ui.horizontal(|ui| {
                    ui.add(TextEdit::singleline(&mut tmp_socket)).highlight();

                    if ui
                        .add(egui_macroquad::egui::Button::new("connect"))
                        .on_hover_text("Connect to server")
                        .clicked()
                    {
                        storage.set("intro_complete", "true");
                    }
                });
            });
    });
    

    if tmp_socket.len() > cube_spin.len() {
        *frame_accel += 0.20;
    }

    if *frame_accel > 0.0 {
        *orbit_angle += *frame_accel;
        *frame_accel -= 0.02;
    }

    storage.set("socket", &tmp_socket);
    egui_macroquad::draw();
}

fn update_camera(camera: &mut Camera3D, orbit_angle: &mut f32) {
    const ORBIT_RADIUS: f32 = 20.0; // Adjust as needed

    // Calculate the new camera position based on the orbit radius and angle
    let x = ORBIT_RADIUS * orbit_angle.cos();
    let z = ORBIT_RADIUS * orbit_angle.sin();

    let new_position = vec3(-x, 15.0, -z);

    // Set the new camera position
    camera.position = new_position;
    if *orbit_angle == 360.0 {
        *orbit_angle = 0.0;
    }

    // Increment the orbit angle for the next frame
     // Adjust the increment rate as needed
}
