use macroquad::prelude::*;
use rand::gen_range;
use BrushState::Paintbrush;

//Brush handles what lies behind the cursor, paint color, and particles
#[derive(Clone, Debug, PartialEq)]
pub enum BrushState {
    Off,
    Paintbrush,
    Eraser,
}

//COLOR SIZE
pub struct Brush {

    pub pos : (f32, f32),
    pub pos_last : (f32, f32),
    pub active : bool,

    pub size: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,

    pub hamper_self: bool, // For ui
    pub hamper_particles: bool,
    pub state: BrushState,

    //Size Osc
    pub size_osc_minmax: (f32, f32),
    pub size_osc_speed: f32,
    pub size_osc_goingup: bool,
    pub add_mark: bool,
    pub add_rev_mark: bool,
    pub add_size_osc: bool,
    pub mark_cease: bool, // For size osc mark

    //Chromatic mod
    pub add_cmodulate: bool,
    pub r_speed: u8,
    pub r_minmax: (u8, u8),
    pub r_goingup: bool,
    pub g_speed: u8,
    pub g_minmax: (u8, u8),
    pub g_goingup: bool,
    pub b_speed: u8,
    pub b_minmax: (u8, u8),
    pub b_goingup: bool,
    pub a_speed: u8,
    pub a_minmax: (u8, u8),
    pub a_goingup: bool,


    //Etch static
    pub is_using_mouse : bool,

    //Etch Fluid

    //Eraser
    pub rot: f32,
}

impl Default for Brush {
    fn default() -> Self {
        Brush {
            pos : mouse_position(),
            pos_last : (0.0,0.0),

            active : false,

            size: gen_range(15.0, 300.0),
            r: gen_range(0, 255),
            g: gen_range(0, 255),
            b: gen_range(0, 255),
            a: gen_range(0, 255),
            hamper_self: false,
            hamper_particles: false,
            state: Paintbrush,

            //Size Osc
            size_osc_minmax: (1.0, 15.0),
            size_osc_speed: 1.0,
            size_osc_goingup: false,
            add_mark: false,
            add_rev_mark: false,
            add_size_osc: false,
            mark_cease: false,

            //Chromatic mod
            add_cmodulate: false,
            r_speed: 0,
            r_minmax: (0, 255),
            r_goingup: false,
            g_speed: 0,
            g_minmax: (0, 255),
            g_goingup: false,
            b_speed: 0,
            b_minmax: (0, 255),
            b_goingup: false,
            a_speed: 0,
            a_minmax: (0, 255),
            a_goingup: false,

            //Etch static
            is_using_mouse : true,

            //Etch Fluid

            rot: 0.0,
        }
    }
}

impl Brush {
    pub fn rotation_update(&mut self, num: f32) {
        if self.rot <= 360.0 {
            self.rot += num;
        } else {
            self.rot = 0.0;
        }
    }
}
