mod gui;

use eframe::egui;
use gui::app::PrimeVisualizerApp;
use gui::config::VisualizerConfig;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Prime Number Visualizer",
        options,
        Box::new(|cc: &eframe::CreationContext<'_>| {
            Ok(Box::new(PrimeVisualizerApp::new(
                VisualizerConfig::default(),
                &cc.egui_ctx,
            )))
        }),
    )
}
