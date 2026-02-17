use eframe::egui;
use primes::sieve_of_eratosthenes;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisualizationType {
    UlamSpiral,
    SacksSpiral,
    Grid,
    Row,
    PrimeWheel,
}

impl VisualizationType {
    fn uses_cell_spacing(self) -> bool {
        matches!(self, Self::UlamSpiral | Self::SacksSpiral | Self::Grid)
    }

    fn uses_modulo(self) -> bool {
        matches!(self, Self::PrimeWheel)
    }
}

impl Default for VisualizationType {
    fn default() -> Self {
        Self::UlamSpiral
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
        }
    }
}

#[derive(Clone)]
struct VisualizerConfig {
    max_number: usize,
    prime_size: f32,
    non_prime_size: f32,
    cell_spacing: f32,
    modulo: usize,
    show_numbers: bool,
    prime_color: egui::Color32,
    non_prime_color: egui::Color32,
    background_color: egui::Color32,
    visualization: VisualizationType,
}

impl Default for VisualizerConfig {
    fn default() -> Self {
        Self {
            max_number: 10000,
            prime_size: 2.0,
            non_prime_size: 1.0,
            cell_spacing: 20.0,
            modulo: 30,
            show_numbers: true,
            prime_color: egui::Color32::from_rgba_unmultiplied(255, 200, 50, 255),
            non_prime_color: egui::Color32::from_rgba_unmultiplied(100, 100, 100, 255),
            background_color: egui::Color32::from_rgba_unmultiplied(20, 20, 30, 255),
            visualization: VisualizationType::UlamSpiral,
        }
    }
}

struct PrimeVisualizerApp {
    config: VisualizerConfig,
    primes: HashSet<usize>,
}

impl PrimeVisualizerApp {
    fn new(config: VisualizerConfig) -> Self {
        let primes = sieve_of_eratosthenes(config.max_number);
        let primes_set: HashSet<usize> = primes.into_iter().collect();

        Self {
            config,
            primes: primes_set,
        }
    }

    fn regenerate_primes(&mut self) {
        self.primes = sieve_of_eratosthenes(self.config.max_number)
            .into_iter()
            .collect();
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
        let size = if is_prime {
            self.config.prime_size
        } else {
            self.config.non_prime_size
        };
        let color = if is_prime {
            self.config.prime_color
        } else {
            self.config.non_prime_color
        };

        let circle_radius = size / 2.0;
        painter.circle_filled(egui::Pos2::new(x, y), circle_radius, color);

        if self.config.show_numbers && size >= 6.0 {
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
        let ring_spacing = if max_ring > 0.0 {
            available / max_ring
        } else {
            available
        };
        let scale = ring_spacing.min(self.config.cell_spacing);

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
        let scale = if max_coord > 0.0 {
            available / max_coord
        } else {
            1.0
        };
        let scale = scale.min(self.config.cell_spacing);

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
        let scale = scale.min(self.config.cell_spacing);

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

        let auto_scale = available_width.min(available_height) / side as f32;
        let scale = auto_scale.min(self.config.cell_spacing);

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
                    });

                ui.separator();

                ui.label("Max Number:");
                ui.add(egui::Slider::new(&mut self.config.max_number, 100..=100000).step_by(100.0));

                ui.separator();
                ui.label("Prime Size:");
                ui.add(egui::Slider::new(&mut self.config.prime_size, 2.0..=50.0));

                ui.label("Non-Prime Size:");
                ui.add(egui::Slider::new(
                    &mut self.config.non_prime_size,
                    1.0..=30.0,
                ));

                if self.config.visualization.uses_cell_spacing() {
                    ui.label("Cell Spacing:");
                    ui.add(egui::Slider::new(&mut self.config.cell_spacing, 2.0..=50.0));
                }

                if self.config.visualization.uses_modulo() {
                    ui.label("Modulo:");
                    ui.add(egui::Slider::new(&mut self.config.modulo, 2..=210).step_by(2.0));
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Show Numbers");
                    ui.checkbox(&mut self.config.show_numbers, "");
                });

                ui.separator();
                ui.label("Prime Color");
                ui.color_edit_button_srgba(&mut self.config.prime_color);

                ui.label("Non-Prime Color");
                ui.color_edit_button_srgba(&mut self.config.non_prime_color);

                ui.label("Background Color");
                ui.color_edit_button_srgba(&mut self.config.background_color);

                ui.separator();

                if ui.button("Apply Changes").clicked() {
                    self.regenerate_primes();
                }

                ui.separator();
                ui.label(
                    egui::RichText::new(format!("Total Primes: {}", self.primes.len())).small(),
                );

                ui.label(
                    egui::RichText::new(format!("Showing: 1 to {}", self.config.max_number))
                        .small(),
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
