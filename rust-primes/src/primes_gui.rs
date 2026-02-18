use eframe::egui;
use primes::generate_primes;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum VisualizationType {
    #[default]
    UlamSpiral,
    SacksSpiral,
    Grid,
    Row,
    PrimeWheel,
    PrimeDensity,
    RiemannZeta,
    HexagonalLattice,
    TriangularLattice,
    FermatsSpiral,
    SacksMobiusSpiral,
    UlamMobiusSpiral,
    PrimeDensityGradient,
}

impl VisualizationType {
    fn uses_point_size(self) -> bool {
        matches!(
            self,
            Self::UlamSpiral
                | Self::SacksSpiral
                | Self::Grid
                | Self::Row
                | Self::PrimeWheel
                | Self::HexagonalLattice
                | Self::TriangularLattice
                | Self::FermatsSpiral
                | Self::SacksMobiusSpiral
                | Self::UlamMobiusSpiral
                | Self::PrimeDensityGradient
        )
    }

    fn supports_twin_primes(self) -> bool {
        matches!(
            self,
            Self::UlamSpiral
                | Self::SacksSpiral
                | Self::Grid
                | Self::Row
                | Self::HexagonalLattice
                | Self::TriangularLattice
                | Self::FermatsSpiral
                | Self::SacksMobiusSpiral
                | Self::UlamMobiusSpiral
                | Self::PrimeDensityGradient
        )
    }

    fn uses_modulo(self) -> bool {
        matches!(self, Self::PrimeWheel)
    }

    fn uses_num_zeros(self) -> bool {
        matches!(self, Self::RiemannZeta)
    }

    fn uses_grid_size(self) -> bool {
        matches!(self, Self::PrimeDensityGradient)
    }

    fn description(self) -> &'static str {
        match self {
            Self::UlamSpiral => "Classic diagonal prime pattern on a square grid spiral",
            Self::SacksSpiral => "Archimedean spiral (r = sqrt(n)) revealing curved patterns",
            Self::Grid => "Simple Cartesian square grid layout",
            Self::Row => "Single horizontal number line",
            Self::PrimeWheel => "Concentric rings colored by modulo residue",
            Self::PrimeDensity => "Graph of pi(x) vs x/ln(x) - Prime Number Theorem",
            Self::RiemannZeta => "Critical strip showing non-trivial zeros on sigma=0.5",
            Self::HexagonalLattice => "6-direction symmetric spiral on hexagonal grid",
            Self::TriangularLattice => "3-direction symmetric spiral on triangular grid",
            Self::FermatsSpiral => "Phyllotaxis spiral with golden angle (sunflower pattern)",
            Self::SacksMobiusSpiral => {
                "Archimedean spiral with gap-colored lines between consecutive primes"
            }
            Self::UlamMobiusSpiral => {
                "Square-grid spiral with gap-colored lines between consecutive primes"
            }
            Self::PrimeDensityGradient => "Heatmap grid showing local prime density",
        }
    }
}

impl std::fmt::Display for VisualizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisualizationType::UlamSpiral => write!(f, "Ulam Spiral"),
            VisualizationType::SacksSpiral => write!(f, "Sacks Spiral"),
            VisualizationType::Grid => write!(f, "Grid"),
            VisualizationType::Row => write!(f, "Row"),
            VisualizationType::PrimeWheel => write!(f, "Prime Wheel"),
            VisualizationType::PrimeDensity => write!(f, "Prime Density"),
            VisualizationType::RiemannZeta => write!(f, "Riemann Zeta"),
            VisualizationType::HexagonalLattice => write!(f, "Hexagonal Lattice"),
            VisualizationType::TriangularLattice => write!(f, "Triangular Lattice"),
            VisualizationType::FermatsSpiral => write!(f, "Fermat's Spiral"),
            VisualizationType::SacksMobiusSpiral => write!(f, "Sacks Mobius Spiral"),
            VisualizationType::UlamMobiusSpiral => write!(f, "Ulam Mobius Spiral"),
            VisualizationType::PrimeDensityGradient => write!(f, "Prime Density Gradient"),
        }
    }
}

#[derive(Clone)]
struct VisualizerConfig {
    max_number: usize,
    prime_size: usize,
    non_prime_size: usize,
    modulo: usize,
    show_numbers: bool,
    prime_color: egui::Color32,
    non_prime_color: egui::Color32,
    background_color: egui::Color32,
    visualization: VisualizationType,
    num_zeros: usize,
    show_twin_primes: bool,
    twin_color: egui::Color32,
    show_cousin_primes: bool,
    cousin_color: egui::Color32,
    show_sexy_primes: bool,
    sexy_color: egui::Color32,
    grid_size: usize,
}

impl Default for VisualizerConfig {
    fn default() -> Self {
        Self {
            max_number: 10000,
            prime_size: 2,
            non_prime_size: 1,
            modulo: 30,
            show_numbers: false,
            prime_color: egui::Color32::from_rgba_unmultiplied(255, 220, 80, 255),
            non_prime_color: egui::Color32::from_rgba_unmultiplied(60, 60, 70, 180),
            background_color: egui::Color32::from_rgba_unmultiplied(20, 20, 30, 255),
            visualization: VisualizationType::UlamSpiral,
            num_zeros: 10,
            show_twin_primes: false,
            twin_color: egui::Color32::from_rgba_unmultiplied(255, 50, 50, 255),
            show_cousin_primes: false,
            cousin_color: egui::Color32::from_rgba_unmultiplied(255, 120, 120, 255),
            show_sexy_primes: false,
            sexy_color: egui::Color32::from_rgba_unmultiplied(255, 180, 180, 255),
            grid_size: 40,
        }
    }
}

struct PrimeVisualizerApp {
    config: VisualizerConfig,
    primes: HashSet<usize>,
    primes_vec: Vec<usize>,
    max_pixels: usize,
    cached_max_number: usize,
}

impl PrimeVisualizerApp {
    fn new(config: VisualizerConfig) -> Self {
        let max_number = config.max_number;
        let primes_vec = generate_primes(max_number, false, None, None, None);
        let primes_set: HashSet<usize> = primes_vec.iter().copied().collect();

        Self {
            config,
            primes: primes_set,
            primes_vec,
            max_pixels: 1_000_000,
            cached_max_number: max_number,
        }
    }

    fn regenerate_primes(&mut self) {
        // Only regenerate if max_number has actually changed
        if self.config.max_number != self.cached_max_number {
            self.primes_vec = generate_primes(self.config.max_number, false, None, None, None);
            self.primes = self.primes_vec.iter().copied().collect();
            self.cached_max_number = self.config.max_number;
        }
    }

    fn generate_ulam_spiral_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
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

    fn generate_grid_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
        let side = (max_n as f32).sqrt() as usize + 1;
        (1..=max_n)
            .map(|n| {
                let row = (n - 1) / side;
                let col = (n - 1) % side;
                (n, col as f32, row as f32)
            })
            .collect()
    }

    fn generate_row_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
        (1..=max_n).map(|n| (n, n as f32, 0.0)).collect()
    }

    fn generate_sacks_spiral_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
        (1..=max_n)
            .map(|n| {
                let n_f = n as f32;
                let r = n_f.sqrt();
                let theta = n_f * 0.5;
                let x = r * theta.cos();
                let y = r * theta.sin();
                (n, x, y)
            })
            .collect()
    }

    fn draw_number(&self, n: usize, x: f32, y: f32, painter: &egui::Painter) {
        let is_prime = self.primes.contains(&n);

        let is_twin_prime = is_prime
            && self.config.show_twin_primes
            && (self.primes.contains(&(n + 2)) || (n > 2 && self.primes.contains(&(n - 2))));

        let is_cousin_prime = is_prime
            && self.config.show_cousin_primes
            && !is_twin_prime
            && (self.primes.contains(&(n + 4)) || (n > 4 && self.primes.contains(&(n - 4))));

        let is_sexy_prime = is_prime
            && self.config.show_sexy_primes
            && !is_twin_prime
            && !is_cousin_prime
            && (self.primes.contains(&(n + 6)) || (n > 6 && self.primes.contains(&(n - 6))));

        let size = if is_prime {
            self.config.prime_size as f32
        } else {
            self.config.non_prime_size as f32
        };

        // Skip if size is zero
        if size == 0.0 {
            return;
        }

        let color = if is_twin_prime {
            self.config.twin_color
        } else if is_cousin_prime {
            self.config.cousin_color
        } else if is_sexy_prime {
            self.config.sexy_color
        } else if is_prime {
            self.config.prime_color
        } else {
            self.config.non_prime_color
        };

        // Always use circle for consistent rendering
        let radius = size / 2.0;
        painter.circle_filled(egui::Pos2::new(x, y), radius.max(0.5), color);

        // Auto-hide text when there are too many numbers
        let show_text = self.config.show_numbers && size >= 6.0 && self.config.max_number <= 10000;

        if show_text {
            let text = format!("{}", n);
            let font_id = egui::FontId::proportional(size * 0.6);
            painter.text(
                egui::Pos2::new(x, y),
                egui::Align2::CENTER_CENTER,
                text,
                font_id,
                self.config.background_color,
            );
        }
    }

    fn draw_visualization(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, self.config.background_color);

        match self.config.visualization {
            VisualizationType::UlamSpiral => self.draw_ulam_spiral(ui, rect),
            VisualizationType::SacksSpiral => self.draw_sacks_spiral(ui, rect),
            VisualizationType::Grid => self.draw_grid(ui, rect),
            VisualizationType::Row => self.draw_row(ui, rect),
            VisualizationType::PrimeWheel => self.draw_prime_wheel(ui, rect),
            VisualizationType::PrimeDensity => self.draw_prime_density(ui, rect),
            VisualizationType::RiemannZeta => self.draw_riemann_zeta(ui, rect),
            VisualizationType::HexagonalLattice => self.draw_hexagonal_spiral(ui, rect),
            VisualizationType::TriangularLattice => self.draw_triangle_spiral(ui, rect),
            VisualizationType::FermatsSpiral => self.draw_fermats_spiral(ui, rect),
            VisualizationType::SacksMobiusSpiral => self.draw_sacks_mobius_spiral(ui, rect),
            VisualizationType::UlamMobiusSpiral => self.draw_ulam_mobius_spiral(ui, rect),
            VisualizationType::PrimeDensityGradient => self.draw_prime_density_gradient(ui, rect),
        }
    }

    fn generate_prime_wheel_positions(max_n: usize, modulo: usize) -> Vec<(usize, f32, f32)> {
        (1..=max_n)
            .map(|n| {
                let remainder = n % modulo;
                let quotient = n / modulo;

                let theta = remainder as f32 * 2.0 * std::f32::consts::PI / modulo as f32
                    - std::f32::consts::PI / 2.0;
                let r = quotient as f32 + 1.0;

                let x = r * theta.cos();
                let y = r * theta.sin();
                (n, x, y)
            })
            .collect()
    }

    fn draw_prime_wheel(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions =
            Self::generate_prime_wheel_positions(self.config.max_number, self.config.modulo);

        if positions.is_empty() {
            return;
        }

        let modulo = self.config.modulo as f32;
        let max_ring = (self.config.max_number / self.config.modulo) as f32 + 2.0;

        let available = rect.width().min(rect.height()) / 2.0 - 20.0;
        let scale = if max_ring > 0.0 {
            available / max_ring
        } else {
            available
        };

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for (n, _, _) in &positions {
            let remainder = *n % self.config.modulo;
            let quotient = *n / self.config.modulo;

            let theta =
                remainder as f32 * 2.0 * std::f32::consts::PI / modulo - std::f32::consts::PI / 2.0;
            let r = (quotient as f32 + 1.0) * scale;

            let screen_x = center_x + r * theta.cos();
            let screen_y = center_y + r * theta.sin();
            self.draw_number(*n, screen_x, screen_y, painter);
        }

        for spoke in 0..self.config.modulo {
            let theta =
                spoke as f32 * 2.0 * std::f32::consts::PI / modulo - std::f32::consts::PI / 2.0;
            let inner_r = scale;
            let outer_r = max_ring * scale;

            let start_x = center_x + inner_r * theta.cos();
            let start_y = center_y + inner_r * theta.sin();
            let end_x = center_x + outer_r * theta.cos();
            let end_y = center_y + outer_r * theta.sin();

            painter.line_segment(
                [
                    egui::Pos2::new(start_x, start_y),
                    egui::Pos2::new(end_x, end_y),
                ],
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(50, 50, 50, 100)),
            );
        }
    }

    fn draw_prime_density(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let max_n = self.config.max_number;
        if max_n < 10 {
            return;
        }

        let prime_count = self.primes_vec.len();

        let intervals = 100_usize.max(max_n / 100);
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

            while prime_idx < prime_count && self.primes_vec[prime_idx] <= x {
                count += 1;
                prime_idx += 1;
            }

            let ln_x = (x as f64).ln().max(1.0) as f32;
            let x_ln_x_val = x as f32 / ln_x;

            pi_x.push((x as f32, count as f32));
            x_ln_x.push((x as f32, x_ln_x_val));
        }

        let max_y = pi_x.last().map(|(_, y)| *y).unwrap_or(1.0).max(1.0);

        let margin = 50.0;
        let graph_left = rect.left() + margin;
        let graph_right = rect.right() - margin;
        let graph_top = rect.top() + margin;
        let graph_bottom = rect.bottom() - margin;
        let graph_width = graph_right - graph_left;
        let graph_height = graph_bottom - graph_top;

        let painter = ui.painter();

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

        let max_x = max_n as f32;

        for i in 0..x_ln_x.len() - 1 {
            let (x1, y1) = x_ln_x[i];
            let (x2, y2) = x_ln_x[i + 1];

            let px1 = graph_left + (x1 / max_x) * graph_width;
            let py1 = graph_bottom - (y1 / max_y) * graph_height;
            let px2 = graph_left + (x2 / max_x) * graph_width;
            let py2 = graph_bottom - (y2 / max_y) * graph_height;

            painter.line_segment(
                [egui::Pos2::new(px1, py1), egui::Pos2::new(px2, py2)],
                egui::Stroke::new(2.0, self.config.non_prime_color),
            );
        }

        for i in 0..pi_x.len() - 1 {
            let (x1, y1) = pi_x[i];
            let (x2, y2) = pi_x[i + 1];

            let px1 = graph_left + (x1 / max_x) * graph_width;
            let py1 = graph_bottom - (y1 / max_y) * graph_height;
            let px2 = graph_left + (x2 / max_x) * graph_width;
            let py2 = graph_bottom - (y2 / max_y) * graph_height;

            painter.line_segment(
                [egui::Pos2::new(px1, py1), egui::Pos2::new(px2, py2)],
                egui::Stroke::new(2.0, self.config.prime_color),
            );
        }
    }

    fn draw_riemann_zeta(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let margin = 60.0;
        let graph_left = rect.left() + margin;
        let graph_right = rect.right() - margin;
        let graph_top = rect.top() + margin;
        let graph_bottom = rect.bottom() - margin;
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

        let max_imag = (self.config.max_number as f32 / 10.0).max(50.0);

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

        let num_zeros_to_show = self.config.num_zeros.min(zeros.len());
        let zero_radius = 4.0;

        for (i, &imag) in zeros.iter().enumerate().take(num_zeros_to_show) {
            let imag = imag as f32;
            if imag > max_imag {
                break;
            }

            let x = critical_line_x;
            let y = graph_bottom - (imag / max_imag) * graph_height;

            painter.circle_filled(egui::Pos2::new(x, y), zero_radius, self.config.prime_color);

            if i < 10 || num_zeros_to_show <= 20 {
                let label = format!("{:.1}", imag);
                painter.text(
                    egui::Pos2::new(x + 8.0, y - 6.0),
                    egui::Align2::LEFT_BOTTOM,
                    label,
                    egui::FontId::proportional(9.0),
                    self.config.prime_color,
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

    fn generate_triangle_spiral_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
        let mut positions = Vec::with_capacity(max_n);

        if max_n == 0 {
            return positions;
        }

        let mut x = 0i32;
        let mut y = 0i32;

        let tri_directions: [(i32, i32); 3] = [
            (2, 0),   // East (0°)
            (-1, 2),  // 120° from East
            (-1, -2), // 240° from East
        ];

        let mut steps_in_direction = 1;
        let mut steps_since_turn = 0;
        let mut turn_count = 0;
        let mut dir_idx = 0;

        for n in 1..=max_n {
            positions.push((n, x as f32, y as f32));

            if n == max_n {
                break;
            }

            x += tri_directions[dir_idx].0;
            y += tri_directions[dir_idx].1;
            steps_since_turn += 1;

            if steps_since_turn == steps_in_direction {
                steps_since_turn = 0;
                dir_idx = (dir_idx + 1) % 3;
                turn_count += 1;
                if turn_count % 2 == 0 {
                    steps_in_direction += 1;
                }
            }
        }

        positions
    }

    fn draw_triangle_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_triangle_spiral_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for (_, x, y) in &positions {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        let range_x = max_x - min_x;
        let range_y = max_y - min_y;

        let margin = 20.0;
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
        let scale = scale_x.min(scale_y);

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for (n, x, y) in &positions {
            let screen_x = center_x + (*x - (min_x + max_x) / 2.0) * scale;
            let screen_y = center_y - (*y - (min_y + max_y) / 2.0) * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn generate_hexagonal_spiral_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
        let mut positions = Vec::with_capacity(max_n);

        if max_n == 0 {
            return positions;
        }

        let mut x = 0i32;
        let mut y = 0i32;

        let hex_directions: [(i32, i32); 6] = [
            (2, 0),   // East
            (1, 2),   // Southeast (60°)
            (-1, 2),  // Southwest (120°)
            (-2, 0),  // West (180°)
            (-1, -2), // Northwest (240°)
            (1, -2),  // Northeast (300°)
        ];

        let mut steps_in_direction = 1;
        let mut steps_since_turn = 0;
        let mut turn_count = 0;
        let mut dir_idx = 0;

        for n in 1..=max_n {
            positions.push((n, x as f32, y as f32));

            if n == max_n {
                break;
            }

            x += hex_directions[dir_idx].0;
            y += hex_directions[dir_idx].1;
            steps_since_turn += 1;

            if steps_since_turn == steps_in_direction {
                steps_since_turn = 0;
                dir_idx = (dir_idx + 1) % 6;
                turn_count += 1;
                if turn_count % 2 == 0 {
                    steps_in_direction += 1;
                }
            }
        }

        positions
    }

    fn draw_hexagonal_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_hexagonal_spiral_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for (_, x, y) in &positions {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        let range_x = max_x - min_x;
        let range_y = max_y - min_y;

        let margin = 20.0;
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
        let scale = scale_x.min(scale_y);

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for (n, x, y) in &positions {
            let screen_x = center_x + (*x - (min_x + max_x) / 2.0) * scale;
            let screen_y = center_y - (*y - (min_y + max_y) / 2.0) * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn generate_fermats_spiral_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
        (1..=max_n)
            .map(|n| {
                let n_f = n as f32;
                let r = n_f.sqrt();
                let theta = n_f * 2.39996_f32; // golden angle in radians
                let x = r * theta.cos();
                let y = r * theta.sin();
                (n, x, y)
            })
            .collect()
    }

    fn draw_fermats_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_fermats_spiral_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let mut max_r = 0.0f32;
        for (_, x, y) in &positions {
            let r = (x * x + y * y).sqrt();
            max_r = max_r.max(r);
        }

        let available = rect.width().min(rect.height()) / 2.0 - 20.0;
        let scale = if max_r > 0.0 { available / max_r } else { 1.0 };

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for (n, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y - *y * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn gap_color(gap: usize) -> egui::Color32 {
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

    fn draw_sacks_mobius_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        if self.primes_vec.len() < 2 {
            return;
        }

        let positions: Vec<(usize, f32, f32)> = self
            .primes_vec
            .iter()
            .enumerate()
            .map(|(idx, &n)| {
                let idx_f = idx as f32;
                let r = idx_f * 0.8; // linear radius increase
                let theta = idx_f * 0.5; // fixed angle step
                let x = r * theta.cos();
                let y = r * theta.sin();
                (n, x, y)
            })
            .collect();

        let mut max_coord = 0.0f32;
        for (_, x, y) in &positions {
            max_coord = max_coord.max(x.abs()).max(y.abs());
        }

        let margin = 20.0;
        let available = rect.width().min(rect.height()) / 2.0 - margin;
        let scale = if max_coord > 0.0 {
            available / max_coord
        } else {
            1.0
        };

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for i in 0..positions.len() - 1 {
            let (_, x1, y1) = positions[i];
            let (_, x2, y2) = positions[i + 1];
            let gap = positions[i + 1].0 - positions[i].0;

            let screen_x1 = center_x + x1 * scale;
            let screen_y1 = center_y - y1 * scale;
            let screen_x2 = center_x + x2 * scale;
            let screen_y2 = center_y - y2 * scale;

            let color = Self::gap_color(gap);
            let stroke_width = if gap <= 4 {
                2.5
            } else if gap <= 6 {
                2.0
            } else if gap <= 10 {
                1.5
            } else if gap <= 20 {
                1.0
            } else {
                0.5
            };

            painter.line_segment(
                [
                    egui::Pos2::new(screen_x1, screen_y1),
                    egui::Pos2::new(screen_x2, screen_y2),
                ],
                egui::Stroke::new(stroke_width, color),
            );
        }

        for (n, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y - *y * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn draw_ulam_mobius_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        if self.primes_vec.len() < 2 {
            return;
        }

        // Use Ulam spiral positions based on prime index (O(n) instead of O(n²))
        // Generate positions only up to the number of primes we have
        let spiral_positions = Self::generate_ulam_spiral_positions(self.primes_vec.len());
        let positions: Vec<(usize, f32, f32)> = self
            .primes_vec
            .iter()
            .enumerate()
            .map(|(idx, &n)| {
                let (_, x, y) = spiral_positions[idx];
                (n, x, y)
            })
            .collect();

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        for (_, x, y) in &positions {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        let range_x = max_x - min_x;
        let range_y = max_y - min_y;

        let margin = 20.0;
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
        let scale = scale_x.min(scale_y);

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for i in 0..positions.len() - 1 {
            let (_, x1, y1) = positions[i];
            let (_, x2, y2) = positions[i + 1];
            let gap = positions[i + 1].0 - positions[i].0;

            let screen_x1 = center_x + (x1 - (min_x + max_x) / 2.0) * scale;
            let screen_y1 = center_y - (y1 - (min_y + max_y) / 2.0) * scale;
            let screen_x2 = center_x + (x2 - (min_x + max_x) / 2.0) * scale;
            let screen_y2 = center_y - (y2 - (min_y + max_y) / 2.0) * scale;

            let color = Self::gap_color(gap);
            let stroke_width = if gap <= 4 {
                2.5
            } else if gap <= 6 {
                2.0
            } else if gap <= 10 {
                1.5
            } else if gap <= 20 {
                1.0
            } else {
                0.5
            };

            painter.line_segment(
                [
                    egui::Pos2::new(screen_x1, screen_y1),
                    egui::Pos2::new(screen_x2, screen_y2),
                ],
                egui::Stroke::new(stroke_width, color),
            );
        }

        for (n, x, y) in &positions {
            let screen_x = center_x + (*x - (min_x + max_x) / 2.0) * scale;
            let screen_y = center_y - (*y - (min_y + max_y) / 2.0) * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn draw_prime_density_gradient(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        if self.primes_vec.is_empty() {
            return;
        }

        let margin = 10.0;
        let graph_left = rect.left() + margin;
        let graph_right = rect.right() - margin;
        let graph_top = rect.top() + margin;
        let graph_bottom = rect.bottom() - margin;
        let graph_width = graph_right - graph_left;
        let graph_height = graph_bottom - graph_top;

        let painter = ui.painter();
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::Pos2::new(graph_left, graph_top),
                egui::vec2(graph_width, graph_height),
            ),
            0.0,
            self.config.background_color,
        );

        let grid_size = self.config.grid_size;
        let cell_width = graph_width / grid_size as f32;
        let cell_height = graph_height / grid_size as f32;

        let mut density_grid = vec![0.0_f32; grid_size * grid_size];

        for &p in &self.primes_vec {
            let x_frac = p as f32 / self.config.max_number as f32;
            let y_frac = (p * p % self.config.max_number) as f32 / self.config.max_number as f32;

            let grid_x = ((x_frac * grid_size as f32) as usize).min(grid_size - 1);
            let grid_y = ((y_frac * grid_size as f32) as usize).min(grid_size - 1);

            let idx = grid_y * grid_size + grid_x;
            density_grid[idx] += 1.0;
        }

        let max_density = density_grid.iter().cloned().fold(0.0_f32, f32::max);

        for gy in 0..grid_size {
            for gx in 0..grid_size {
                let idx = gy * grid_size + gx;
                let density = density_grid[idx];
                let normalized = if max_density > 0.0 {
                    density / max_density
                } else {
                    0.0
                };

                let r = (self.config.prime_color.r() as f32 * normalized) as u8;
                let g = (self.config.prime_color.g() as f32 * normalized) as u8;
                let b = (self.config.prime_color.b() as f32 * normalized) as u8;

                let color = egui::Color32::from_rgba_unmultiplied(r, g, b, 255);

                let x = graph_left + gx as f32 * cell_width;
                let y = graph_top + gy as f32 * cell_height;

                painter.rect_filled(
                    egui::Rect::from_min_size(
                        egui::Pos2::new(x, y),
                        egui::vec2(cell_width, cell_height),
                    ),
                    0.0,
                    color,
                );
            }
        }
    }

    fn draw_ulam_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_ulam_spiral_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let mut max_coord = 0.0f32;
        for (_, x, y) in &positions {
            max_coord = max_coord.max(x.abs()).max(y.abs());
        }

        let available = rect.width().min(rect.height()) / 2.0 - 20.0;

        // Scale to always fit within bounds
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
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn draw_sacks_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_sacks_spiral_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let mut max_r = 0.0f32;
        for (_, x, y) in &positions {
            let r = (x * x + y * y).sqrt();
            max_r = max_r.max(r);
        }

        let available = rect.width().min(rect.height()) / 2.0 - 20.0;
        let scale = if max_r > 0.0 { available / max_r } else { 1.0 };

        let center_x = rect.center().x;
        let center_y = rect.center().y;
        let painter = ui.painter();

        for (n, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + *y * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn draw_grid(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_grid_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let side = (self.config.max_number as f32).sqrt() as usize + 1;
        let available_width = rect.width() - 40.0;
        let available_height = rect.height() - 40.0;

        let scale = available_width.min(available_height) / side as f32;

        let start_x = rect.left() + 20.0 + scale / 2.0;
        let start_y = rect.top() + 20.0 + scale / 2.0;
        let painter = ui.painter();

        for (n, x, y) in &positions {
            let screen_x = start_x + *x * scale;
            let screen_y = start_y + *y * scale;
            self.draw_number(*n, screen_x, screen_y, painter);
        }
    }

    fn draw_row(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_row_positions(self.config.max_number);

        if positions.is_empty() {
            return;
        }

        let max_x = self.config.max_number as f32;

        let available_width = rect.width() - 40.0;
        let scale = available_width / max_x;

        let center_y = rect.center().y;
        let start_x = rect.left() + 20.0 + scale / 2.0;
        let painter = ui.painter();

        for (n, x, _) in &positions {
            let screen_x = start_x + *x * scale;
            self.draw_number(*n, screen_x, center_y, painter);
        }
    }
}

impl eframe::App for PrimeVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Get available canvas size for max_number limit
        let screen_rect = ctx.available_rect();
        let pixels = (screen_rect.width() * screen_rect.height()) as usize;
        self.max_pixels = (pixels / 2).max(100);

        // Clamp max_number if it exceeds canvas pixels
        if self.config.max_number > self.max_pixels {
            self.config.max_number = self.max_pixels;
        }

        egui::SidePanel::left("controls")
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Prime Visualizer");
                ui.separator();

                ui.label("Visualization:");
                egui::ComboBox::from_id_salt("viz_type")
                    .selected_text(format!("{}", self.config.visualization))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::UlamSpiral,
                            "Ulam Spiral",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::SacksSpiral,
                            "Sacks Spiral",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::Grid,
                            "Grid",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::Row,
                            "Row",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::PrimeWheel,
                            "Prime Wheel",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::PrimeDensity,
                            "Prime Density",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::RiemannZeta,
                            "Riemann Zeta",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::HexagonalLattice,
                            "Hexagonal Lattice",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::TriangularLattice,
                            "Triangular Lattice",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::FermatsSpiral,
                            "Fermat's Spiral",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::SacksMobiusSpiral,
                            "Sacks Mobius Spiral",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::UlamMobiusSpiral,
                            "Ulam Mobius Spiral",
                        );
                        ui.selectable_value(
                            &mut self.config.visualization,
                            VisualizationType::PrimeDensityGradient,
                            "Prime Density Gradient",
                        );
                    });

                ui.separator();

                ui.label("Max Number:");
                ui.add(
                    egui::Slider::new(&mut self.config.max_number, 100..=self.max_pixels)
                        .step_by(100.0),
                );

                ui.separator();

                let show_point_controls = self.config.visualization.uses_point_size();

                if show_point_controls {
                    ui.label("Prime Size:");
                    ui.add(egui::Slider::new(&mut self.config.prime_size, 1..=50));

                    ui.label("Non-Prime Size:");
                    ui.add(egui::Slider::new(&mut self.config.non_prime_size, 0..=30));
                }

                if self.config.visualization.uses_modulo() {
                    ui.label("Modulo:");
                    ui.add(egui::Slider::new(&mut self.config.modulo, 2..=210).step_by(2.0));
                }

                if self.config.visualization.uses_num_zeros() {
                    ui.label("Zeros:");
                    ui.add(egui::Slider::new(&mut self.config.num_zeros, 1..=20));
                }

                if self.config.visualization.uses_grid_size() {
                    ui.label("Grid Size:");
                    ui.add(egui::Slider::new(&mut self.config.grid_size, 10..=100).step_by(5.0));
                }

                if show_point_controls {
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Show Numbers");
                        ui.checkbox(&mut self.config.show_numbers, "");
                    });
                }

                if self.config.visualization.supports_twin_primes() {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Highlight Twin Primes");
                        ui.checkbox(&mut self.config.show_twin_primes, "");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Highlight Cousin Primes");
                        ui.checkbox(&mut self.config.show_cousin_primes, "");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Highlight Sexy Primes");
                        ui.checkbox(&mut self.config.show_sexy_primes, "");
                    });
                }

                ui.separator();
                ui.label("Prime Color");
                ui.color_edit_button_srgba(&mut self.config.prime_color);

                ui.label("Non-Prime Color");
                ui.color_edit_button_srgba(&mut self.config.non_prime_color);

                if self.config.show_twin_primes {
                    ui.label("Twin Color");
                    ui.color_edit_button_srgba(&mut self.config.twin_color);
                }

                if self.config.show_cousin_primes {
                    ui.label("Cousin Color");
                    ui.color_edit_button_srgba(&mut self.config.cousin_color);
                }

                if self.config.show_sexy_primes {
                    ui.label("Sexy Color");
                    ui.color_edit_button_srgba(&mut self.config.sexy_color);
                }

                ui.label("Background Color");
                ui.color_edit_button_srgba(&mut self.config.background_color);

                ui.separator();

                if ui.button("Apply Changes").clicked() {
                    self.regenerate_primes();
                }

                ui.separator();
                ui.label(
                    egui::RichText::new(format!("Total Primes: {}", self.primes.len()))
                        .font(egui::FontId::proportional(12.0)),
                );

                ui.label(
                    egui::RichText::new(format!("Showing: 1 to {}", self.config.max_number))
                        .font(egui::FontId::proportional(12.0)),
                );

                ui.separator();
                ui.label(
                    egui::RichText::new(format!(
                        "{}:\n{}",
                        self.config.visualization,
                        self.config.visualization.description()
                    ))
                    .font(egui::FontId::proportional(12.0))
                    .italics(),
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_visualization(ui, ui.available_rect_before_wrap());
        });
    }
}

fn main() -> eframe::Result<()> {
    let config = VisualizerConfig::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Prime Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(PrimeVisualizerApp::new(config)))),
    )
}
