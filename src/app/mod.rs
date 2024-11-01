mod bezier;
mod cartesian;
mod cartesian3;

enum Scene {
    Bezier,
    Cartesian,
    Cartesian3D,
}

pub struct App {
    bezier: bezier::BezierCurve,
    cartesian: cartesian::Cartesian,
    cartesian3: cartesian3::Cartesian3D,
    scene: Scene,
}

impl App {
    fn new(
        bezier: bezier::BezierCurve,
        cartesian: cartesian::Cartesian,
        cartesian3: cartesian3::Cartesian3D,
        scene: Scene,
    ) -> Self {
        Self {
            bezier,
            cartesian,
            cartesian3,
            scene,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        let bez = bezier::BezierCurve::default();
        let cart = cartesian::Cartesian::default();
        let cart3 = cartesian3::Cartesian3D::default();
        Self::new(bez, cart, cart3, Scene::Cartesian)
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
            Scene::Cartesian3D => {
                self.cartesian3.update(ctx);
            }
        }

        if self.cartesian.switch == 2 {
            self.scene = Scene::Cartesian3D;
            self.cartesian.switch = 0;
        } else if self.cartesian.switch == 3 {
            self.scene = Scene::Bezier;
            self.cartesian.switch = 0;
        } else if self.bezier.switch == 1 {
            self.scene = Scene::Cartesian;
            self.bezier.switch = 0;
        } else if self.bezier.switch == 2 {
            self.scene = Scene::Cartesian3D;
            self.bezier.switch = 0;
        } else if self.cartesian3.switch == 1 {
            self.scene = Scene::Cartesian;
            self.cartesian3.switch = 0;
        } else if self.cartesian3.switch == 3 {
            self.scene = Scene::Bezier;
            self.cartesian3.switch = 0;
        }
    }
}
