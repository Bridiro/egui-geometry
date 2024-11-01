use eframe::egui::{self, Color32, Pos2, Vec2};
use exmex::{lazy_static::lazy_static, prelude::*, regex};

lazy_static! {
    static ref POINT_REGEX: regex::Regex =
        regex::Regex::new(r"(\w)\s*=\s*\(([^,]+),\s*([^)]+),\s*([^)]+)\)").unwrap();
    static ref LINE_REGEX: regex::Regex = regex::Regex::new(
        r"line\s*\(([^,]+),\s*([^,]+),\s*([^,]+),\s*([^,]+),\s*([^,]+),\s*([^,]+)\)"
    )
    .unwrap();
    static ref PLANE_REGEX: regex::Regex =
        regex::Regex::new(r"plane\s*\(([^,]+),\s*([^,]+),\s*([^,]+),\s*([^,]+)\)").unwrap();
}

enum Object3D {
    Point(String, f64, f64, f64),
    Line(String, (f64, f64, f64), (f64, f64, f64)),
    Plane(String, f64, f64, f64, f64),
}

pub struct Cartesian3D {
    inputs: Vec<(String, Color32)>,
    zoom: f32,
    pan: Pos2,
    rotation: (f32, f32),
    axis_color: Color32,
    grid_color: Color32,
    pub switch: u8,
}

impl Cartesian3D {
    pub fn update(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset View").clicked() {
                    self.zoom = 1.0;
                    self.pan = Pos2::ZERO;
                    self.rotation = (0.0, 0.0);
                }

                ui.vertical(|ui| {
                    if ui.button("Cartesian").clicked() {
                        self.switch = 1;
                    }

                    if ui.button("Bezier").clicked() {
                        self.switch = 3;
                    }
                });
            });
        });

        egui::SidePanel::left("objects").show(ctx, |ui| {
            ui.label("3D Objects:");
            let mut to_remove = None;
            for (i, (object_str, color)) in self.inputs.iter_mut().enumerate() {
                ui.color_edit_button_srgba(color);
                ui.text_edit_singleline(object_str);
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            }
            if let Some(i) = to_remove {
                self.inputs.remove(i);
            }

            if ui.button("Add Object").clicked() {
                self.inputs.push((String::new(), Color32::WHITE));
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.min_rect();

            self.handle_mouse_input(ui, rect);
            self.draw_axes(ui, rect);

            for (object_str, color) in &self.inputs {
                if let Some(object) = self.parse_object(object_str) {
                    self.draw_object(ui, rect, object, *color);
                }
            }
        });
    }

    fn handle_mouse_input(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        self.pan += ui.interact(rect, ui.id(), egui::Sense::drag()).drag_delta();
        let mouse_pos_before_zoom = ui.input(|i| i.pointer.hover_pos()).unwrap_or_default();

        ui.input(|i| {
            let zoom_factor = 1.0 + i.smooth_scroll_delta.y * 0.01;
            let new_zoom = (self.zoom * zoom_factor).clamp(0.1, 10.0);

            let zoom_adjustment = mouse_pos_before_zoom - (rect.center() + self.pan.to_vec2());
            self.pan += zoom_adjustment * (1.0 - zoom_factor);
            self.zoom = new_zoom;

            if i.key_pressed(egui::Key::ArrowLeft) {
                self.rotation.1 -= 5.0;
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.rotation.1 += 5.0;
            }
            if i.key_pressed(egui::Key::ArrowUp) {
                self.rotation.0 -= 5.0;
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.rotation.0 += 5.0;
            }
        });
    }

    fn draw_axes(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let axis_length = 100.0 * self.zoom as f64;
        let axis_color = self.axis_color;

        let x_axis = [
            self.project_3d_to_2d((axis_length, 0.0, 0.0), rect),
            self.project_3d_to_2d((-axis_length, 0.0, 0.0), rect),
        ];
        let y_axis = [
            self.project_3d_to_2d((0.0, axis_length, 0.0), rect),
            self.project_3d_to_2d((0.0, -axis_length, 0.0), rect),
        ];
        let z_axis = [
            self.project_3d_to_2d((0.0, 0.0, axis_length), rect),
            self.project_3d_to_2d((0.0, 0.0, -axis_length), rect),
        ];

        ui.painter()
            .line_segment(x_axis, egui::Stroke::new(2.0, axis_color));
        ui.painter()
            .line_segment(y_axis, egui::Stroke::new(2.0, axis_color));
        ui.painter()
            .line_segment(z_axis, egui::Stroke::new(2.0, axis_color));
    }

    fn draw_object(&self, ui: &mut egui::Ui, rect: egui::Rect, object: Object3D, color: Color32) {
        match object {
            Object3D::Point(name, x, y, z) => {
                let pos = self.project_3d_to_2d((x, y, z), rect);
                ui.painter().circle_filled(pos, 5.0, color);
                ui.painter().text(
                    pos + egui::vec2(10.0, 0.0),
                    egui::Align2::LEFT_CENTER,
                    name,
                    egui::FontId::default(),
                    color,
                );
            }
            Object3D::Line(_name, start, end) => {
                let start_2d = self.project_3d_to_2d(start, rect);
                let end_2d = self.project_3d_to_2d(end, rect);
                ui.painter()
                    .line_segment([start_2d, end_2d], egui::Stroke::new(2.0, color));
            }
            Object3D::Plane(name, a, b, c, d) => {
                ui.label(format!("Rendering plane {}", name));
            }
        }
    }

    fn project_3d_to_2d(&self, (x, y, z): (f64, f64, f64), rect: egui::Rect) -> Pos2 {
        let (pitch, yaw) = (self.rotation.0.to_radians(), self.rotation.1.to_radians());
        let cos_yaw = yaw.cos();
        let sin_yaw = yaw.sin();
        let cos_pitch = pitch.cos();
        let sin_pitch = pitch.sin();

        let dx = x as f32 * cos_yaw - z as f32 * sin_yaw;
        let dz = x as f32 * sin_yaw + z as f32 * cos_yaw;
        let dy = y as f32 * cos_pitch - dz * sin_pitch;

        let pos = rect.center() + egui::vec2(dx, -dy) * self.zoom + self.pan.to_vec2();
        pos
    }

    fn parse_object(&self, input: &str) -> Option<Object3D> {
        if let Some(caps) = POINT_REGEX.captures(input) {
            let name = caps[1].to_string();
            let x: f64 = caps[2].parse().ok()?;
            let y: f64 = caps[3].parse().ok()?;
            let z: f64 = caps[4].parse().ok()?;
            Some(Object3D::Point(name, x, y, z))
        } else if let Some(caps) = LINE_REGEX.captures(input) {
            let name = caps[1].to_string();
            let start = (
                caps[1].parse().ok()?,
                caps[2].parse().ok()?,
                caps[3].parse().ok()?,
            );
            let end = (
                caps[4].parse().ok()?,
                caps[5].parse().ok()?,
                caps[6].parse().ok()?,
            );
            Some(Object3D::Line(name, start, end))
        } else if let Some(caps) = PLANE_REGEX.captures(input) {
            let name = caps[1].to_string();
            let a: f64 = caps[2].parse().ok()?;
            let b: f64 = caps[3].parse().ok()?;
            let c: f64 = caps[4].parse().ok()?;
            let d: f64 = caps[5].parse().ok()?;
            Some(Object3D::Plane(name, a, b, c, d))
        } else {
            None
        }
    }
}

impl Default for Cartesian3D {
    fn default() -> Self {
        Self {
            inputs: vec![],
            zoom: 1.0,
            pan: Pos2::ZERO,
            rotation: (0.0, 30.0),
            axis_color: Color32::from_rgb(100, 100, 200),
            grid_color: Color32::from_rgb(200, 200, 200),
            switch: 2,
        }
    }
}
