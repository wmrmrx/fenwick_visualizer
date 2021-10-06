mod app;

fn main() {
    let app = app::FenwickTree::new(16);
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::Vec2 { x: 800.0, y: 600.0 }),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
