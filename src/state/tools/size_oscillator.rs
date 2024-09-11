use crate::state::canvas::Canvas;
use macroquad::prelude::*;

//No behavior becuase self.is mutated direclty, uses paintbrush for behavior
//SELF AS CURSOR
impl Canvas {
    //Goes from BIG --> SMALL
    pub async fn mark(&mut self) {
        if self.brush.active && !self.brush.mark_cease{
            if self.brush.size > self.brush.size_osc_minmax.0  {
               self.brush.size -= self.brush.size_osc_speed; 
            }
            else {
                self.brush.mark_cease = true;
                self.brush.size = 0.0;
            }
        }
    }

    //Goes from SMALL --> BIG
    pub async fn rev_mark(&mut self) {
        if self.brush.active && !self.brush.mark_cease{
            if self.brush.size < self.brush.size_osc_minmax.1  {
                self.brush.size += self.brush.size_osc_speed; 
            }
        else {
            self.brush.mark_cease = true;
            self.brush.size = self.brush.size_osc_minmax.0;
        }
        }
    }

    //Render BIG HOLLOW POLY AROUND SMALL SOLID POLY
    pub fn render_size_oscillator(&self) {
        draw_poly_lines(
            self.brush.pos.0,
            self.brush.pos.1,
            12,
            self.brush.size_osc_minmax.1,
            -self.brush.rot,
            5.0,
            macroquad::color::Color::from_rgba(
                255 - self.brush.r,
                255 - self.brush.g,
                255 - self.brush.b,
                self.brush.a,
            ),
        );
        draw_poly_lines(
            self.brush.pos.0,
            self.brush.pos.1,
            12,
            self.brush.size_osc_minmax.1,
            self.brush.rot,
            5.0,
            macroquad::color::Color::from_rgba(
                self.brush.r,
                self.brush.g,
                self.brush.b,
                self.brush.a,
            ),
        );
        draw_poly(
            self.brush.pos.0,
            self.brush.pos.1,
            12,
            self.brush.size_osc_minmax.0,
            self.brush.rot,
            macroquad::color::Color::from_rgba(
                255 - self.brush.r,
                255 - self.brush.g,
                255 - self.brush.b,
                self.brush.a,
            ),
        );
        draw_poly(
            self.brush.pos.0,
            self.brush.pos.1,
            12,
            self.brush.size_osc_minmax.0,
            -self.brush.rot,
            macroquad::color::Color::from_rgba(
                self.brush.r,
                self.brush.g,
                self.brush.b,
                self.brush.a,
            ),
        );
    }

    //BOOL GOINGUP true == Going bigger, false == going smaller;
    //Speed is directly assosiated with size using subtraction
    pub fn size_oscillate(&mut self) {
        if self.brush.size_osc_goingup {
            self.brush.size += self.brush.size_osc_speed;
        } else {
            self.brush.size -= self.brush.size_osc_speed;
        }
        if self.brush.size <= self.brush.size_osc_minmax.0
            || self.brush.size >= self.brush.size_osc_minmax.1
        {
            self.brush.size_osc_goingup = !self.brush.size_osc_goingup;
        }
    }
}
