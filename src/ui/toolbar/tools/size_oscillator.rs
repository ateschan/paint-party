
use crate::ui::toolbar::toolbar_tray::ToolbarTray;
use crate::state::canvas::Canvas;

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

            if ui.radio(canvas.brush.add_size_osc, "Oscillate Size").clicked() {
                canvas.brush.add_size_osc = !canvas.brush.add_size_osc; 
                println!("{:?}", canvas.brush.add_size_osc);
            }

            ui.add(
                egui::Slider::new(&mut self.size_osc_minmax.0, 1.0..=self.size_osc_minmax.1).trailing_fill(true),
            )
            .on_hover_text("Osc Min");

            ui.add(
                egui::Slider::new(&mut self.size_osc_minmax.1,self.size_osc_minmax.0..=600.0).trailing_fill(true),
            )
            .on_hover_text("Osc Max");

            ui.add(
                egui::Slider::new(&mut self.size_osc_speed, 0.001..=5.0).trailing_fill(true),
            )
            .on_hover_text("Osc Speed");


        });

        if canvas.brush.add_size_osc {
            self.size_oscillate(canvas);
        }

        result.response
    }

    fn size_oscillate(&mut self, canvas : &mut Canvas) {
        
       if self.size_osc_goingup {
            canvas.brush.size += self.size_osc_speed;
        }
        else {
            canvas.brush.size -= self.size_osc_speed;
        }

        if canvas.brush.size <= self.size_osc_minmax.0 || canvas.brush.size >= self.size_osc_minmax.1{
            self.size_osc_goingup = !self.size_osc_goingup;
        }
    }
}
