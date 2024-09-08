use std::borrow::{Borrow, BorrowMut};

use crate::state::canvas;
use crate::state::{brush::BrushState, canvas::Canvas};
use crate::ui::ui_driver::GuiModule;
use crate::networking::ws::WsClient;
use async_trait::async_trait;
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};

#[derive(Default)]
pub struct ToolbarTray {
    pub tmp_room: u16,
    pub tmp_size: f32,

    //Color oscillator r, g, b, speeds, r, g, b, ranges
    //
    //Size oscillator min, max, speed
    pub size_osc_minmax : (f32, f32),
    pub size_osc_speed : f32,
    pub size_osc_goingup : bool,

    //Server util
    pub refresh_flag: bool,
    pub clear_flag: bool,
}



// TODO:  Add correspingng tools for it. First up: Color oscillator
 
#[async_trait]
impl GuiModule for ToolbarTray {
    fn render(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas, wsc: &mut WsClient) {
        egui::Window::new(RichText::new("Toolbar"))
            .resizable(false)
            .default_pos([150.0, 10.0])
            .frame(
                egui::Frame::default()
                    .inner_margin(4.0)
                    .shadow(Shadow::NONE)
                    .fill(Color32::TRANSPARENT)
                    .stroke(egui_macroquad::egui::Stroke::new(1.0, Color32::TRANSPARENT)),
            )
            .show(egui_ctx, |ui| {
                egui_ctx.set_visuals(egui::Visuals::light());
                self.init(canvas, wsc);
                self.mouse_state(egui_ctx, canvas);

                ui.vertical(|ui| {
                    ui.collapsing("PAINT_BASIC",|ui| {
                        self.toolset_1(ui, canvas);
                    });
                    ui.collapsing("SIZE_OSCILLATOR",|ui| {
                        self.size_oscillator(ui, canvas);
                    });
                    ui.collapsing("SERVER_UTILS",|ui| {
                        self.server_util(ui, wsc);
                        canvas.brush.size = self.tmp_size;
                    });
                });
            });
        if self.clear_flag {
            canvas.lines.clear();
        }
    }
    async fn handle_ws(&mut self, wsc: &mut WsClient) {
        if self.refresh_flag {
            wsc.user.room = self.tmp_room;
            match wsc.canvas_get().await {
                Ok(a) => println!("{a}"),
                Err(e) => panic!("{}", e),
            }
            self.refresh_flag = false;
        }
        if self.clear_flag {
            wsc.user.room = self.tmp_room;
            match wsc.canvas_delete().await {
                Ok(a) => println!("{a}"),
                Err(e) => panic!("{}", e),
            }
            self.clear_flag = false;
        }
    }
}

impl ToolbarTray {
    fn init(&mut self, canvas: &mut Canvas, wsc: &mut WsClient) {
        self.tmp_room = wsc.user.room;
        self.tmp_size = canvas.brush.size;
    }
}

    pub fn swap_brush_state(canvas : &mut Canvas, state : BrushState){
        canvas.brush.state = state;
    }
