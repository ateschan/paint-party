use ::egui::debug_text::print;
use macroquad::prelude::*;
use std::vec::Vec;
use reqwest;
use egui_macroquad::egui;

pub mod state;
use state::brush::{Brush, Dot};

static mut BRUSH : Brush = Brush {
    r : 1.0,
    g : 1.0,
    b : 1.0
};

#[macroquad::main("Paint Party")]
async fn main() {
    let mut lines : Vec<Dot> = Vec::new();
    let mut ct = 0;
    loop {
        if ct == 600{
            //lines = request_exchange(&lines);
            ct = 0;
        }
    egui_macroquad::draw();
        clear_background(WHITE);
        unsafe {
        if is_mouse_button_down(MouseButton::Left) {
                let m_pos = mouse_position();
                let dot = Dot {
                x : m_pos.0,
                y : m_pos.1,
                color : macroquad::color::Color::from_rgba((BRUSH.r * 255.0) as u8, (BRUSH.g * 255.0) as u8, (BRUSH.b * 255.0) as u8, 255)
                };
            println!("{:?}", dot.color);
            lines.push(dot)
            }
        }
        render(&mut lines);
        ct+=1;
        render_gui();
        next_frame().await
    }
}



// Query timer (Push and pull)
// pub fn request_exchange(lines : &Vec<Dot>) -> Vec<Dot> {
//     let mut lines = Vec::new();
//         let resp = match reqwest::blocking::get("https://httpbin.org/ip") {
//             Ok(resp) => resp.text().unwrap(),
//             Err(err) => panic!("Error: {}", err)
//             };
//         println!("{}", resp);
//     lines
// }

pub fn render_gui(){ 
    unsafe {
    egui_macroquad::ui(|egui_ctx| {

        egui::Window::new("egui + macroquad")
        .show(egui_ctx, |ui| {
            if ui.ui_contains_pointer() {
            color = []
            ui.label("Pick a color:");
            ui.color_edit_button_rgb(&mut color);
            if ui.button("SWAP").clicked() {
                BRUSH.swapcolor(color);
            }
                    }


            // println!{"{}", color[0]}
            
            });
        });
    egui_macroquad::draw();
    }
}

pub fn render(lines : &mut Vec<Dot>){
    for circle in lines.iter_mut(){
        draw_circle(circle.x, circle.y, 5.0, circle.color);
    }
}
