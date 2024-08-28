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
    pub id : String
}


pub fn init_state(storage: &mut LocalStorage) {
    storage.set("brush_r", "255");
    storage.set("brush_g", "255");
    storage.set("brush_b", "255");
    storage.set("brush_a", "255");
    storage.set("brush_size", "15.0");
    storage.set("brush_state", "true");
    storage.set("clear_local_flag", "false");
    storage.set("room", "0");
    storage.set("apikey", "");
    storage.set("socket", "");
    storage.set("refresh_flag", "false");
    storage.set("intro_complete", "false");
}
