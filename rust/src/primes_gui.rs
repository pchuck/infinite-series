use eframe::egui;
use primes::sieve_of_eratosthenes;
use std::collections::HashSet;

#[derive(Clone)]
struct SpiralConfig {
    max_number: usize,
    prime_size: f32,
    non_prime_size: f32,
    cell_spacing: f32,
    show_numbers: bool,
    prime_color: egui::Color32,
    non_prime_color: egui::Color32,
    background_color: egui::Color32,
}

impl Default for SpiralConfig {
    fn default() -> Self {
        Self {
            max_number: 10000,
            prime_size: 12.0,
            non_prime_size: 6.0,
            cell_spacing: 20.0,
            show_numbers: true,
            prime_color: egui::Color32::from_rgba_unmultiplied(255, 200, 50, 255),
            non_prime_color: egui::Color32::from_rgba_unmultiplied(100, 100, 100, 255),
            background_color: egui::Color32::from_rgba_unmultiplied(20, 20, 30, 255),
        }
    }
}

struct PrimeSpiralApp {
    config: SpiralConfig,
    primes: HashSet<usize>,
    needs_redraw: bool,
}

impl PrimeSpiralApp {
    fn new(config: SpiralConfig) -> Self {
        let primes = sieve_of_eratosthenes(config.max_number);
        let primes_set: HashSet<usize> = primes.into_iter().collect();

        Self {
            config,
            primes: primes_set,
            needs_redraw: true,
        }
    }

    fn regenerate_primes(&mut self) {
        self.primes = sieve_of_eratosthenes(self.config.max_number)
            .into_iter()
            .collect();
    }

    fn generate_spiral_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
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

    fn draw_spiral(&self, ui: &mut egui::Ui, rect: egui::Rect) {
        let positions = Self::generate_spiral_positions(self.config.max_number);

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

        let range_x = (max_x - min_x).max(1.0);
        let range_y = (max_y - min_y).max(1.0);

        let available_width = rect.width() - 40.0;
        let available_height = rect.height() - 40.0;

        let scale_x = available_width / range_x;
        let scale_y = available_height / range_y;
        let scale = scale_x.min(scale_y).min(self.config.cell_spacing * 2.0);

        let center_x = rect.center().x;
        let center_y = rect.center().y;

        let painter = ui.painter();

        for (n, x, y) in &positions {
            let screen_x = center_x + *x * scale;
            let screen_y = center_y + *y * scale;

            let is_prime = self.primes.contains(n);
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
            painter.circle_filled(egui::Pos2::new(screen_x, screen_y), circle_radius, color);

            if self.config.show_numbers && size >= 6.0 {
                let text = format!("{}", n);
                let font_id = egui::FontId::proportional(size * 0.6);
                painter.text(
                    egui::Pos2::new(screen_x, screen_y),
                    egui::Align2::CENTER_CENTER,
                    text,
                    font_id,
                    self.config.background_color,
                );
            }
        }
    }
}

impl eframe::App for PrimeSpiralApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls")
            .default_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Prime Spiral Settings");
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

                ui.label("Cell Spacing:");
                ui.add(egui::Slider::new(&mut self.config.cell_spacing, 5.0..=50.0));

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
                    self.needs_redraw = true;
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
            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, self.config.background_color);
            self.draw_spiral(ui, rect);
        });
    }
}

fn main() -> eframe::Result<()> {
    let config = SpiralConfig::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Prime Spiral Visualization",
        options,
        Box::new(|_cc| Ok(Box::new(PrimeSpiralApp::new(config)))),
    )
}
