mod app;
mod config;
mod draw_number;
mod helpers;
mod types;
mod visualizations;

use app::NumberVisualizerApp;
use config::VisualizerConfig;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Number Sequence Visualizer",
        options,
        Box::new(|_cc: &eframe::CreationContext<'_>| {
            Ok(Box::new(NumberVisualizerApp::new(
                VisualizerConfig::default(),
            )))
        }),
    )
}
