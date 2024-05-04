use eframe::egui;
use egui::Frame;
use egui_tabs::Tabs;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native("Tab Demo", options, Box::new(|cc| Box::<MyApp>::default()))
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                Tabs::new(3).show(ui, |ui, ind| {
                    if ind == 0 {
                        ui.label("Tab A");
                    } else if ind == 1 {
                        ui.label("Tab B");
                    } else if ind == 2 {
                        ui.label("Tab C");
                    }
                });
            });
    }
}
