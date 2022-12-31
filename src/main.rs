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

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(app::FenwickTree::new(16, cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
