use macroquad::prelude::*;
use std::vec::Vec;
use egui_macroquad::egui;
use std::collections::HashSet;

pub mod state;
use state::brush::{Brush, Dot};

static mut BRUSH : Brush = Brush {
    r : 0.37,
    g : 0.80,
    b : 1.0,
    size : 5.0,
    sw : true,
    room : 0000
};

fn get_unique_dots(dots: &mut Vec<Dot>) ->  Vec<Dot> {
    let mut result = Vec::new();
    for dot in dots {
        if !result.contains(dot) {
            result.push(dot.clone());
        }
    }
    result
}

#[macroquad::main("Paint Party")]
async fn main() {
    let mut lines : Vec<Dot> = Vec::new();
    let mut cache : Vec<Dot> = Vec::new();
    for val in get(&mut cache).await {
        lines.push(val);
    }
    let mut ct = 0;
    loop {
        if ct > 99 && (cache.len() > 0){
            for val in get(&mut get_unique_dots(&mut cache)).await {
                lines.push(val);
            }
            put(&mut lines, &mut cache).await;
            ct = 0;
        }
        while lines.len() > 1000 {
            lines.pop().unwrap();
        }
        egui_macroquad::draw();
        clear_background(WHITE);
        render_paint(&mut lines);
        render_paint(&mut cache);
        

        unsafe {
            if is_mouse_button_down(MouseButton::Left) && BRUSH.sw == true {
                let dot = Dot {
                    x : mouse_position().0,
                    y : mouse_position().1,
                    r : BRUSH.r,
                    g : BRUSH.g,
                    b : BRUSH.b,
                    size : BRUSH.size,
                    };
                cache.push(dot);
                }
                draw_circle(mouse_position().0, mouse_position().1, BRUSH.size, macroquad::color::Color::from_rgba((BRUSH.r * 255.0) as u8, (BRUSH.g * 255.0) as u8, (BRUSH.b * 255.0) as u8, 255)
            );
        }

        println!("{:?}", cache);
        ct+=1; 
        render_gui(&mut lines);
        next_frame().await
    }
}



//Query timer (Push and pull)
pub async fn get(lines : &mut Vec<Dot>) -> Vec<Dot> {
        unsafe{
        let client = reqwest::blocking::Client::new();

        let resp = match reqwest::blocking::get("http://127.0.0.1:8000/".to_owned() + &BRUSH.room.to_string()) {
            Ok(resp) => resp.text().unwrap(),
            Err(err) => "Error: {}".to_owned() + &err.to_string()
            };

        match serde_json::from_str::<Vec<Dot>>(&resp) {
            Ok(vec) => {
                println!("RETREIVED");
                for item in vec {
                    lines.push(item)
                }
            },
            Err(e) => {
                //panic!("{:?}{:?}", resp, e);
                return lines.clone();
            }
        }
        lines.clone()
    }
}

pub async fn put(lines : &mut Vec<Dot>, cache : &mut Vec<Dot>){
    *cache = Vec::new();
    unsafe {
        let client = reqwest::blocking::Client::new();
        if lines.len() > 0 {
            let res = client.post("http://127.0.0.1:8000/".to_owned() + &BRUSH.room.to_string())
            .json(lines).send();
        }
    }
}




pub fn render_gui(lines : &mut Vec<Dot>){ 
    unsafe {
    egui_macroquad::ui(|egui_ctx| {

        egui::Window::new("tooltip")
        .show(egui_ctx, |ui| {
            BRUSH.sw = match egui_ctx.is_pointer_over_area() {
                        true => false,
                        false => true
                };
            let mut color = [BRUSH.r, BRUSH.g, BRUSH.b];
            let roomnumber = ui.add(egui::DragValue::new(&mut BRUSH.room).speed(0.5).clamp_range(0.0..=100.0));
            let slider = ui.add(egui::Slider::new(&mut BRUSH.size, 0.0..=100.0));
            slider.on_hover_text("Drag me!");
            ui.color_edit_button_rgb(&mut color);
            BRUSH = BRUSH.swapcolor(color);

        });
    });
    egui_macroquad::draw();
    }
}

pub fn render_paint(lines : &mut Vec<Dot>){
    for circle in lines.iter_mut(){
        draw_circle(circle.x, circle.y, circle.size, macroquad::color::Color::from_rgba((circle.r * 255.0) as u8, (circle.g * 255.0) as u8, (circle.b * 255.0) as u8, 255));
        }
}
