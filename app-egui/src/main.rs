mod app;
mod ui;

fn main() {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_drag_and_drop(true)
            .with_title("OmniCIV")
            .with_app_id("io.github.zitronenjoghurt.omniciv"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "OmniCIV",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
