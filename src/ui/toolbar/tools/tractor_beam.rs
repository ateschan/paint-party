//Tractor beam ui
use crate::state::brush::BrushState::*;
use crate::state::canvas::Canvas;
use crate::ui::toolbar::toolbar_tray::ToolbarTray;

impl ToolbarTray {
    pub fn tractor_beam(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        canvas: &mut Canvas,
    ) -> egui_macroquad::egui::Response {
        let result = ui.horizontal(|ui| {
            ui.vertical(|ui| {
                if ui.button("Cut").clicked() {
                    canvas.brush.state = TractorCut;
                }
                if ui.button("Copy").clicked() {
                    canvas.brush.state = TractorCopy;
                }
                if ui.button("Orbit").clicked() {
                    canvas.brush.state = TractorOrbit;
                }
                if ui.button("Magnet").clicked() {
                    canvas.brush.state = TractorMagnet;
                }
                if ui.button("Mutate").clicked() {
                    canvas.brush.state = TractorMutate;
                }
                if ui.button("Fluid").clicked() {
                    canvas.brush.state = TractorFluid;
                }
            });
            ui.vertical(|ui| {
                if ui
                    .radio(canvas.brush.beam_rope_toggle, "Tractor Ropes")
                    .clicked()
                {
                    canvas.brush.beam_rope_toggle = !canvas.brush.beam_rope_toggle;
                }

                ui.add(
                    egui::Slider::new(&mut canvas.brush.tractor_vel_x, -5.0..=5.0)
                        .trailing_fill(true),
                )
                .on_hover_text("MODIFIER X");

                ui.add(
                    egui::Slider::new(&mut canvas.brush.tractor_vel_y, -5.0..=5.0)
                        .trailing_fill(true),
                )
                .on_hover_text("MODIFIER Y");

                ui.add(
                    egui::Slider::new(&mut canvas.brush.beam_randomness, 0.0..=15.0)
                        .trailing_fill(true),
                )
                .on_hover_text("RANDOMNESS");
            });
        });
        result.response
    }
}
