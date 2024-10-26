mod bezier;
mod cartesian;

enum Scene {
    Bezier,
    Cartesian,
}

pub struct App {
    bezier: bezier::BezierCurve,
    cartesian: cartesian::Cartesian,
    scene: Scene,
}

impl App {
    fn new(bezier: bezier::BezierCurve, cartesian: cartesian::Cartesian, scene: Scene) -> Self {
        Self {
            bezier,
            cartesian,
            scene,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let bez = bezier::BezierCurve::default();
        let cart = cartesian::Cartesian::default();
        Self::new(bez, cart, Scene::Bezier)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match &mut self.scene {
            Scene::Bezier => {
                self.bezier.update(ctx);
            }
            Scene::Cartesian => {
                self.cartesian.update(ctx);
            }
        }

        if self.cartesian.switch {
            self.scene = Scene::Bezier;
            self.cartesian.switch = false;
        } else if self.bezier.switch {
            self.scene = Scene::Cartesian;
            self.bezier.switch = false;
        }
    }
}
