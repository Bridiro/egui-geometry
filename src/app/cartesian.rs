use eframe::egui;

#[allow(dead_code)]
pub struct Cartesian {
    text: String,
}

impl Cartesian {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.group(|ui| {
                ui.set_height(70.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Text:");
                    ui.text_edit_singleline(&mut self.text);
                });
            });
        });
    }
}

impl Default for Cartesian {
    fn default() -> Self {
        Self {
            text: "Hello, Cartesian!".to_owned(),
        }
    }
}
