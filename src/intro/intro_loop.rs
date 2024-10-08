use super::password::password;
use crate::intro::socket::socket;
use egui_macroquad::egui::{self, epaint::Shadow, Align2, Color32};
use macroquad::prelude::*;
use quad_storage::LocalStorage;

//Intro screen to enter the websocket address
//
//
pub async fn enter_intro(storage: &mut LocalStorage) {
    let mut cam = Camera3D {
        position: vec3(-20., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };

    let mut orbit_angle: f32 = 0.0;
    let party_logo = load_texture("assets/party.png").await.unwrap();
    let mut frame_accel = 0.3;

    request_new_screen_size(1920.0, 1080.0);

    storage.set("intro_complete_flag", "false");

    match storage.get("socket") {
        Some(_) => {}
        None => {
            storage.set("socket", "");
        }
    }

    match storage.get("apikey") {
        Some(_) => {}
        None => {
            storage.set("apikey", "");
        }
    }

    loop {
        render_intro(
            storage,
            &mut cam,
            &mut orbit_angle,
            &party_logo,
            &mut frame_accel,
        )
        .await;

        if storage
            .get("intro_complete_flag")
            .unwrap()
            .parse::<bool>()
            .unwrap()
        {
            break;
        }
        next_frame().await
    }
}

pub async fn render_intro(
    storage: &mut LocalStorage,
    cam: &mut Camera3D,
    orbit_angle: &mut f32,
    party_logo: &Texture2D,
    frame_accel: &mut f32,
) {
    let mut tmp_socket = storage.get("socket").unwrap();
    let mut tmp_pass = storage.get("apikey").unwrap();
    let cube_spin = tmp_socket.clone();

    clear_background(WHITE);
    set_camera(cam);
    update_camera(cam, orbit_angle);

    //let bytes: Vec<u8> = vec![255, 0, 0, 192, 0, 255, 0, 192, 0, 0, 255, 192, 255, 255, 255, 192];
    //let texture = Texture2D::from_rgba8(2, 2, &bytes);

    draw_cube(
        vec3(0., 5., -0.),
        vec3(10., 10., 10.),
        Some(party_logo),
        WHITE,
    );
    //draw_cube(vec3(0., 10., -0.), vec3(9., 1., 9.),Some(&texture),WHITE);
    //draw_cube(vec3(0., 0., 0.), vec3(9., 1., 9.),Some(&texture),WHITE);
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
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::TRANSPARENT)),
            )
            .show(egui_ctx, |ui| {
                egui_ctx.set_visuals(egui::Visuals::light());
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add(socket(&mut tmp_socket, storage)).highlight();
                        ui.add_space(8.0);
                        ui.add(password(&mut tmp_pass)).highlight();
                    });
                });
            });
    });

    if tmp_socket.len() > cube_spin.len() {
        *frame_accel += 0.20;
    }

    if tmp_socket.len() < cube_spin.len() {
        *frame_accel -= 0.20;
    }

    if *frame_accel < 0.0 {
        *orbit_angle -= *frame_accel;
        *frame_accel += 0.02;
    }

    if *frame_accel > 0.0 {
        *orbit_angle += *frame_accel;
        *frame_accel -= 0.02;
    }

    storage.set("socket", &tmp_socket);
    storage.set("apikey", &tmp_pass);
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
}
