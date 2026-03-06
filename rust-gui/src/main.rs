#![no_main]

mod app;
mod config;
mod constants;
mod draw_number;
mod helpers;
mod types;
mod visualizations;

use app::NumberVisualizerApp;
use config::VisualizerConfig;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();
    let canvas = web_sys::window()
        .expect("no window")
        .document()
        .expect("no document")
        .get_element_by_id("the_canvas_id")
        .expect("no canvas element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("failed to convert to canvas");

    let runner = eframe::WebRunner::new();
    spawn_local(async move {
        runner
            .start(
                canvas,
                web_options,
                Box::new(|_cc| {
                    Ok(Box::new(NumberVisualizerApp::new(
                        VisualizerConfig::default(),
                    )))
                }),
            )
            .await
            .expect("Failed to start eframe web app");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::egui;

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
