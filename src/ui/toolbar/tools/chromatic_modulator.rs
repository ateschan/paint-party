use super::super::toolbar_tray::ToolbarTray;
use crate::state::canvas::Canvas;

impl ToolbarTray {
    // pub fn color_oscillator (&mut self,ui: &mut egui_macroquad::egui::Ui,canvas: &mut Canvas) -> egui_macroquad::egui::Response {
    // }
    //
    pub fn chromatic_modulator(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        canvas: &mut Canvas,
    ) -> egui_macroquad::egui::Response {
        let result = ui.horizontal(|ui| {
            // if ui.button("o-O-o-O").on_hover_text("Oscillate Size").clicked() {
            //     canvas.brush.add_size_osc = !canvas.b
            // }

            //I think I should add a bigger ring to preview
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.radio(canvas.brush.add_cmodulate, "Modulate").clicked() {
                        canvas.brush.add_cmodulate = !canvas.brush.add_cmodulate;
                    }

                });
                                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.r,
                                0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("R");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.g,
                                0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("G");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.b,
                                0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("B");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.a,
                                0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("A");
                    });
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.r_minmax.0,
                                0..=canvas.brush.r_minmax.1,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("R Min");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.r_minmax.1,
                                canvas.brush.r_minmax.0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("R Max");

                        ui.add(
                            egui::Slider::new(&mut canvas.brush.r_speed, 0..=15)
                                .trailing_fill(true),
                        )
                        .on_hover_text("R Speed");
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.g_minmax.0,
                                1..=canvas.brush.g_minmax.1,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("G Min");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.g_minmax.1,
                                canvas.brush.g_minmax.0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("G Max");

                        ui.add(
                            egui::Slider::new(&mut canvas.brush.g_speed, 0..=15)
                                .trailing_fill(true),
                        )
                        .on_hover_text("G Speed");
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.b_minmax.0,
                                1..=canvas.brush.b_minmax.1,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("B Min");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.b_minmax.1,
                                canvas.brush.b_minmax.0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("B Max");

                        ui.add(
                            egui::Slider::new(&mut canvas.brush.b_speed, 0..=15)
                                .trailing_fill(true),
                        )
                        .on_hover_text("B Speed");
                    });
                    ui.vertical(|ui| {
                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.a_minmax.0,
                                1..=canvas.brush.a_minmax.1,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("A Min");

                        ui.add(
                            egui::Slider::new(
                                &mut canvas.brush.a_minmax.1,
                                canvas.brush.a_minmax.0..=255,
                            )
                            .trailing_fill(true),
                        )
                        .on_hover_text("A Max");

                        ui.add(
                            egui::Slider::new(&mut canvas.brush.a_speed, 0..=15)
                                .trailing_fill(true),
                        )
                        .on_hover_text("A Speed");
                    });
                });
            });
        });

        result.response
    }
}

// Range r MAX
// Range g MAX
// Range b MAX
//
// Range r MIN
// Range g MIN
// Range b MIN
//
// Speed r
// Speed g
// Speed b
