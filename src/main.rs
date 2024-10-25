mod app;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Bezier Curve",
        options,
        Box::new(|_cc| Ok(Box::<app::App>::default())),
    )
}
