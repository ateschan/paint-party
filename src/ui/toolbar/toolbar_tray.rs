use crate::state::canvas::Canvas;
use crate::ui::ui_driver::GuiModule;
use crate::{networking::ws::WsClient, state::brush::BrushState::*};
use async_trait::async_trait;
use egui_macroquad::egui::{self, epaint::Shadow, Color32, RichText};

#[derive(Default)]
pub struct ToolbarTray {
    pub tmp_room: i32,
    pub tmp_size: f32,

    pub refresh_flag: bool,
    pub clear_flag: bool,
}



// TODO:  ADD drop down panels for control sections
// TODO:  Add correspingng tools for it. First up: Color oscillator
//  
 
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
                    self.brush_1(ui, canvas);
                });
                ui.vertical(|ui| self.server_1(ui, wsc));
                canvas.brush.size = self.tmp_size;
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

    pub fn brush_1(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        canvas: &mut Canvas,
    ) -> egui_macroquad::egui::Response {
        let result = ui.horizontal(|ui| {
            let mut color_button: egui_macroquad::egui::Color32 =
                egui_macroquad::egui::Color32::from_rgba_unmultiplied(
                    canvas.brush.r,
                    canvas.brush.g,
                    canvas.brush.b,
                    canvas.brush.a,
                );

            ui.color_edit_button_srgba(&mut color_button)
                .on_hover_text("Change color");

            if ui.button("O").on_hover_text("Eraser").clicked() {
                canvas.brush.state = Eraser;
            }
            if ui.button("//").on_hover_text("Paintbrush").clicked() {
                canvas.brush.state = Paintbrush;
            }

            if ui
                .add(egui_macroquad::egui::SelectableLabel::new(
                    canvas.brush.hamper_particles,
                    "*",
                ))
                .on_hover_text("Particles Toggle")
                .clicked()
            {
                canvas.brush.hamper_particles = !canvas.brush.hamper_particles;
            }

            ui.add_sized(
                ui.available_size(),
                egui::Slider::new(&mut self.tmp_size, 1.0..=600.0).trailing_fill(true),
            )
            .on_hover_text("Brush Size");
            canvas.brush.r = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[0];
            canvas.brush.g = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[1];
            canvas.brush.b = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[2];
            canvas.brush.a = egui_macroquad::egui::Color32::to_srgba_unmultiplied(&color_button)[3];
        });
        result.response
    }

    fn server_1(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        wsc: &mut WsClient,
    ) -> egui_macroquad::egui::Response {
        let result = ui.horizontal(|ui| {
            if ui
                .add(
                    egui_macroquad::egui::DragValue::new(&mut self.tmp_room)
                        .update_while_editing(false)
                        .speed(1.00)
                        .clamp_range(0.0..=9999.0),
                )
                .on_hover_text("Server room")
                .on_hover_cursor(egui::CursorIcon::Default)
                .lost_focus()
                && ui.input(|i| i.key_pressed(egui_macroquad::egui::Key::Enter))
            {
                wsc.user.room = self.tmp_room;
                self.refresh_flag = true;
            }
            if ui.button("CLEAR").on_hover_text("Erase All").clicked() {
                self.clear_flag = true;
            }
            if ui.button("↺").on_hover_text("Refresh").clicked() {
                self.refresh_flag = true;
            }

            //ui.add_sized(ui.available_size(), password(&mut self.tmp_pass));
        });
        result.response
    }

    pub fn mouse_state(&mut self, egui_ctx: &egui::Context, canvas: &mut Canvas) {
        canvas.brush.hamper_self = egui_ctx.is_using_pointer() || egui_ctx.is_pointer_over_area();
    }
}
