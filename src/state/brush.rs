use macroquad::prelude::*;
use nanoserde::{DeJson, SerJson};
use quad_storage::LocalStorage;

#[derive(Clone, Debug, SerJson, DeJson, PartialEq)]
pub struct Dot {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub size: f32,
    pub id: String,
}

pub fn init_state(storage: &mut LocalStorage) {
    //Brush
    storage.set("brush_r", "255");
    storage.set("brush_g", "255");
    storage.set("brush_b", "255");
    storage.set("brush_a", "255");
    storage.set("brush_size", "15.0");

    //On Off Erase
    storage.set("brush_state", "On");
    storage.set("brush_hamper", "true");

    //Networking
    storage.set("room", "0");
    storage.set("apikey", "");
    storage.set("socket", "");

    //State flags
    storage.set("clear_local_flag", "false");
    storage.set("refresh_flag", "false");
    storage.set("intro_complete_flag", "false");
}

