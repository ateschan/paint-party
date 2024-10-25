//TODO:
//FLUID FUNCTION    <-> 0 <->
//
use super::super::brush::BrushState::*;
use crate::state::brush;
use crate::state::canvas::Canvas;
use crate::state::dot::Dot;
use macroquad::prelude::*;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct BeamNode {
    vel_x: f32,
    vel_y: f32,
    x: f32,
    y: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    size: f32,
    id: String,
}

impl BeamNode {
    pub fn render(&self) {
        draw_circle(
            self.x,
            self.y,
            self.size,
            macroquad::color::Color::from_rgba(self.r, self.g, self.b, self.a),
        );
    }
}

impl Canvas {
    pub async fn render_tractor_beam_idle(&mut self) {
        let dot = Dot {
            x: self.brush.pos.0,
            y: self.brush.pos.1,
            r: self.brush.r,
            g: self.brush.g,
            b: self.brush.b,
            a: self.brush.a,
            size: self.brush.size,
            ..Dot::default()
        };

        if self.brush.beam_rope_toggle {
            let overlapping = self.is_overlapping(&dot);
            for dot in self.lines.iter() {
                if overlapping.contains(&dot.id) {
                    draw_line(
                        dot.x,
                        dot.y,
                        self.brush.pos.0,
                        self.brush.pos.1,
                        3.0,
                        Color::from_rgba(dot.r, dot.g, dot.b, dot.a),
                    );
                }
            }
        }
        draw_poly_lines(
            self.brush.pos.0,
            self.brush.pos.1,
            50,
            self.brush.size,
            self.brush.rot,
            5.0,
            macroquad::color::Color::from_rgba(
                self.brush.r,
                self.brush.g,
                self.brush.b,
                self.brush.a,
            ),
        );
    }

    //For each node in beam nodes render out node
    pub async fn render_tractor_beam_active(&self) {
        draw_poly_lines(
            self.brush.pos.0,
            self.brush.pos.1,
            50,
            self.brush.size,
            self.brush.rot,
            5.0,
            macroquad::color::Color::from_rgba(
                self.brush.r,
                self.brush.g,
                self.brush.b,
                self.brush.a,
            ),
        );
    }

    //FLUID like orbit, but with no radius lock, accelerating torwards the center at a vel of x,y
    //flipping signs as needed
    pub async fn tractor_beam(&mut self) {
        if !self.brush.beam_cache.is_empty() && self.brush.beam_nodes.is_empty() {
            if self.brush.state == TractorCopy {
                self.copy_dots_by_id_into_beam_nodes(&self.brush.beam_cache.clone());
            }
            //Default implementation, removes dots converts to nodes. WS utility is handled in
            //canvas
            else {
                self.remove_dots_by_id_into_beam_nodes(&self.brush.beam_cache.clone());
            }
        } else {
            let dot = Dot {
                x: self.brush.pos.0,
                y: self.brush.pos.1,
                r: self.brush.r,
                g: self.brush.g,
                b: self.brush.b,
                a: self.brush.a,
                size: self.brush.size,
                ..Dot::default()
            };
            let overlapping = self.is_overlapping(&dot);
            for dot in self.lines.iter() {
                if overlapping.contains(&dot.id) {
                    self.brush.beam_cache.push(dot.id.clone());
                }
            }
        }

        for node in self.brush.beam_nodes.iter_mut() {
            //Magnet nodes are NOT dependent on cursor
            //TODO: Magnet needs improving
            if self.brush.state == TractorMutate {
                let distance =
                    (node.x - self.brush.pos.0).powf(2.0) + (node.y - self.brush.pos.1).powf(2.0) - 10.;
                    node.vel_x += (self.brush.pos.0 - node.x) / 300.0 * self.brush.tractor_vel_x;
                    node.vel_y += (self.brush.pos.1 - node.y) / 300.0 * self.brush.tractor_vel_y;
                    node.y += node.vel_y;
                    node.x += node.vel_x;
                
                //This value needs to scale with distance from the center
                // node.vel_y *= 0.95;
                // node.vel_x *= 0.95;
            } 
            else {
                node.x += self.brush.pos.0 - self.brush.pos_last.0;
                node.y += self.brush.pos.1 - self.brush.pos_last.1;
            }

            if self.brush.state == TractorFluid {
                let distance = (node.x - self.brush.pos.0).powf(2.0) + (node.y - self.brush.pos.1).powf(2.0) - 10.;
                // if (self.brush.pos.0.abs() - self.brush.pos_last.0.abs()) != 0.0 && (self.brush.pos.1.abs() - self.brush.pos_last.1.abs()) != 0.0 {
                // node.vel_x += (self.brush.pos.0 - self.brush.pos_last.0)/ 100.;
                // node.vel_y += (self.brush.pos.1 - self.brush.pos_last.1)/ 100.;
                // }
                if distance < (self.brush.size).powf(2.0)  {
                    node.vel_x += (self.brush.pos.0 - self.brush.pos_last.0) /6. ;
                    node.vel_y += (self.brush.pos.1 - self.brush.pos_last.1) /6.;
                    node.x += node.vel_x;
                    node.y += node.vel_y;
                }

                else {
                    // node.x -= self.brush.tractor_vel_x;
                    // node.y -= self.brush.tractor_vel_y;
                    node.vel_y = -node.vel_y ;
                    node.vel_x = -node.vel_x;
                    node.x += node.vel_x * 1.10;
                    node.y += node.vel_y * 1.10;
                }
                node.vel_y *= 0.95;
                node.vel_x *= 0.95;
            }

            if self.brush.state == TractorOrbit {
                rotate_point(self.brush.pos, node);
            }
        }
    }

    //UTIL
    //-------------------------------------------------------------------------------------------------------------
    pub async fn beam_recover(&mut self) {
        let mut beambuilder: Vec<Dot> = Vec::new();
        let mut beam_garbage_builder: Vec<String> = Vec::new();
        for node in self.brush.beam_nodes.iter_mut() {
            let dot = Dot {
                x: node.x,
                y: node.y,
                r: node.r,
                g: node.g,
                b: node.b,
                a: node.a,
                size: node.size,
                id: node.id.clone(),
            };
            beambuilder.push(dot.clone());
            beam_garbage_builder.push(dot.id);
        }

        self.cache.extend(beambuilder);
        if self.brush.state == TractorCut {
            self.garbage.extend(beam_garbage_builder);
        }
        self.brush.beam_cache = Vec::new();
        self.brush.beam_nodes = Vec::new();
    }

    //removes dot from canvas adds to beamnodes
    pub fn remove_dots_by_id_into_beam_nodes(&mut self, ids_to_remove: &[String]) {
        self.lines.retain(|dot| {
            if !ids_to_remove.contains(&dot.id) {
                true
            } else {
                if self.brush.state == TractorFluid {
                    self.brush.beam_nodes.push(BeamNode {
                        vel_x: macroquad::rand::gen_range(-self.brush.beam_randomness, self.brush.beam_randomness) * self.brush.tractor_vel_x,
                        vel_y: macroquad::rand::gen_range(-self.brush.beam_randomness, self.brush.beam_randomness) * self.brush.tractor_vel_y,
                        x: dot.x,
                        y: dot.y,
                        r: dot.r,
                        g: dot.g,
                        b: dot.b,
                        a: dot.a,
                        size: dot.size,
                        id: dot.id.clone(),
                    });
                } 
                else {
                    self.brush.beam_nodes.push(BeamNode {
                        vel_x: self.brush.tractor_vel_x,
                        vel_y: self.brush.tractor_vel_y,
                        x: dot.x,
                        y: dot.y,
                        r: dot.r,
                        g: dot.g,
                        b: dot.b,
                        a: dot.a,
                        size: dot.size,
                        id: dot.id.clone(),
                    });
                }
                false
            }
        });
    }

    pub fn copy_dots_by_id_into_beam_nodes(&mut self, ids_to_copy: &[String]) {
        for dot in self.lines.iter_mut() {
            if ids_to_copy.contains(&dot.id) {
                self.brush.beam_nodes.push(BeamNode {
                    vel_x: 0.0,
                    vel_y: 0.0,
                    x: dot.x,
                    y: dot.y,
                    r: dot.r,
                    g: dot.g,
                    b: dot.b,
                    a: dot.a,
                    size: dot.size,
                    id: nanoid::nanoid!(),
                });
            }
        }
    }
}

fn rotate_point(pos: (f32, f32), point: &mut BeamNode) {
    let angle_radians = 2.0 * (PI / 180.0) * point.vel_y as f64 / point.vel_x as f64;

    let translated_x: f64 = (point.x - pos.0) as f64;
    let translated_y: f64 = (point.y - pos.1) as f64;

    let (rotated_x, rotated_y) = (
        translated_x * angle_radians.cos() - translated_y * angle_radians.sin(),
        translated_x * angle_radians.sin() + translated_y * angle_radians.cos(),
    );

    point.x = rotated_x as f32 + pos.0;
    point.y = rotated_y as f32 + pos.1;
}
