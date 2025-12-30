mod app;
mod editor;
mod document;
mod util;

use app::App;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Text Editor MVP",
        options,
        Box::new(|_cc| Box::new(App::new())),
    )
    .expect("failed to start eframe");
}
