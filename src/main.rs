mod app;
use app::MyApp;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    }; // Default options
    eframe::run_native(
        "rSSH-Win",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    ) // Run the app
}
