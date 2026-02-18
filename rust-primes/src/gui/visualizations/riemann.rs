//! Riemann zeta visualization

use crate::gui::MARGIN_SMALL;
use eframe::egui;

pub fn draw(app: &crate::gui::app::PrimeVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let graph_left = rect.left() + MARGIN_SMALL;
    let graph_right = rect.right() - MARGIN_SMALL;
    let graph_top = rect.top() + MARGIN_SMALL;
    let graph_bottom = rect.bottom() - MARGIN_SMALL;
    let graph_width = graph_right - graph_left;
    let graph_height = graph_bottom - graph_top;

    let painter = ui.painter();

    painter.rect_filled(
        egui::Rect::from_min_size(
            egui::Pos2::new(graph_left, graph_top),
            egui::vec2(graph_width, graph_height),
        ),
        0.0,
        egui::Color32::from_rgba_unmultiplied(10, 10, 20, 255),
    );

    let max_imag = (app.config.max_number as f32 / 10.0).max(50.0);

    let min_re = 0.0f32;
    let max_re = 1.0f32;

    painter.line_segment(
        [
            egui::Pos2::new(graph_left, graph_top),
            egui::Pos2::new(graph_left, graph_bottom),
        ],
        egui::Stroke::new(2.0, egui::Color32::GRAY),
    );
    painter.line_segment(
        [
            egui::Pos2::new(graph_left, graph_bottom),
            egui::Pos2::new(graph_right, graph_bottom),
        ],
        egui::Stroke::new(2.0, egui::Color32::GRAY),
    );

    let critical_line_x = graph_left + (0.5 - min_re) / (max_re - min_re) * graph_width;
    painter.line_segment(
        [
            egui::Pos2::new(critical_line_x, graph_top),
            egui::Pos2::new(critical_line_x, graph_bottom),
        ],
        egui::Stroke::new(
            2.0,
            egui::Color32::from_rgba_unmultiplied(100, 200, 100, 200),
        ),
    );

    let font_id = egui::FontId::proportional(12.0);
    painter.text(
        egui::Pos2::new(graph_left + 5.0, graph_top + 5.0),
        egui::Align2::LEFT_TOP,
        "Im(s)",
        font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        egui::Pos2::new(graph_right - 5.0, graph_bottom - 15.0),
        egui::Align2::RIGHT_CENTER,
        "Re(s)",
        font_id.clone(),
        egui::Color32::WHITE,
    );
    painter.text(
        egui::Pos2::new(critical_line_x + 3.0, graph_top + 5.0),
        egui::Align2::LEFT_TOP,
        "σ=0.5",
        font_id.clone(),
        egui::Color32::from_rgba_unmultiplied(100, 200, 100, 255),
    );

    let zeros: &[f64] = &[
        14.134725141734695,
        21.022039638771556,
        25.01085758014569,
        32.9350615877392,
        37.586178158825675,
        40.9187190121475,
        43.3270732779141,
        48.00515088122016,
        49.7738324776723,
        52.97012314714253,
        56.44624769732639,
        59.34704400260235,
        60.8317785075098,
        65.11254406408108,
        67.07981039249472,
        69.54640171117398,
        72.0671576744819,
        75.70469069982593,
        77.1448400688748,
        79.33737502024643,
    ];

    let num_zeros_to_show = app.config.num_zeros.min(zeros.len());
    let zero_radius = 4.0;

    for (i, &imag) in zeros.iter().enumerate().take(num_zeros_to_show) {
        let imag = imag as f32;
        if imag > max_imag {
            break;
        }

        let x = critical_line_x;
        let y = graph_bottom - (imag / max_imag) * graph_height;

        painter.circle_filled(egui::Pos2::new(x, y), zero_radius, app.config.prime_color);

        if i < 10 || num_zeros_to_show <= 20 {
            let label = format!("{:.1}", imag);
            painter.text(
                egui::Pos2::new(x + 8.0, y - 6.0),
                egui::Align2::LEFT_BOTTOM,
                label,
                egui::FontId::proportional(9.0),
                app.config.prime_color,
            );
        }
    }

    let num_text = format!("Showing {} zeros", num_zeros_to_show);
    painter.text(
        egui::Pos2::new(graph_right - 5.0, graph_top + 5.0),
        egui::Align2::RIGHT_TOP,
        num_text,
        egui::FontId::proportional(11.0),
        egui::Color32::from_rgba_unmultiplied(180, 180, 180, 255),
    );

    let pnt_text = "Non-trivial zeros lie on σ=0.5 (Riemann Hypothesis)";
    painter.text(
        egui::Pos2::new(graph_left + 5.0, graph_bottom - 15.0),
        egui::Align2::LEFT_CENTER,
        pnt_text,
        egui::FontId::proportional(10.0),
        egui::Color32::from_rgba_unmultiplied(150, 150, 150, 255),
    );
}
