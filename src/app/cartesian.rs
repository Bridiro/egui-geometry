use eframe::egui::{self, Color32, Pos2};
use exmex::{lazy_static::lazy_static, prelude::*, regex};

lazy_static! {
    static ref POINT_REGEX: regex::Regex =
        regex::Regex::new(r"(\w)\s*=\s*\(([^,]+),\s*([^)]+)\)").unwrap();
    static ref EXPLICIT_REGEX: regex::Regex = regex::Regex::new(r"y\s*=\s*(.+)").unwrap();
    static ref IMPLICIT_REGEX: regex::Regex = regex::Regex::new(r"(.+)\s*=\s*0").unwrap();
    static ref PARAMETRIC_REGEX: regex::Regex =
        regex::Regex::new(r"x\s*\(\s*t\s*\)\s*=\s*(.+),\s*y\s*\(\s*t\s*\)\s*=\s*(.+)").unwrap();
}

enum FunctionType {
    Explicit(String),
    Implicit(String),
    Parametric { x_func: String, y_func: String },
}

pub struct Cartesian {
    inputs: Vec<(String, Color32)>,
    side_bar_open: bool,
    zoom: f32,
    pan: Pos2,
    axis_color: Color32,
    grid_color: Color32,
    pub switch: u8,
}

impl Cartesian {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.group(|ui| {
                ui.set_height(45.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    if !self.side_bar_open {
                        if ui
                            .add_sized([100.0, 10.0], egui::Button::new("Open"))
                            .clicked()
                        {
                            self.side_bar_open = true;
                        }
                        ui.separator();
                    }

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

                    if ui
                        .add_sized([100.0, 10.0], egui::Button::new("Reset"))
                        .on_hover_text("Reset the view")
                        .clicked()
                    {
                        self.zoom = 1.0;
                        self.pan = Pos2::ZERO;
                    }

                    if ui
                        .add_sized([100.0, 10.0], egui::Button::new("3D Cartesian"))
                        .on_hover_text("Switch to 3D Cartesian graph")
                        .clicked()
                    {
                        self.switch = 2;
                    }

                    if ui
                        .add_sized([100.0, 10.0], egui::Button::new("Bezier"))
                        .on_hover_text("Switch to Bezier curve")
                        .clicked()
                    {
                        self.switch = 3;
                    }
                });
            });
        });

        if self.side_bar_open {
            egui::SidePanel::left("functions").show(ctx, |ui| {
                ui.group(|ui| {
                    ui.set_width(160.0);
                    ui.add_sized([100.0, 10.0], egui::Button::new("Close"))
                        .on_hover_text("Close the side bar")
                        .clicked()
                        .then(|| self.side_bar_open = false);

                    ui.separator();

                    ui.label("Items:");

                    let mut to_remove = None;
                    for (i, (function, color)) in &mut self.inputs.iter_mut().enumerate() {
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.set_width(150.0);
                            ui.color_edit_button_srgba(color);
                            ui.text_edit_singleline(function);
                            if ui
                                .add_sized([20.0, 20.0], egui::Button::new("X"))
                                .on_hover_text("Remove item")
                                .clicked()
                            {
                                to_remove = Some(i);
                            }
                        });
                    }
                    ui.separator();
                    ui.add_sized([100.0, 10.0], egui::Button::new("Add"))
                        .on_hover_text("Add a new item")
                        .clicked()
                        .then(|| self.inputs.push((String::new(), Color32::WHITE)));
                    if let Some(i) = to_remove {
                        self.inputs.remove(i);
                    }
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.min_rect();
            self.pan += ui.interact(rect, ui.id(), egui::Sense::drag()).drag_delta();
            let mouse_pos_before_zoom = ui.input(|i| i.pointer.hover_pos()).unwrap_or_default();

            ui.input(|i| {
                let zoom_factor = 1.0 + i.smooth_scroll_delta.y * 0.01;
                let new_zoom = (self.zoom * zoom_factor).clamp(0.1, 10.0);

                let zoom_adjustment = mouse_pos_before_zoom - (rect.center() + self.pan.to_vec2());
                self.pan += zoom_adjustment * (1.0 - zoom_factor);
                self.zoom = new_zoom;
            });

            self.draw_grid(ui, rect);
            for i in 0..self.inputs.len() {
                if let Some(point) = self.parse_point(i) {
                    self.draw_point(ui, rect, &point.0, (point.1, point.2), self.inputs[i].1);
                } else if let Some(_) = self.parse_variable(i) {
                } else {
                    self.draw_function(ui, rect, i);
                }
            }
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

    fn draw_function(&self, ui: &mut egui::Ui, rect: egui::Rect, i: usize) {
        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let grid_unit = 40.0;

        let start_x = rect.left();
        let end_x = rect.right();

        let mut last_pos = None;

        for screen_x in (start_x as i32)..(end_x as i32) {
            let world_x = ((screen_x as f32 - center_x - self.pan.x) / self.zoom) / grid_unit;
            if let Some(world_y) = self.evaluate_expression(i, world_x as f64) {
                let screen_y = center_y - (world_y as f32 * self.zoom * grid_unit - self.pan.y);
                let pos = Pos2::new(screen_x as f32, screen_y);

                if let Some(last) = last_pos {
                    ui.painter()
                        .line_segment([last, pos], egui::Stroke::new(1.0, self.inputs[i].1));
                }

                last_pos = Some(pos);
            } else {
                last_pos = None;
            }
        }
    }

    fn draw_point(
        &self,
        ui: &mut egui::Ui,
        rect: egui::Rect,
        name: &str,
        (x, y): (f64, f64),
        color: Color32,
    ) {
        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let grid_unit = 40.0;

        let screen_x = center_x + (x as f32 * self.zoom * grid_unit) + self.pan.x;
        let screen_y = center_y - (y as f32 * self.zoom * grid_unit) + self.pan.y;

        let pos = Pos2::new(screen_x, screen_y);

        let point_radius = 5.0 * self.zoom;

        ui.painter()
            .circle(pos, point_radius, color, egui::Stroke::new(1.0, color));
        ui.painter().text(
            pos + Pos2::new(10.0 * self.zoom, 0.0).to_vec2(),
            egui::Align2::CENTER_CENTER,
            name,
            egui::FontId::default(),
            color,
        );
    }

    fn evaluate_expression(&self, i: usize, x: f64) -> Option<f64> {
        let mut expr = self.inputs[i].0.clone();

        for i in 0..self.inputs.len() {
            if let Some((name, value)) = self.parse_variable(i) {
                expr = expr.replace(&name, &value.to_string());
            } else if let Some((name, px, py)) = self.parse_point(i) {
                expr = expr.replace(&format!("{}.x", name), &px.to_string());
                expr = expr.replace(&format!("{}.y", name), &py.to_string());
            }
        }

        match self.parse_function(&expr)? {
            FunctionType::Explicit(func) => {
                let parsed = exmex::parse::<f64>(&func).ok()?;
                parsed.eval(&[x]).ok()
            }
            FunctionType::Implicit(func) => {
                let parsed = exmex::parse::<f64>(&func).ok()?;
                parsed.eval(&[x]).ok()
            }
            FunctionType::Parametric { x_func, y_func } => {
                let parsed1 = exmex::parse::<f64>(&x_func).ok()?;
                let parsed2 = exmex::parse::<f64>(&y_func).ok()?;
                parsed1
                    .eval(&[x])
                    .ok()
                    .and_then(|x| parsed2.eval(&[x]).ok())
            }
        }
    }

    fn parse_point(&self, i: usize) -> Option<(String, f64, f64)> {
        let input = &self.inputs[i].0;
        let mut converted = String::from(input);
        for i in 0..self.inputs.len() {
            if let Some((name, value)) = self.parse_variable(i) {
                converted = input.to_string().replace(&name, &value.to_string());
            }
        }
        if let Some(caps) = POINT_REGEX.captures(&converted) {
            let name = caps[1].to_string();
            let x = caps[2].parse().ok()?;
            let y = caps[3].parse().ok()?;
            Some((name, x, y))
        } else {
            None
        }
    }

    fn parse_variable(&self, i: usize) -> Option<(String, f64)> {
        if let Some(value_str) = self.inputs[i].0.split_once('=') {
            Some((value_str.0.into(), value_str.1.parse().ok()?))
        } else {
            None
        }
    }

    fn parse_function(&self, input: &str) -> Option<FunctionType> {
        if let Some(caps) = EXPLICIT_REGEX.captures(input) {
            Some(FunctionType::Explicit(caps[1].to_string()))
        } else if let Some(caps) = IMPLICIT_REGEX.captures(input) {
            Some(FunctionType::Implicit(caps[1].to_string()))
        } else if let Some(caps) = PARAMETRIC_REGEX.captures(input) {
            Some(FunctionType::Parametric {
                x_func: caps[1].to_string(),
                y_func: caps[2].to_string(),
            })
        } else {
            None
        }
    }
}

impl Default for Cartesian {
    fn default() -> Self {
        Self {
            inputs: vec![],
            side_bar_open: true,
            zoom: 1.0,
            pan: Pos2::ZERO,
            axis_color: Color32::WHITE,
            grid_color: Color32::from_gray(100),
            switch: 0,
        }
    }
}
