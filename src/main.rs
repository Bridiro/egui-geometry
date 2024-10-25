mod bezier;

use eframe::egui::{self, Color32, Pos2, TextEdit};
use eframe::NativeOptions;

struct BezierCurve {
    points: Vec<Pos2>,
    line_color: Color32,
    point_color: Color32,
    selected_point: Option<usize>,
    zoom: f32,
    pan: Pos2,
}

impl Default for BezierCurve {
    fn default() -> Self {
        let p0 = Pos2::new(50.0, 400.0);
        let p1 = Pos2::new(200.0, 200.0);
        let p2 = Pos2::new(350.0, 400.0);
        Self {
            points: vec![p0, p1, p2],
            line_color: Color32::WHITE,
            point_color: Color32::WHITE,
            selected_point: None,
            zoom: 1.0,
            pan: Pos2::ZERO,
        }
    }
}

impl eframe::App for BezierCurve {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.group(|ui| {
                ui.set_height(70.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Color of the Curve:");
                    ui.color_edit_button_srgba(&mut self.line_color);

                    ui.separator();

                    ui.label("Color of the Control Points:");
                    ui.color_edit_button_srgba(&mut self.point_color);

                    ui.separator();

                    ui.vertical(|ui| {
                        if ui.button("+").clicked() {
                            let half_diff = (self.points[self.points.len() - 1]
                                - self.points[self.points.len() - 2])
                                / 2.0;
                            let p = Pos2::new(
                                self.points[self.points.len() - 1].x + half_diff.x,
                                self.points[self.points.len() - 1].y + half_diff.y,
                            );
                            let p1 = Pos2::new(p.x + half_diff.x, p.y + half_diff.y);
                            self.points.push(p);
                            self.points.push(p1);
                        }

                        if ui.button("-").clicked() {
                            if (self.points.len() - 2) >= 3 {
                                self.points.pop();
                                self.points.pop();
                            }
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label("Edit Coordinates of the Control Points:");

                        if let Some(point_index) = self.selected_point {
                            let mut point = self.points[point_index];

                            ui.horizontal(|ui| {
                                ui.label("X:");
                                let mut x_str = point.x.to_string();
                                if ui.add(TextEdit::singleline(&mut x_str)).changed() {
                                    if let Ok(x) = x_str.parse() {
                                        point.x = x;
                                    }
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label("Y:");
                                let mut y_str = point.y.to_string();
                                if ui.add(TextEdit::singleline(&mut y_str)).changed() {
                                    if let Ok(y) = y_str.parse() {
                                        point.y = y;
                                    }
                                }
                            });
                        } else {
                            ui.label("Select a point to edit its coordinates.");
                        }
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.min_rect();
            if ui.interact(rect, ui.id(), egui::Sense::click()).clicked() {
                self.selected_point = None;
            }
            self.pan += ui.interact(rect, ui.id(), egui::Sense::drag()).drag_delta();
            ui.input(|i| {
                self.zoom *= 1.0 + i.smooth_scroll_delta.y * 0.01;
                self.zoom = self.zoom.clamp(0.1, 10.0);
            });

            for i in (0..self.points.len() - 2).step_by(2) {
                let mut scaled_p0 = Pos2::new(
                    self.points[i].x * self.zoom + self.pan.x,
                    self.points[i].y * self.zoom + self.pan.y,
                );
                let mut scaled_p1 = Pos2::new(
                    self.points[i + 1].x * self.zoom + self.pan.x,
                    self.points[i + 1].y * self.zoom + self.pan.y,
                );
                let mut scaled_p2 = Pos2::new(
                    self.points[i + 2].x * self.zoom + self.pan.x,
                    self.points[i + 2].y * self.zoom + self.pan.y,
                );

                if bezier::draggable_point(ui, &mut scaled_p0, self.point_color).clicked() {
                    self.selected_point = Some(0);
                }
                if bezier::draggable_point(ui, &mut scaled_p1, self.point_color).clicked() {
                    self.selected_point = Some(1);
                }
                if bezier::draggable_point(ui, &mut scaled_p2, self.point_color).clicked() {
                    self.selected_point = Some(2);
                }

                self.points[i].x = scaled_p0.x / self.zoom - self.pan.x / self.zoom;
                self.points[i].y = scaled_p0.y / self.zoom - self.pan.y / self.zoom;
                self.points[i + 1].x = scaled_p1.x / self.zoom - self.pan.x / self.zoom;
                self.points[i + 1].y = scaled_p1.y / self.zoom - self.pan.y / self.zoom;
                self.points[i + 2].x = scaled_p2.x / self.zoom - self.pan.x / self.zoom;
                self.points[i + 2].y = scaled_p2.y / self.zoom - self.pan.y / self.zoom;

                bezier::draw_bezier_curve(ui, scaled_p0, scaled_p1, scaled_p2, self.line_color);
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = NativeOptions::default();
    eframe::run_native(
        "Bezier Curve",
        options,
        Box::new(|_cc| Ok(Box::<BezierCurve>::default())),
    )
}
