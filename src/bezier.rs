use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Shape, Stroke, Ui};

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

pub fn draw_bezier_curve(ui: &mut Ui, p0: Pos2, p1: Pos2, p2: Pos2, color: Color32) {
    let segments = 100;
    let points = bezier_points(p0, p1, p2, segments);
    let path = Shape::line(points, Stroke::new(2.0, color));
    ui.painter().add(path);
}

pub fn draggable_point(ui: &mut Ui, point: &mut Pos2, color: Color32) -> Response {
    let size = 7.0;
    let rect = Rect::from_center_size(*point, egui::vec2(size * 2.0, size * 2.0));
    let response = ui.allocate_rect(rect, Sense::click_and_drag());

    if response.dragged() {
        *point += response.drag_delta();
    }

    ui.painter().circle_filled(*point, size, color);
    response
}
