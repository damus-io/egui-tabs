use eframe::egui;
use egui::{Frame, Layout, Direction};
use egui_tabs::Tabs;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native("Tab Demo", options, Box::new(|_cc| Box::<MyApp>::default()))
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|_cc| Box::<MyApp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none())
            .show(ctx, |ui| {
                Tabs::new(3).height(32.0).layout(Layout::centered_and_justified(Direction::TopDown)).show(ui, |ui, ind| {
                    if ind == 0 {
                        ui.label("Tab A")
                    } else if ind == 1 {
                        ui.label("Tab B")
                    } else if ind == 2 {
                        ui.label("Tab C")
                    } else {
                        ui.label("Unknown")
                    }
                });
            });
    }
}
