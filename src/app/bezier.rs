use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, TextEdit, Ui};

pub struct BezierCurve {
    points: Vec<Pos2>,
    line_color: Color32,
    point_color: Color32,
    lines_color: Color32,
    selected_point: Option<usize>,
    zoom: f32,
    pan: Pos2,
    lines_on: bool,
    points_on: bool,
    pub switch: bool,
}

impl BezierCurve {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.group(|ui| {
                ui.set_height(70.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.vertical(|ui| {
                        ui.label("Color of the Curve:");
                        ui.color_edit_button_srgba(&mut self.line_color);
                        ui.label("Color of the Control Points:");
                        ui.color_edit_button_srgba(&mut self.point_color);
                    });
                    ui.vertical(|ui| {
                        ui.label("Color of the Lines:");
                        ui.color_edit_button_srgba(&mut self.lines_color);
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        if ui
                            .add_sized([45.0, 10.0], egui::Button::new("+"))
                            .on_hover_text("Add a line segment")
                            .clicked()
                        {
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

                        if ui
                            .add_sized([45.0, 10.0], egui::Button::new("-"))
                            .on_hover_text("Remove last line segment")
                            .clicked()
                        {
                            if (self.points.len() - 2) >= 3 {
                                self.points.pop();
                                self.points.pop();
                            }
                        }

                        if ui
                            .add_sized([45.0, 10.0], egui::Button::new("Reset"))
                            .on_hover_text("Reset the control points")
                            .clicked()
                        {
                            self.points = vec![
                                Pos2::new(50.0, 400.0),
                                Pos2::new(200.0, 200.0),
                                Pos2::new(350.0, 400.0),
                            ];
                            self.zoom = 1.0;
                            self.pan = Pos2::ZERO;
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.set_width(150.0);
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

                    ui.separator();

                    ui.vertical(|ui| {
                        if ui
                            .add_sized([100.0, 10.0], egui::Button::new("Toggle Lines"))
                            .on_hover_text("Toggle the visibility of the lines.")
                            .clicked()
                        {
                            self.lines_on = !self.lines_on;
                        }

                        if ui
                            .add_sized([100.0, 10.0], egui::Button::new("Toggle Points"))
                            .on_hover_text("Toggle the visibility of the control points.")
                            .clicked()
                        {
                            self.points_on = !self.points_on;
                        }

                        if ui
                            .add_sized([100.0, 10.0], egui::Button::new("Cartesian"))
                            .on_hover_text("Switch to the Cartesian graph.")
                            .clicked()
                        {
                            self.switch = true;
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
                let mouse_pos = i.pointer.hover_pos().unwrap_or(Pos2::ZERO);
                let previous_zoom = self.zoom;

                self.zoom *= 1.0 + i.smooth_scroll_delta.y * 0.01;
                self.zoom = self.zoom.clamp(0.1, 10.0);

                let zoom_factor = self.zoom / previous_zoom;
                self.pan = mouse_pos - (mouse_pos - self.pan) * zoom_factor;
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

                if draggable_point(ui, &mut scaled_p0, self.point_color, self.points_on).clicked() {
                    self.selected_point = Some(0);
                }
                if draggable_point(ui, &mut scaled_p1, self.point_color, self.points_on).clicked() {
                    self.selected_point = Some(1);
                }
                if draggable_point(ui, &mut scaled_p2, self.point_color, self.points_on).clicked() {
                    self.selected_point = Some(2);
                }

                self.points[i].x = scaled_p0.x / self.zoom - self.pan.x / self.zoom;
                self.points[i].y = scaled_p0.y / self.zoom - self.pan.y / self.zoom;
                self.points[i + 1].x = scaled_p1.x / self.zoom - self.pan.x / self.zoom;
                self.points[i + 1].y = scaled_p1.y / self.zoom - self.pan.y / self.zoom;
                self.points[i + 2].x = scaled_p2.x / self.zoom - self.pan.x / self.zoom;
                self.points[i + 2].y = scaled_p2.y / self.zoom - self.pan.y / self.zoom;

                if self.lines_on {
                    draw_dotted_line(ui, scaled_p0, scaled_p1, self.lines_color);
                    draw_dotted_line(ui, scaled_p1, scaled_p2, self.lines_color);
                }

                draw_bezier_curve(ui, scaled_p0, scaled_p1, scaled_p2, self.line_color);
            }
        });
    }
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
            lines_color: Color32::WHITE,
            selected_point: None,
            zoom: 1.0,
            pan: Pos2::ZERO,
            lines_on: true,
            points_on: true,
            switch: false,
        }
    }
}

fn bezier_points(p0: Pos2, p1: Pos2, p2: Pos2, segments: usize) -> Vec<Pos2> {
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let x = (1.0 - t).powi(2) * p0.x + 2.0 * (1.0 - t) * t * p1.x + t.powi(2) * p2.x;
        let y = (1.0 - t).powi(2) * p0.y + 2.0 * (1.0 - t) * t * p1.y + t.powi(2) * p2.y;
        points.push(Pos2::new(x, y));
    }
    points
}

fn draw_bezier_curve(ui: &mut Ui, p0: Pos2, p1: Pos2, p2: Pos2, color: Color32) {
    let segments = 100;
    let points = bezier_points(p0, p1, p2, segments);
    let path = Shape::line(points, Stroke::new(2.0, color));
    ui.painter().add(path);
}

fn draggable_point(ui: &mut Ui, point: &mut Pos2, color: Color32, points_on: bool) -> Response {
    let size = 7.0;
    let rect = Rect::from_center_size(*point, egui::vec2(size * 2.0, size * 2.0));
    let response = ui.allocate_rect(rect, Sense::click_and_drag());

    if response.dragged() {
        *point += response.drag_delta();
    }

    if points_on {
        ui.painter().circle_filled(*point, size, color);
    }
    response
}

fn draw_dotted_line(ui: &mut Ui, p0: Pos2, p1: Pos2, color: Color32) {
    let distance = ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt();
    let num_dots = (distance / 5.0).ceil() as usize;
    let mut points = Vec::with_capacity(num_dots);

    for i in 0..=num_dots {
        let t = i as f32 / num_dots as f32;
        let x = p0.x + t * (p1.x - p0.x);
        let y = p0.y + t * (p1.y - p0.y);
        points.push(Pos2::new(x, y));
    }

    for point in points {
        ui.painter().circle_filled(point, 2.0, color);
    }
}
