use crate::state::canvas::Canvas;
use macroquad::prelude::*;

//No behavior becuase self.is mutated direclty, uses paintbrush for behavior
//SELF AS CURSOR
impl Canvas {
    pub fn r_modulate(&mut self) {
        if self.brush.r_goingup {
            let inc = self.brush.r as i32 + self.brush.r_speed as i32;
            if inc >= 255 || inc >= self.brush.r_minmax.1 as i32 {
                self.brush.r_goingup = false
            }
            else {
                self.brush.r = (inc % 256) as u8;
            }
        }
        else {
            let inc = self.brush.r as i32 - self.brush.r_speed as i32;
            if inc <= 0 || inc <= self.brush.r_minmax.0 as i32 {
                self.brush.r_goingup = true
            }
            else {
                self.brush.r = (inc % 256) as u8;
            }
        }
    }

    pub fn g_modulate(&mut self) {
        if self.brush.g_goingup {
            let inc = self.brush.g as i32 + self.brush.g_speed as i32;
            if inc >= 255 || inc >= self.brush.g_minmax.1 as i32 {
                self.brush.g_goingup = false
            }
            else {
                self.brush.g = (inc % 256) as u8;
            }
        }
        else {
            let inc = self.brush.g as i32 - self.brush.g_speed as i32;
            if inc <= 0 || inc <= self.brush.g_minmax.0 as i32 {
                self.brush.g_goingup = true
            }
            else {
                self.brush.g = (inc % 256) as u8;
            }
        }
    }

    pub fn b_modulate(&mut self) {
        if self.brush.b_goingup {
            let inc = self.brush.b as i32 + self.brush.b_speed as i32;
            if inc >= 255 || inc >= self.brush.b_minmax.1 as i32 {
                self.brush.b_goingup = false
            }
            else {
                self.brush.b = (inc % 256) as u8;
            }
        }
        else {
            let inc = self.brush.b as i32 - self.brush.b_speed as i32;
            if inc <= 0 || inc <= self.brush.b_minmax.0 as i32 {
                self.brush.b_goingup = true
            }
            else {
                self.brush.b = (inc % 256) as u8;
            }
        }
    } 

    pub fn a_modulate(&mut self) {
        if self.brush.a_goingup {
            let inc = self.brush.a as i32 + self.brush.a_speed as i32;
            if inc >= 255 || inc >= self.brush.a_minmax.1 as i32 {
                self.brush.a_goingup = false
            }
            else {
                self.brush.a = (inc % 256) as u8;
            }
        }
        else {
            let inc = self.brush.a as i32 - self.brush.a_speed as i32;
            if inc <= 0 || inc <= self.brush.a_minmax.0 as i32 {
                self.brush.a_goingup = true
            }
            else {
                self.brush.a = (inc % 256) as u8;
            }
        }
    }
}
