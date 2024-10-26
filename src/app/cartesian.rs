use eframe::egui::{self, Color32, Pos2};

#[allow(dead_code)]
pub struct Cartesian {
    expression: String,
    zoom: f32,
    pan: Pos2,
    axis_color: Color32,
    grid_color: Color32,
    function_color: Color32,
}

impl Cartesian {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.group(|ui| {
                ui.label("Expression:");
                ui.text_edit_singleline(&mut self.expression);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.min_rect();
            self.pan += ui.interact(rect, ui.id(), egui::Sense::drag()).drag_delta();
            ui.input(|i| {
                self.zoom *= 1.0 + i.smooth_scroll_delta.y * 0.01;
                self.zoom = self.zoom.clamp(0.1, 10.0);
            });

            self.draw_grid(ui, rect);
        });
    }

    fn draw_grid(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let grid_spacing = 40.0 * self.zoom;
        let center: Pos2 = rect.center() + self.pan.to_vec2();

        let mut x = center.x % grid_spacing;
        while x < rect.right() {
            ui.painter().line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(1.0, self.grid_color),
            );
            x += grid_spacing;
        }

        let mut y = center.y % grid_spacing;
        while y < rect.bottom() {
            ui.painter().line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                egui::Stroke::new(1.0, self.grid_color),
            );
            y += grid_spacing;
        }

        ui.painter().line_segment(
            [
                egui::pos2(rect.left(), center.y),
                egui::pos2(rect.right(), center.y),
            ],
            egui::Stroke::new(2.0, self.axis_color),
        );
        ui.painter().line_segment(
            [
                egui::pos2(center.x, rect.top()),
                egui::pos2(center.x, rect.bottom()),
            ],
            egui::Stroke::new(2.0, self.axis_color),
        );
    }
}

impl Default for Cartesian {
    fn default() -> Self {
        Self {
            expression: String::new(),
            zoom: 1.0,
            pan: Pos2::ZERO,
            axis_color: Color32::WHITE,
            grid_color: Color32::from_gray(100),
            function_color: Color32::WHITE,
        }
    }
}
