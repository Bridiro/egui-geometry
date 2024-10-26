mod bezier;
mod cartesian;

use eframe::egui::Key;

enum AppScene {
    Bezier(bezier::BezierCurve),
    Cartesian(cartesian::Cartesian),
}

pub struct App {
    scene: AppScene,
}

impl App {
    fn new(scene: AppScene) -> Self {
        Self { scene }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(AppScene::Bezier(bezier::BezierCurve::default()))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.scene {
            AppScene::Bezier(bezier) => {
                bezier.update(ctx);
            }
            AppScene::Cartesian(cartesian) => {
                cartesian.update(ctx);
            }
        }

        ctx.input(|i| {
            if i.key_pressed(Key::ArrowLeft) || i.key_pressed(Key::ArrowRight) {
                self.scene = match self.scene {
                    AppScene::Bezier(_) => AppScene::Cartesian(cartesian::Cartesian::default()),
                    AppScene::Cartesian(_) => AppScene::Bezier(bezier::BezierCurve::default()),
                };
            }
        });
    }
}
