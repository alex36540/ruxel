pub mod model;
pub mod ui;
pub mod file_interactions;
pub mod change_manager;
use ui::Ruxel;

fn main() {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };
    match eframe::run_native("Ruxel", options, Box::new(|cc| Box::new(Ruxel::new(cc)))) {
        Ok(_) => println!("Bye bye..."),
        Err(e) => println!("Ouch! That wasn't supposed to happen... {}", e),
    }
}
