//! PNG Export functionality for visualizations

use crate::app::NumberVisualizerApp;
use crate::helpers::{gap_color, gap_stroke_width};
use crate::types::VisualizationType;
use crate::visualizations::shared_3d::{
    adjust_brightness, depth_factor, project_3d_to_2d, Point3D,
};
use eframe::egui;
use image::{ImageBuffer, Rgba};
use std::path::PathBuf;

pub struct Exporter;

impl Exporter {
    pub fn export_png(
        app: &NumberVisualizerApp,
        output_path: &PathBuf,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

        let bg = app.config.background_color;
        for pixel in img.pixels_mut() {
            *pixel = Rgba([bg.r(), bg.g(), bg.b(), bg.a()]);
        }

        match app.config.visualization {
            VisualizationType::UlamSpiral => Self::render_ulam_spiral(&mut img, app, width, height),
            VisualizationType::SacksSpiral => {
                Self::render_sacks_spiral(&mut img, app, width, height)
            }
            VisualizationType::Grid => Self::render_grid(&mut img, app, width, height),
            VisualizationType::Row => Self::render_row(&mut img, app, width, height),
            VisualizationType::FermatsSpiral => {
                Self::render_fermats_spiral(&mut img, app, width, height)
            }
            VisualizationType::HexagonalLattice => {
                Self::render_hexagonal_lattice(&mut img, app, width, height)
            }
            VisualizationType::TriangularLattice => {
                Self::render_triangular_lattice(&mut img, app, width, height)
            }
            VisualizationType::PrimeWheel => Self::render_prime_wheel(&mut img, app, width, height),
            VisualizationType::PrimeDensity => {
                Self::render_prime_density(&mut img, app, width, height)
            }
            VisualizationType::RiemannZeta => Self::render_riemann(&mut img, app, width, height),
            VisualizationType::SacksMobiusSpiral => {
                Self::render_mobius_spiral(&mut img, app, width, height, true)
            }
            VisualizationType::UlamMobiusSpiral => {
                Self::render_mobius_spiral(&mut img, app, width, height, false)
            }
            VisualizationType::PrimeDensityGradient => {
                Self::render_density_gradient(&mut img, app, width, height)
            }
            VisualizationType::Helix3D => Self::render_helix_3d(&mut img, app, width, height),
            VisualizationType::Sphere3D => Self::render_sphere_3d(&mut img, app, width, height),
            VisualizationType::Torus3D => Self::render_torus_3d(&mut img, app, width, height),
            VisualizationType::Cone3D => Self::render_cone_3d(&mut img, app, width, height),
            VisualizationType::Cylinder3D => Self::render_cylinder_3d(&mut img, app, width, height),
            VisualizationType::CubeQuadratic3D => {
                Self::render_cube_quadratic_3d(&mut img, app, width, height)
            }
            VisualizationType::CubeSimple3D => {
                Self::render_cube_quadratic_3d(&mut img, app, width, height)
            }
            VisualizationType::Mobius3D => Self::render_mobius_3d(&mut img, app, width, height),
            VisualizationType::Klein3D => Self::render_klein_3d(&mut img, app, width, height),
            VisualizationType::Pyramid3D => Self::render_pyramid_3d(&mut img, app, width, height),
            VisualizationType::Dodecahedron3D => {
                Self::render_dodecahedron_3d(&mut img, app, width, height)
            }
            VisualizationType::Icosahedron3D => {
                Self::render_icosahedron_3d(&mut img, app, width, height)
            }
            VisualizationType::Trefoil3D => Self::render_trefoil_3d(&mut img, app, width, height),
        }

        img.save(output_path)
            .map_err(|e| format!("Failed to save PNG: {}", e))
    }

    fn render_ulam_spiral(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_ulam_positions(app.config.max_number);
        Self::render_spiral_points(img, app, width, height, &positions);
    }

    fn render_sacks_spiral(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_sacks_positions(app.config.max_number);
        Self::render_spiral_points(img, app, width, height, &positions);
    }

    fn render_fermats_spiral(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_fermats_positions(app.config.max_number);
        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        Self::render_2d_points(
            img, app, width, height, &positions, highlights, rot_x, rot_y, true,
        );
    }

    fn render_hexagonal_lattice(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_hexagonal_positions(app.config.max_number);
        Self::render_lattice_points(img, app, width, height, &positions);
    }

    fn render_triangular_lattice(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_triangular_positions(app.config.max_number);
        Self::render_lattice_points(img, app, width, height, &positions);
    }

    fn render_spiral_points(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
        positions: &[(usize, f32, f32)],
    ) {
        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        Self::render_2d_points(
            img, app, width, height, positions, highlights, rot_x, rot_y, false,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn render_2d_points(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
        positions: &[(usize, f32, f32)],
        highlights: &std::collections::HashSet<usize>,
        _rot_x: f32,
        _rot_y: f32,
        flip_y: bool,
    ) {
        let max_coord = positions
            .iter()
            .map(|(_, x, y)| x.abs().max(y.abs()))
            .fold(0.0f32, |a, b| a.max(b));

        let available = (width as f32).min(height as f32) / 2.0 - 40.0;
        let scale = if max_coord > 0.0 {
            available / max_coord
        } else {
            1.0
        };

        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let y_mult = if flip_y { -1.0 } else { 1.0 };

        for (n, x, y) in positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + y_mult * *y * scale;
            let is_highlighted = highlights.contains(n);

            if screen_x < 0.0
                || screen_x >= width as f32
                || screen_y < 0.0
                || screen_y >= height as f32
            {
                continue;
            }

            let color = if is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };

            let radius = if is_highlighted {
                app.config.highlight_size as f32 / 2.0
            } else {
                app.config.non_highlight_size as f32 / 2.0
            }
            .max(0.5);

            Self::draw_circle(img, screen_x as i32, screen_y as i32, radius as i32, color);
        }
    }

    fn render_lattice_points(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
        positions: &[(usize, f32, f32)],
    ) {
        let highlights = app.highlights();

        let (min_x, max_x, min_y, max_y) = Self::calculate_bounds(positions);
        let mid_x = (min_x + max_x) / 2.0;
        let mid_y = (min_y + max_y) / 2.0;

        let range_x = max_x - min_x;
        let range_y = max_y - min_y;
        let available_width = width as f32 - 80.0;
        let available_height = height as f32 - 80.0;
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
        let scale = scale_x.min(scale_y);

        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;

        for (n, x, y) in positions {
            let screen_x = center_x + (*x - mid_x) * scale;
            let screen_y = center_y - (*y - mid_y) * scale;
            let is_highlighted = highlights.contains(n);

            if screen_x < 0.0
                || screen_x >= width as f32
                || screen_y < 0.0
                || screen_y >= height as f32
            {
                continue;
            }

            let color = if is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };

            let radius = if is_highlighted {
                app.config.highlight_size as f32 / 2.0
            } else {
                app.config.non_highlight_size as f32 / 2.0
            }
            .max(0.5);

            Self::draw_circle(img, screen_x as i32, screen_y as i32, radius as i32, color);
        }
    }

    fn render_grid(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_grid_positions(app.config.max_number);
        let highlights = app.highlights();

        let (min_x, max_x, min_y, max_y) = Self::calculate_bounds(&positions);
        let range_x = max_x - min_x;
        let range_y = max_y - min_y;

        let available_width = width as f32 - 80.0;
        let available_height = height as f32 - 80.0;
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
        let scale = scale_x.min(scale_y);

        let start_x = (width as f32 - range_x * scale) / 2.0;
        let start_y = (height as f32 + range_y * scale) / 2.0;

        for (n, x, y) in &positions {
            let screen_x = start_x + *x * scale;
            let screen_y = start_y - *y * scale;
            let is_highlighted = highlights.contains(n);

            if screen_x < 0.0
                || screen_x >= width as f32
                || screen_y < 0.0
                || screen_y >= height as f32
            {
                continue;
            }

            let color = if is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };

            let radius = if is_highlighted {
                app.config.highlight_size as f32 / 2.0
            } else {
                app.config.non_highlight_size as f32 / 2.0
            }
            .max(0.5);

            Self::draw_circle(img, screen_x as i32, screen_y as i32, radius as i32, color);
        }
    }

    fn render_row(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_row_positions(app.config.max_number);
        let highlights = app.highlights();

        let max_x = positions
            .iter()
            .map(|(_, x, _)| *x)
            .fold(0.0f32, |a, b| a.max(b));

        let scale = (width as f32 - 80.0) / max_x.max(1.0);
        let start_x = 40.0;
        let center_y = height as f32 / 2.0;

        for (n, x, _) in &positions {
            let screen_x = start_x + *x * scale;
            let is_highlighted = highlights.contains(n);

            if screen_x < 0.0 || screen_x >= width as f32 {
                continue;
            }

            let color = if is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };

            let radius = if is_highlighted {
                app.config.highlight_size as f32 / 2.0
            } else {
                app.config.non_highlight_size as f32 / 2.0
            }
            .max(0.5);

            Self::draw_circle(img, screen_x as i32, center_y as i32, radius as i32, color);
        }
    }

    fn render_prime_wheel(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let positions = crate::visualizations::generate_prime_wheel_positions(
            app.config.max_number,
            app.config.modulo,
        );
        let highlights = app.highlights();

        let max_r = positions
            .iter()
            .map(|(_, x, y)| (x * x + y * y).sqrt())
            .fold(0.0f32, |a, b| a.max(b));

        let available = (width.min(height)) as f32 / 2.0 - 40.0;
        let scale = if max_r > 0.0 { available / max_r } else { 1.0 };

        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;

        for (n, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + *y * scale;
            let is_highlighted = highlights.contains(n);

            if screen_x < 0.0
                || screen_x >= width as f32
                || screen_y < 0.0
                || screen_y >= height as f32
            {
                continue;
            }

            let color = if is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };

            let radius = if is_highlighted {
                app.config.highlight_size as f32 / 2.0
            } else {
                app.config.non_highlight_size as f32 / 2.0
            }
            .max(0.5);

            Self::draw_circle(img, screen_x as i32, screen_y as i32, radius as i32, color);
        }
    }

    fn render_mobius_spiral(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
        use_sacks: bool,
    ) {
        let primes_vec = app.primes_vec();

        if primes_vec.len() < 2 {
            return;
        }

        let spiral_positions: Vec<(usize, f32, f32)> = if use_sacks {
            crate::visualizations::generate_sacks_positions(primes_vec.len())
        } else {
            crate::visualizations::generate_ulam_positions(primes_vec.len())
        };

        let positions: Vec<(usize, f32, f32)> = primes_vec
            .iter()
            .enumerate()
            .map(|(idx, &n)| {
                let (_, x, y) = spiral_positions[idx];
                (n, x, y)
            })
            .collect();

        let max_coord = positions
            .iter()
            .map(|(_, x, y)| x.abs().max(y.abs()))
            .fold(0.0f32, |a, b| a.max(b));

        let available = (width.min(height)) as f32 / 2.0 - 40.0;
        let scale = if max_coord > 0.0 {
            available / max_coord
        } else {
            1.0
        };

        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let y_sign = if use_sacks { -1.0 } else { 1.0 };

        for i in 0..positions.len() - 1 {
            let (_, x1, y1) = positions[i];
            let (_, x2, y2) = positions[i + 1];
            let gap = positions[i + 1].0 - positions[i].0;

            let screen_x1 = center_x + x1 * scale;
            let screen_y1 = center_y + y_sign * y1 * scale;
            let screen_x2 = center_x + x2 * scale;
            let screen_y2 = center_y + y_sign * y2 * scale;

            let color = gap_color(gap);
            let stroke_width = gap_stroke_width(gap);

            Self::draw_line(
                img,
                screen_x1,
                screen_y1,
                screen_x2,
                screen_y2,
                stroke_width,
                color,
            );
        }

        for (n, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + y_sign * *y * scale;
            let is_highlighted = app.primes_set().contains(n);

            let color = if is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };

            let radius = if is_highlighted {
                app.config.highlight_size as f32 / 2.0
            } else {
                app.config.non_highlight_size as f32 / 2.0
            }
            .max(0.5);

            Self::draw_circle(img, screen_x as i32, screen_y as i32, radius as i32, color);
        }
    }

    fn render_prime_density(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n < 10 {
            return;
        }

        let prime_count = app.primes_vec().len();
        let intervals = (max_n / 100).clamp(1, 200);
        let interval_size = max_n / intervals;

        let mut pi_x: Vec<(f32, f32)> = Vec::with_capacity(intervals + 1);
        let mut x_ln_x: Vec<(f32, f32)> = Vec::with_capacity(intervals + 1);

        let mut count = 0;
        let mut prime_idx = 0;

        for i in 0..=intervals {
            let x = i * interval_size;
            if x < 2 {
                pi_x.push((x as f32, 0.0));
                x_ln_x.push((x as f32, 0.0));
                continue;
            }

            while prime_idx < prime_count && app.primes_vec()[prime_idx] <= x {
                count += 1;
                prime_idx += 1;
            }

            let ln_x = (x as f64).ln().max(1.0) as f32;
            let x_ln_x_val = x as f32 / ln_x;

            pi_x.push((x as f32, count as f32));
            x_ln_x.push((x as f32, x_ln_x_val));
        }

        let max_y = pi_x.last().map(|(_, y)| *y).unwrap_or(1.0).max(1.0);

        let margin = 40.0;
        let graph_left = margin;
        let graph_right = width as f32 - margin;
        let graph_top = margin;
        let graph_bottom = height as f32 - margin;
        let graph_width = graph_right - graph_left;
        let graph_height = graph_bottom - graph_top;

        let bg = app.config.background_color;
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgba([bg.r(), bg.g(), bg.b(), bg.a()]));
            }
        }

        Self::draw_line(
            img,
            graph_left,
            graph_top,
            graph_left,
            graph_bottom,
            2.0,
            egui::Color32::GRAY,
        );
        Self::draw_line(
            img,
            graph_left,
            graph_bottom,
            graph_right,
            graph_bottom,
            2.0,
            egui::Color32::GRAY,
        );

        let max_x = max_n as f32;

        for i in 0..x_ln_x.len() - 1 {
            let (x1, y1) = x_ln_x[i];
            let (x2, y2) = x_ln_x[i + 1];
            let px1 = graph_left + (x1 / max_x) * graph_width;
            let py1 = graph_bottom - (y1 / max_y) * graph_height;
            let px2 = graph_left + (x2 / max_x) * graph_width;
            let py2 = graph_bottom - (y2 / max_y) * graph_height;
            Self::draw_line(img, px1, py1, px2, py2, 2.0, app.config.non_highlight_color);
        }

        for i in 0..pi_x.len() - 1 {
            let (x1, y1) = pi_x[i];
            let (x2, y2) = pi_x[i + 1];
            let px1 = graph_left + (x1 / max_x) * graph_width;
            let py1 = graph_bottom - (y1 / max_y) * graph_height;
            let px2 = graph_left + (x2 / max_x) * graph_width;
            let py2 = graph_bottom - (y2 / max_y) * graph_height;
            Self::draw_line(img, px1, py1, px2, py2, 2.0, app.config.highlight_color);
        }
    }

    fn render_riemann(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let margin = 40.0;
        let graph_left = margin;
        let graph_right = width as f32 - margin;
        let graph_top = margin;
        let graph_bottom = height as f32 - margin;
        let graph_width = graph_right - graph_left;
        let graph_height = graph_bottom - graph_top;

        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Rgba([10, 10, 20, 255]));
            }
        }

        let max_imag = (app.config.max_number as f32 / 10.0).max(50.0);
        let critical_line_x = graph_left + 0.5 * graph_width;

        Self::draw_line(
            img,
            graph_left,
            graph_top,
            graph_left,
            graph_bottom,
            2.0,
            egui::Color32::GRAY,
        );
        Self::draw_line(
            img,
            graph_left,
            graph_bottom,
            graph_right,
            graph_bottom,
            2.0,
            egui::Color32::GRAY,
        );
        Self::draw_line(
            img,
            critical_line_x,
            graph_top,
            critical_line_x,
            graph_bottom,
            2.0,
            egui::Color32::from_rgba_unmultiplied(100, 200, 100, 200),
        );

        const RIEMANN_ZEROS: &[f64] = &[
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

        let num_zeros_to_show = app.config.num_zeros.min(RIEMANN_ZEROS.len());

        for (_, &imag) in RIEMANN_ZEROS.iter().enumerate().take(num_zeros_to_show) {
            let imag = imag as f32;
            if imag > max_imag {
                break;
            }

            let x = critical_line_x;
            let y = graph_bottom - (imag / max_imag) * graph_height;

            Self::draw_circle(img, x as i32, y as i32, 4, app.config.highlight_color);
        }
    }

    fn render_density_gradient(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let grid_size = app.config.grid_size.clamp(10, 100);
        let max_n = app.config.max_number;
        let primes_set = app.primes_set();

        let cell_width = width as f32 / grid_size as f32;
        let cell_height = height as f32 / grid_size as f32;

        for gy in 0..grid_size {
            for gx in 0..grid_size {
                let start = ((gx * grid_size + gy) as f32 * max_n as f32
                    / (grid_size * grid_size) as f32) as usize;
                let end = (((gx + 1) * grid_size + gy + 1) as f32 * max_n as f32
                    / (grid_size * grid_size) as f32) as usize;

                let prime_count = (start..end.min(max_n))
                    .filter(|n| primes_set.contains(n))
                    .count();
                let cell_total = end - start;
                let density = if cell_total > 0 {
                    prime_count as f32 / cell_total as f32
                } else {
                    0.0
                };

                let brightness = (density * 255.0) as u8;
                let color = Rgba([brightness, brightness, brightness, 255]);

                let px = (gx as f32 * cell_width) as u32;
                let py = (gy as f32 * cell_height) as u32;
                let pw = (cell_width.ceil()) as u32;
                let ph = (cell_height.ceil()) as u32;

                for dy in 0..ph {
                    for dx in 0..pw {
                        if px + dx < width && py + dy < height {
                            img.put_pixel(px + dx, py + dy, color);
                        }
                    }
                }
            }
        }
    }

    fn render_helix_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let theta = (n as f32 - 1.0) * 8.0 * std::f32::consts::PI / max_n_f;
            let radius = 100.0;
            let y = (n as f32 - max_n_f / 2.0) * 3.0;
            let point = Point3D::new(radius * theta.cos(), y, radius * theta.sin());
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_sphere_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let theta = 2.0 * std::f32::consts::PI * (n - 1) as f32 / golden_ratio;
            let phi = (1.0 - 2.0 * n as f32 / max_n as f32).acos();
            let r = if is_highlighted { 115.0 } else { 100.0 };
            let point = Point3D::new(
                r * phi.sin() * theta.cos(),
                r * phi.cos(),
                r * phi.sin() * theta.sin(),
            );
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_torus_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let theta = (n as f32 - 1.0) * 8.0 * std::f32::consts::PI / max_n_f;
            let phi = (n as f32 - 1.0) * 2.0 * std::f32::consts::PI / 50.0;
            let major = 80.0;
            let minor = if is_highlighted { 40.0 } else { 30.0 };
            let point = Point3D::new(
                (major + minor * phi.cos()) * theta.cos(),
                minor * phi.sin(),
                (major + minor * phi.cos()) * theta.sin(),
            );
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_cone_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let theta = (n as f32 - 1.0) * 10.0 * std::f32::consts::PI / max_n_f;
            let r = ((n as f32 - 1.0) / max_n_f * 100.0).max(10.0);
            let h = (n as f32 - 1.0) / max_n_f * 200.0 - 100.0;
            let radius = if is_highlighted { r + 15.0 } else { r };
            let point = Point3D::new(radius * theta.cos(), h, radius * theta.sin());
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_cylinder_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let theta = (n as f32 - 1.0) * 8.0 * std::f32::consts::PI / max_n_f;
            let h = (n as f32 - 1.0) / max_n_f * 200.0 - 100.0;
            let r = if is_highlighted { 95.0 } else { 80.0 };
            let point = Point3D::new(r * theta.cos(), h, r * theta.sin());
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_cube_quadratic_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let face_size = (max_n_f.sqrt().ceil() as usize / 6).max(1);
            let face = ((n - 1) / face_size) % 6;
            let idx = (n - 1) % face_size;
            let angle = (idx as f32) / face_size as f32 * std::f32::consts::PI * 2.0;
            let r = if is_highlighted { 95.0 } else { 80.0 };
            let point = match face {
                0 => Point3D::new(r * angle.cos(), r * angle.sin(), r),
                1 => Point3D::new(-r, r * angle.cos(), r * angle.sin()),
                2 => Point3D::new(-r * angle.cos(), -r * angle.sin(), -r),
                3 => Point3D::new(r, -r * angle.cos(), -r * angle.sin()),
                4 => Point3D::new(r * angle.cos(), r, r * angle.sin()),
                _ => Point3D::new(r * angle.cos(), -r, -r * angle.sin()),
            };
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_mobius_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let t = (n as f32 - 1.0) / max_n_f * 4.0 * std::f32::consts::PI;
            let r = 80.0;
            let w = if is_highlighted { 45.0 } else { 30.0 };
            let half_t = t / 2.0;
            let point = Point3D::new(
                (r + w * half_t.cos()) * t.cos(),
                w * half_t.sin(),
                (r + w * half_t.cos()) * t.sin(),
            );
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_klein_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let t = (n as f32 - 1.0) / max_n_f * 4.0 * std::f32::consts::PI;
            let s = (n as f32 - 1.0) / max_n_f * 2.0 * std::f32::consts::PI;
            let r = if is_highlighted { 70.0 } else { 60.0 };
            let x = (r + r * t.cos() / 2.0 * t.cos()) * s.cos();
            let y = (r + r * t.cos() / 2.0 * t.cos()) * s.sin();
            let z = r * t.sin() / 2.0;
            let point = Point3D::new(x, y, z);
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_pyramid_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let t = (n as f32 - 1.0) / max_n_f;
            let base = t * 120.0;
            let h = t * 150.0;
            let angle = t * 4.0 * std::f32::consts::PI;
            let r = if is_highlighted { base + 15.0 } else { base };
            let point = Point3D::new(r * angle.cos(), h, r * angle.sin());
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_dodecahedron_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;
        let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let i = n as f32;
            let j = (i * golden_ratio).fract();
            let theta = 2.0 * std::f32::consts::PI * j;
            let phi = (1.0 - 2.0 * i / max_n_f).acos();
            let r = if is_highlighted { 80.0 } else { 70.0 };
            let point = Point3D::new(
                r * phi.sin() * theta.cos(),
                r * phi.cos(),
                r * phi.sin() * theta.sin(),
            );
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_icosahedron_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;
        let golden_ratio = (1.0 + 5.0f32.sqrt()) / 2.0;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let i = n as f32;
            let j = (i * golden_ratio).fract();
            let theta = 2.0 * std::f32::consts::PI * j;
            let phi = (1.0 - 2.0 * i / max_n_f).acos();
            let r = if is_highlighted { 90.0 } else { 80.0 };
            let point = Point3D::new(
                r * phi.sin() * theta.cos(),
                r * phi.cos(),
                r * phi.sin() * theta.sin(),
            );
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_trefoil_3d(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
    ) {
        let max_n = app.config.max_number;
        if max_n == 0 {
            return;
        }

        let highlights = app.highlights();
        let (rot_x, rot_y) = app.get_rotation();
        let max_n_f = max_n as f32;

        let mut projected: Vec<(f32, f32, f32, usize, bool)> = Vec::with_capacity(max_n);
        for n in 1..=max_n {
            let is_highlighted = highlights.contains(&n);
            let t = (n as f32 - 1.0) / max_n_f * 4.0 * std::f32::consts::PI;
            let r = if is_highlighted { 100.0 } else { 80.0 };
            let tube = if is_highlighted { 25.0 } else { 20.0 };
            let x = r * (2.0 * t).sin() + tube * t.sin() * (3.0 * t).cos();
            let y = r * (2.0 * t).cos() + tube * (3.0 * t).sin();
            let z = tube * t.cos() * (3.0 * t).cos();
            let point = Point3D::new(x, y, z);
            let (px, py, pz) = project_3d_to_2d(&point, rot_y, rot_x);
            projected.push((px, py, pz, n, is_highlighted));
        }

        Self::render_3d_projected(img, app, width, height, &mut projected);
    }

    fn render_3d_projected(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        app: &NumberVisualizerApp,
        width: u32,
        height: u32,
        projected: &mut Vec<(f32, f32, f32, usize, bool)>,
    ) {
        projected.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        let mut max_coord = 0.0f32;
        for (x, y, _, _, _) in projected.iter() {
            max_coord = max_coord.max(x.abs()).max(y.abs());
        }
        let available = (width as f32).min(height as f32) / 2.0 - 20.0;
        let scale = if max_coord > 0.0 {
            available / max_coord
        } else {
            1.0
        };

        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;

        for (x, y, depth, _n, is_highlighted) in projected {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + *y * scale;
            let df = depth_factor(*depth);

            if screen_x < 0.0
                || screen_x >= width as f32
                || screen_y < 0.0
                || screen_y >= height as f32
            {
                continue;
            }

            let base_color = if *is_highlighted {
                app.config.highlight_color
            } else {
                app.config.non_highlight_color
            };
            let color = adjust_brightness(base_color, df);

            let radius = (if *is_highlighted {
                app.config.highlight_size as f32 * df
            } else {
                app.config.non_highlight_size as f32 * df
            })
            .max(1.0);

            Self::draw_circle(img, screen_x as i32, screen_y as i32, radius as i32, color);
        }
    }

    fn draw_circle(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        cx: i32,
        cy: i32,
        r: i32,
        color: egui::Color32,
    ) {
        if r <= 0 {
            return;
        }
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < img.width() as i32 && py >= 0 && py < img.height() as i32 {
                        img.put_pixel(
                            px as u32,
                            py as u32,
                            Rgba([color.r(), color.g(), color.b(), color.a()]),
                        );
                    }
                }
            }
        }
    }

    fn draw_line(
        img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        width: f32,
        color: egui::Color32,
    ) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let steps = dx.max(dy).max(1.0) as i32;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = x1 + (x2 - x1) * t;
            let y = y1 + (y2 - y1) * t;
            let r = (width / 2.0).ceil() as i32;
            Self::draw_circle(img, x as i32, y as i32, r, color);
        }
    }

    fn calculate_bounds(positions: &[(usize, f32, f32)]) -> (f32, f32, f32, f32) {
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

    pub fn get_default_filename(app: &NumberVisualizerApp) -> String {
        let series_name = app.series_name().replace(' ', "_");
        let viz_name = format!("{}", app.config.visualization).replace(' ', "_");
        format!("{}_{}.png", series_name, viz_name)
    }
}
