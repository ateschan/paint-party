use crate::state::brush::BrushState::*;
use crate::state::canvas::Canvas;
use crate::ui::toolbar::toolbar_tray::ToolbarTray;

//TODO: How do I spawn an additional window off of an already existing one?
impl ToolbarTray {
    pub fn toolset_1(
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

            if ui.button("0").on_hover_text("Eraser").clicked() {
                super::super::toolbar_tray::swap_brush_state(canvas, Eraser);
            }
            if ui.button("/..").on_hover_text("Paintbrush").clicked() {
                super::super::toolbar_tray::swap_brush_state(canvas, Paintbrush);
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

        if canvas.brush.size != self.tmp_size {
            canvas.brush.size = self.tmp_size;
        }
        result.response
    }
}
