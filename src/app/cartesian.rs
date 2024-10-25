use eframe::egui::{self, Color32, Pos2};
use exmex::prelude::*;

pub struct Cartesian {
    expression: String,
    zoom: f32,
    pan: Pos2,
    axis_color: Color32,
    grid_color: Color32,
    function_color: Color32,
    pub switch: bool,
}

impl Cartesian {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.group(|ui| {
                ui.set_height(45.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.vertical(|ui| {
                        ui.set_width(200.0);
                        ui.label("Expression:");
                        ui.text_edit_singleline(&mut self.expression);
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label("Axis color:");
                        ui.color_edit_button_srgba(&mut self.axis_color);
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label("Grid color:");
                        ui.color_edit_button_srgba(&mut self.grid_color);
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label("Function color:");
                        ui.color_edit_button_srgba(&mut self.function_color);
                    });

                    ui.separator();

                    if ui
                        .add_sized([100.0, 10.0], egui::Button::new("Reset"))
                        .on_hover_text("Reset the view")
                        .clicked()
                    {
                        self.zoom = 1.0;
                        self.pan = Pos2::ZERO;
                    }

                    if ui
                        .add_sized([100.0, 10.0], egui::Button::new("Bezier"))
                        .on_hover_text("Switch to Bezier curve")
                        .clicked()
                    {
                        self.switch = true;
                    }
                });
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
            self.draw_function(ui, rect);
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

    fn draw_function(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let grid_unit = 40.0;

        let start_x = rect.left();
        let end_x = rect.right();

        let mut last_pos = None;

        for screen_x in (start_x as i32)..(end_x as i32) {
            let world_x = ((screen_x as f32 - center_x - self.pan.x) / self.zoom) / grid_unit;

            if let Some(world_y) = self.evaluate_expression(world_x as f64) {
                let screen_y = center_y - (world_y as f32 * self.zoom * grid_unit - self.pan.y);
                let pos = Pos2::new(screen_x as f32, screen_y);

                if let Some(last) = last_pos {
                    ui.painter()
                        .line_segment([last, pos], egui::Stroke::new(1.0, self.function_color));
                }

                last_pos = Some(pos);
            } else {
                last_pos = None;
            }
        }
    }

    fn evaluate_expression(&self, x: f64) -> Option<f64> {
        let expr = exmex::parse::<f64>(&self.expression).ok()?;
        if let Ok(result) = expr.eval(&[x]) {
            Some(result)
        } else {
            None
        }
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
            switch: false,
        }
    }
}
