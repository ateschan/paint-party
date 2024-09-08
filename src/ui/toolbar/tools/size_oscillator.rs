use crate::state::canvas::Canvas;
use crate::ui::toolbar::toolbar_tray::ToolbarTray;

impl ToolbarTray {
    pub fn size_oscillator(
        &mut self,
        ui: &mut egui_macroquad::egui::Ui,
        canvas: &mut Canvas,
    ) -> egui_macroquad::egui::Response {
        let result = ui.horizontal(|ui| {
            // if ui.button("o-O-o-O").on_hover_text("Oscillate Size").clicked() {
            //     canvas.brush.add_size_osc = !canvas.b
            // }
            //
            ui.vertical(|ui| {
                if ui.radio(canvas.brush.add_size_osc, "Oscillate").clicked() {
                    canvas.brush.add_size_osc = !canvas.brush.add_size_osc;
                    canvas.brush.add_mark = false;
                    canvas.brush.add_rev_mark = false;
                    println!("{:?}", canvas.brush.add_size_osc);
                }
                if ui.radio(canvas.brush.add_mark, "Mark").clicked() {
                    canvas.brush.add_mark = !canvas.brush.add_mark;
                    canvas.brush.add_rev_mark = false;
                    canvas.brush.add_size_osc = false;
                    println!("{:?}", canvas.brush.add_mark);
                }
                if ui.radio(canvas.brush.add_rev_mark, "Rev_Mark").clicked() {
                    canvas.brush.add_rev_mark = !canvas.brush.add_rev_mark;
                    canvas.brush.add_mark = false;
                    canvas.brush.add_size_osc = false;
                    println!("{:?}", canvas.brush.add_rev_mark);
                }
            });
            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(
                        &mut canvas.brush.size_osc_minmax.0,
                        1.0..=canvas.brush.size_osc_minmax.1,
                    )
                    .trailing_fill(true),
                )
                .on_hover_text("Osc Min");

                ui.add(
                    egui::Slider::new(
                        &mut canvas.brush.size_osc_minmax.1,
                        canvas.brush.size_osc_minmax.0..=600.0,
                    )
                    .trailing_fill(true),
                )
                .on_hover_text("Osc Max");

                ui.add(
                    egui::Slider::new(&mut canvas.brush.size_osc_speed, 0.001..=5.0)
                        .trailing_fill(true),
                )
                .on_hover_text("Osc Speed");
            });
        });

        result.response
    }
}
