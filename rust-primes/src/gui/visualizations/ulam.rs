use crate::gui::draw_number::draw_number;
use crate::gui::HOVER_THRESHOLD_DEFAULT;
use eframe::egui;

pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    let mut positions = Vec::with_capacity(max_n);

    if max_n == 0 {
        return positions;
    }

    let mut x = 0i32;
    let mut y = 0i32;
    let mut dx = 1i32;
    let mut dy = 0i32;
    let mut steps_in_direction = 1;
    let mut steps_since_turn = 0;
    let mut turn_count = 0;

    for n in 1..=max_n {
        positions.push((n, x as f32, y as f32));

        if n == max_n {
            break;
        }

        x += dx;
        y += dy;
        steps_since_turn += 1;

        if steps_since_turn == steps_in_direction {
            steps_since_turn = 0;

            let (new_dx, new_dy) = match turn_count % 4 {
                0 => (0, 1),
                1 => (-1, 0),
                2 => (0, -1),
                _ => (1, 0),
            };
            dx = new_dx;
            dy = new_dy;

            turn_count += 1;
            if turn_count % 2 == 0 {
                steps_in_direction += 1;
            }
        }
    }

    positions
}

pub fn draw(app: &crate::gui::app::PrimeVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect) {
    let positions = generate_positions(app.config.max_number);

    if positions.is_empty() {
        return;
    }

    let mut max_coord = 0.0f32;
    for (_, x, y) in &positions {
        max_coord = max_coord.max(x.abs()).max(y.abs());
    }

    let available = rect.width().min(rect.height()) / 2.0 - 20.0;
    let scale = if max_coord > 0.0 {
        available / max_coord
    } else {
        1.0
    };

    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let painter = ui.painter();

    for (n, x, y) in &positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;
        draw_number(*n, screen_x, screen_y, painter, &app.primes, &app.config);
    }
}

pub fn find_hovered(
    app: &crate::gui::app::PrimeVisualizerApp,
    mouse_pos: egui::Pos2,
    rect: egui::Rect,
) -> Option<usize> {
    let positions = generate_positions(app.config.max_number);
    if positions.is_empty() {
        return None;
    }

    let mut max_coord = 0.0f32;
    for (_, x, y) in &positions {
        max_coord = max_coord.max(x.abs()).max(y.abs());
    }

    let available = rect.width().min(rect.height()) / 2.0 - 20.0;
    let scale = if max_coord > 0.0 {
        available / max_coord
    } else {
        1.0
    };

    let center_x = rect.center().x;
    let center_y = rect.center().y;

    let mut closest_n: Option<usize> = None;
    let mut min_distance_sq = f32::INFINITY;

    for (n, x, y) in &positions {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;

        let dx = mouse_pos.x - screen_x;
        let dy = mouse_pos.y - screen_y;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq < min_distance_sq && distance_sq < (scale * HOVER_THRESHOLD_DEFAULT).powi(2)
        {
            min_distance_sq = distance_sq;
            closest_n = Some(*n);
        }
    }

    closest_n
}
