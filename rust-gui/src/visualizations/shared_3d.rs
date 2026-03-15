//! Shared utilities for 3D visualizations

use crate::constants::projection;
use eframe::egui;

// Re-export constants for backward compatibility
pub use crate::constants::projection::*;

pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    /// Create a new 3D point with the given coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// Project a 3D point to 2D screen coordinates with perspective.
///
/// Applies Y-axis rotation followed by X-axis rotation, then perspective projection.
///
/// # Returns
/// A tuple of (screen_x, screen_y, depth) where depth is used for sorting.
pub fn project_3d_to_2d(point: &Point3D, rotation_y: f32, rotation_x: f32) -> (f32, f32, f32) {
    let cos_y = rotation_y.cos();
    let sin_y = rotation_y.sin();
    let x1 = point.x * cos_y - point.z * sin_y;
    let z1 = point.x * sin_y + point.z * cos_y;
    let y1 = point.y;

    let cos_x = rotation_x.cos();
    let sin_x = rotation_x.sin();
    let y2 = y1 * cos_x - z1 * sin_x;
    let z2 = y1 * sin_x + z1 * cos_x;

    let scale = projection::PERSPECTIVE / (projection::PERSPECTIVE - z2 + projection::OFFSET);

    (x1 * scale, y2 * scale, z2)
}

/// Adjust the brightness of a color by a multiplicative factor.
///
/// Each RGB component is multiplied by the factor and clamped to 255.
pub fn adjust_brightness(color: egui::Color32, factor: f32) -> egui::Color32 {
    let r = (color.r() as f32 * factor).min(255.0) as u8;
    let g = (color.g() as f32 * factor).min(255.0) as u8;
    let b = (color.b() as f32 * factor).min(255.0) as u8;
    egui::Color32::from_rgb(r, g, b)
}

/// Calculate a depth-based brightness factor for 3D rendering.
///
/// Returns a value between MIN_DEPTH_FACTOR and MAX_DEPTH_FACTOR based on the depth.
/// Used to darken objects that are farther away.
pub fn depth_factor(depth: f32) -> f32 {
    ((depth + projection::OFFSET) / projection::DEPTH_RANGE)
        .clamp(projection::MIN_DEPTH_FACTOR, projection::MAX_DEPTH_FACTOR)
}

/// Draw a complete 3D scene with shared boilerplate.
///
/// Handles mouse drag rotation, projection, depth sorting, scale fitting,
/// and depth-attenuated rendering. Each 3D visualization only needs to provide
/// a closure that maps `(n, is_highlighted)` to a `Point3D`.
///
/// # Arguments
/// * `app` - The application state (for rotation, config, highlights)
/// * `ui` - The egui UI context
/// * `rect` - The drawing rectangle
/// * `id` - A unique string ID for the drag interaction
/// * `generate_point` - Closure `(n: usize, is_highlighted: bool) -> Point3D`
///   that computes the 3D position for number `n`
pub fn draw_3d_scene(
    app: &mut crate::app::NumberVisualizerApp,
    ui: &mut egui::Ui,
    rect: egui::Rect,
    id: &str,
    generate_point: impl Fn(usize, bool) -> Point3D,
) {
    use crate::draw_number::get_prime_pair_color;
    use crate::helpers::MARGIN_SMALL;

    // Drag handling
    let response = ui.interact(rect, egui::Id::new(id), egui::Sense::drag());
    if response.dragged() {
        let delta = response.drag_delta();
        let (mut rotation_x, mut rotation_y) = app.get_rotation();
        rotation_y -= delta.x * DRAG_SENSITIVITY;
        rotation_x -= delta.y * DRAG_SENSITIVITY;
        rotation_x = rotation_x.clamp(-1.5, 1.5);
        app.set_rotation(rotation_x, rotation_y);
    }

    let (rotation_x, rotation_y) = app.get_rotation();
    let max_n = app.config.max_number;
    if max_n == 0 {
        return;
    }

    let highlights = app.highlights();

    // Generate and project all points
    let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
    for n in 1..=max_n {
        let is_highlighted = highlights.contains(&n);
        let point = generate_point(n, is_highlighted);
        let (px, py, pz) = project_3d_to_2d(&point, rotation_y, rotation_x);
        projected.push((px, py, pz, n, is_highlighted));
    }

    // Depth sort (painter's algorithm)
    projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    // Calculate scale to fit in rect
    let mut max_coord = 0.0f32;
    for (x, y, _, _, _) in &projected {
        max_coord = max_coord.max(x.abs()).max(y.abs());
    }
    let available = rect.width().min(rect.height()) / 2.0 - MARGIN_SMALL;
    let scale = if max_coord > 0.0 {
        available / max_coord
    } else {
        1.0
    };

    // Render
    let center_x = rect.center().x;
    let center_y = rect.center().y;
    let painter = ui.painter();

    for (x, y, depth, n, is_highlighted) in &projected {
        let screen_x = center_x + *x * scale;
        let screen_y = center_y + *y * scale;
        let df = depth_factor(*depth);

        if *is_highlighted {
            let size = (app.config.highlight_size as f32 * df) / 2.0;
            let base_color = get_prime_pair_color(*n, highlights, &app.config, app.series_type)
                .unwrap_or(app.config.highlight_color);
            let color = adjust_brightness(base_color, df);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        } else if app.config.non_highlight_size > 0 {
            let size = (app.config.non_highlight_size as f32 * df) / 2.0;
            let color = adjust_brightness(app.config.non_highlight_color, df);
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), size.max(0.5), color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_identity_rotation() {
        // With zero rotation, x should pass through unchanged, y should pass through,
        // and z should remain as-is for depth.
        let point = Point3D::new(10.0, 20.0, 0.0);
        let (px, py, pz) = project_3d_to_2d(&point, 0.0, 0.0);

        // With z=0 and zero rotation, z stays 0, perspective scale = PERSPECTIVE / (PERSPECTIVE + 0 + OFFSET)
        let expected_scale =
            projection::PERSPECTIVE / (projection::PERSPECTIVE + projection::OFFSET);
        assert!((px - 10.0 * expected_scale).abs() < 0.01);
        assert!((py - 20.0 * expected_scale).abs() < 0.01);
        assert!(pz.abs() < 0.01);
    }

    #[test]
    fn test_project_y_rotation_180() {
        // Rotating 180 degrees around Y should flip x and z
        let point = Point3D::new(10.0, 0.0, 0.0);
        let (px, _, _) = project_3d_to_2d(&point, std::f32::consts::PI, 0.0);

        // After 180-degree Y rotation, x should be negated (approximately)
        assert!(px < 0.0, "x should be negative after 180-degree Y rotation");
    }

    #[test]
    fn test_project_preserves_depth_ordering() {
        // Points further back (positive z) should have larger depth values
        let front = Point3D::new(0.0, 0.0, -50.0);
        let back = Point3D::new(0.0, 0.0, 50.0);

        let (_, _, depth_front) = project_3d_to_2d(&front, 0.0, 0.0);
        let (_, _, depth_back) = project_3d_to_2d(&back, 0.0, 0.0);

        assert!(
            depth_back > depth_front,
            "back point should have greater depth than front point"
        );
    }

    #[test]
    fn test_project_perspective_scaling() {
        // Closer objects (negative z) should appear larger (higher scale factor)
        let close = Point3D::new(10.0, 0.0, -50.0);
        let far = Point3D::new(10.0, 0.0, 50.0);

        let (px_close, _, _) = project_3d_to_2d(&close, 0.0, 0.0);
        let (px_far, _, _) = project_3d_to_2d(&far, 0.0, 0.0);

        assert!(
            px_close.abs() > px_far.abs(),
            "closer point should project larger than far point"
        );
    }

    #[test]
    fn test_adjust_brightness_identity() {
        let color = egui::Color32::from_rgb(100, 150, 200);
        let result = adjust_brightness(color, 1.0);
        assert_eq!(result.r(), 100);
        assert_eq!(result.g(), 150);
        assert_eq!(result.b(), 200);
    }

    #[test]
    fn test_adjust_brightness_half() {
        let color = egui::Color32::from_rgb(100, 200, 50);
        let result = adjust_brightness(color, 0.5);
        assert_eq!(result.r(), 50);
        assert_eq!(result.g(), 100);
        assert_eq!(result.b(), 25);
    }

    #[test]
    fn test_adjust_brightness_clamps_at_255() {
        let color = egui::Color32::from_rgb(200, 200, 200);
        let result = adjust_brightness(color, 2.0);
        assert_eq!(result.r(), 255);
        assert_eq!(result.g(), 255);
        assert_eq!(result.b(), 255);
    }

    #[test]
    fn test_adjust_brightness_zero() {
        let color = egui::Color32::from_rgb(100, 150, 200);
        let result = adjust_brightness(color, 0.0);
        assert_eq!(result.r(), 0);
        assert_eq!(result.g(), 0);
        assert_eq!(result.b(), 0);
    }

    #[test]
    fn test_depth_factor_clamped_range() {
        // depth_factor should always be in [MIN_DEPTH_FACTOR, MAX_DEPTH_FACTOR]
        for depth in [-500.0, -100.0, 0.0, 100.0, 500.0] {
            let df = depth_factor(depth);
            assert!(
                df >= projection::MIN_DEPTH_FACTOR,
                "depth_factor({}) = {} should be >= {}",
                depth,
                df,
                projection::MIN_DEPTH_FACTOR
            );
            assert!(
                df <= projection::MAX_DEPTH_FACTOR,
                "depth_factor({}) = {} should be <= {}",
                depth,
                df,
                projection::MAX_DEPTH_FACTOR
            );
        }
    }

    #[test]
    fn test_depth_factor_monotonic() {
        // Larger depth values should produce larger (or equal) depth factors
        let df1 = depth_factor(-100.0);
        let df2 = depth_factor(0.0);
        let df3 = depth_factor(100.0);

        assert!(df2 >= df1, "depth_factor should increase with depth");
        assert!(df3 >= df2, "depth_factor should increase with depth");
    }
}
