use eframe::egui;

pub fn calculate_bounds(positions: &[(usize, f32, f32)]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for (_, x, y) in positions {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    (min_x, max_x, min_y, max_y)
}

pub fn calculate_scale(rect: egui::Rect, range_x: f32, range_y: f32, margin: f32) -> f32 {
    let available_width = rect.width() - 2.0 * margin;
    let available_height = rect.height() - 2.0 * margin;
    let scale_x = if range_x > 0.0 {
        available_width / range_x
    } else {
        1.0
    };
    let scale_y = if range_y > 0.0 {
        available_height / range_y
    } else {
        1.0
    };
    scale_x.min(scale_y)
}

pub fn gap_color(gap: usize) -> egui::Color32 {
    let brightness = match gap {
        2 => 255,
        4 => 220,
        6 => 180,
        8 => 150,
        10 => 120,
        12 => 100,
        14 => 85,
        16 => 70,
        _ if gap <= 20 => 60,
        _ if gap <= 30 => 45,
        _ if gap <= 50 => 30,
        _ => 20,
    };
    egui::Color32::from_rgba_unmultiplied(brightness, brightness, brightness, 255)
}

pub fn gap_stroke_width(gap: usize) -> f32 {
    if gap <= 4 {
        2.5
    } else if gap <= 6 {
        2.0
    } else if gap <= 10 {
        1.5
    } else if gap <= 20 {
        1.0
    } else {
        0.5
    }
}
