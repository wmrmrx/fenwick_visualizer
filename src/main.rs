mod app;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Fenwick Tree Visualizer",
        native_options,
        Box::new(|cc| Box::new(app::FenwickTree::new(16, cc))),
    );
}
