//! Main application and UI

use eframe::egui;
use primes::generate_primes;
use series::{
    generate_catalan_up_to, generate_collatz_times_up_to, generate_fibonacci_up_to,
    generate_happy_up_to, generate_hexagonal_up_to, generate_lucas_up_to,
    generate_powers_of_2_up_to, generate_triangular_up_to,
};
use std::collections::HashSet;
use std::sync::LazyLock;

use crate::config::VisualizerConfig;
use crate::config::{MAX_NUMBER_MAX, MAX_NUMBER_MIN, SIDE_PANEL_MIN_WIDTH};
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations as viz;
use crate::visualizations::density_gradient::GRID_SIZE_MAX;
use crate::visualizations::density_gradient::GRID_SIZE_MIN;
use crate::visualizations::helix_3d::ROTATION_X_DEFAULT;
use crate::visualizations::prime_wheel::MODULO_MAX;
use crate::visualizations::prime_wheel::MODULO_MIN;
use crate::visualizations::riemann::NUM_ZEROS_MAX;
use crate::visualizations::riemann::NUM_ZEROS_MIN;

static EMPTY_SET: LazyLock<HashSet<usize>> = LazyLock::new(HashSet::new);
static EMPTY_VEC: LazyLock<Vec<usize>> = LazyLock::new(Vec::new);

fn empty_set() -> &'static HashSet<usize> {
    &EMPTY_SET
}

fn empty_vec() -> &'static Vec<usize> {
    &EMPTY_VEC
}

pub struct NumberVisualizerApp {
    pub config: VisualizerConfig,
    pub series_type: SeriesType,
    primes: Option<(Vec<usize>, HashSet<usize>)>,
    fibs: Option<(Vec<usize>, HashSet<usize>)>,
    lucas: Option<(Vec<usize>, HashSet<usize>)>,
    triangular: Option<(Vec<usize>, HashSet<usize>)>,
    collatz: Option<(Vec<usize>, HashSet<usize>)>,
    powers: Option<(Vec<usize>, HashSet<usize>)>,
    catalan: Option<(Vec<usize>, HashSet<usize>)>,
    hexagonal: Option<(Vec<usize>, HashSet<usize>)>,
    happy: Option<(Vec<usize>, HashSet<usize>)>,
    cached_max_number: usize,
    pub hovered_number: Option<usize>,
    pub helix_rotation_x: f32,
    pub helix_rotation_y: f32,
    pub error_message: Option<String>,
}

impl NumberVisualizerApp {
    pub fn new(config: VisualizerConfig) -> Self {
        Self {
            config,
            series_type: SeriesType::default(),
            primes: None,
            fibs: None,
            lucas: None,
            triangular: None,
            collatz: None,
            powers: None,
            catalan: None,
            hexagonal: None,
            happy: None,
            cached_max_number: 0,
            hovered_number: None,
            helix_rotation_x: ROTATION_X_DEFAULT,
            helix_rotation_y: 0.0,
            error_message: None,
        }
    }

    fn get_or_compute_series<F, R>(
        cache: &mut Option<(Vec<usize>, HashSet<usize>)>,
        max_number: usize,
        generator: F,
    ) -> &(Vec<usize>, HashSet<usize>)
    where
        F: FnOnce(usize) -> R,
        R: Into<Option<Vec<usize>>>,
    {
        if cache.is_none() {
            let vec: Vec<usize> = generator(max_number).into().unwrap_or_default();
            let set: HashSet<usize> = vec.iter().copied().collect();
            *cache = Some((vec, set));
        }
        cache.as_ref().expect("BUG: series cache was not populated")
    }

    fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn ensure_series_loaded(&mut self) {
        if self.config.max_number != self.cached_max_number {
            self.primes = None;
            self.fibs = None;
            self.lucas = None;
            self.triangular = None;
            self.collatz = None;
            self.powers = None;
            self.catalan = None;
            self.hexagonal = None;
            self.happy = None;
            self.cached_max_number = self.config.max_number;
        }

        self.clear_error();

        let max_number = self.config.max_number;
        match self.series_type {
            SeriesType::Primes => {
                let primes = generate_primes(max_number, false, None, None, None);
                match primes {
                    Ok(primes_vec) => {
                        let set: HashSet<usize> = primes_vec.iter().copied().collect();
                        self.primes = Some((primes_vec, set));
                    }
                    Err(err) => {
                        self.set_error(format!("Failed to generate primes: {}", err));
                        self.primes = Some((Vec::new(), HashSet::new()));
                    }
                }
            }
            SeriesType::Fibonacci => {
                Self::get_or_compute_series(&mut self.fibs, max_number, generate_fibonacci_up_to);
            }
            SeriesType::Lucas => {
                Self::get_or_compute_series(&mut self.lucas, max_number, generate_lucas_up_to);
            }
            SeriesType::Triangular => {
                Self::get_or_compute_series(
                    &mut self.triangular,
                    max_number,
                    generate_triangular_up_to,
                );
            }
            SeriesType::Collatz => {
                Self::get_or_compute_series(
                    &mut self.collatz,
                    max_number,
                    generate_collatz_times_up_to,
                );
            }
            SeriesType::PowersOf2 => {
                Self::get_or_compute_series(
                    &mut self.powers,
                    max_number,
                    generate_powers_of_2_up_to,
                );
            }
            SeriesType::Catalan => {
                Self::get_or_compute_series(&mut self.catalan, max_number, generate_catalan_up_to);
            }
            SeriesType::Hexagonal => {
                Self::get_or_compute_series(
                    &mut self.hexagonal,
                    max_number,
                    generate_hexagonal_up_to,
                );
            }
            SeriesType::Happy => {
                Self::get_or_compute_series(&mut self.happy, max_number, generate_happy_up_to);
            }
        }

        if self.config.visualization.is_primes_only() && self.series_type != SeriesType::Primes {
            self.config.visualization = VisualizationType::UlamSpiral;
        }
    }

    pub fn contains(&self, n: usize) -> bool {
        self.highlights().contains(&n)
    }

    pub fn highlights(&self) -> &HashSet<usize> {
        match self.series_type {
            SeriesType::Primes => self.primes.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
            SeriesType::Fibonacci => self.fibs.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
            SeriesType::Lucas => self.lucas.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
            SeriesType::Triangular => self
                .triangular
                .as_ref()
                .map(|(_, s)| s)
                .unwrap_or(empty_set()),
            SeriesType::Collatz => self.collatz.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
            SeriesType::PowersOf2 => self.powers.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
            SeriesType::Catalan => self.catalan.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
            SeriesType::Hexagonal => self
                .hexagonal
                .as_ref()
                .map(|(_, s)| s)
                .unwrap_or(empty_set()),
            SeriesType::Happy => self.happy.as_ref().map(|(_, s)| s).unwrap_or(empty_set()),
        }
    }

    pub fn primes_set(&self) -> &HashSet<usize> {
        self.primes.as_ref().map(|(_, s)| s).unwrap_or(empty_set())
    }

    pub fn primes_vec(&self) -> &Vec<usize> {
        self.primes.as_ref().map(|(v, _)| v).unwrap_or(empty_vec())
    }

    pub fn series_name(&self) -> &'static str {
        match self.series_type {
            SeriesType::Primes => "prime",
            SeriesType::Fibonacci => "fibonacci",
            SeriesType::Lucas => "lucas",
            SeriesType::Triangular => "triangular",
            SeriesType::Collatz => "collatz",
            SeriesType::PowersOf2 => "power of 2",
            SeriesType::Catalan => "catalan",
            SeriesType::Hexagonal => "hexagonal",
            SeriesType::Happy => "happy",
        }
    }

    pub fn draw_visualization(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, self.config.background_color);

        let mouse_pos = ui.input(|i| i.pointer.hover_pos());
        self.hovered_number = None;

        match self.config.visualization {
            VisualizationType::UlamSpiral => {
                let positions = viz::generate_ulam_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_ulam(self, mp, rect, &positions));
                viz::draw_ulam(self, ui, rect, &positions);
            }
            VisualizationType::SacksSpiral => {
                let positions = viz::generate_sacks_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_sacks(self, mp, rect, &positions));
                viz::draw_sacks(self, ui, rect, &positions);
            }
            VisualizationType::Grid => {
                let positions = viz::generate_grid_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_grid(self, mp, rect, &positions));
                viz::draw_grid(self, ui, rect, &positions);
            }
            VisualizationType::Row => {
                let positions = viz::generate_row_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_row(self, mp, rect, &positions));
                viz::draw_row(self, ui, rect, &positions);
            }
            VisualizationType::FermatsSpiral => {
                let positions = viz::generate_fermats_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_fermats(self, mp, rect, &positions));
                viz::draw_fermats(self, ui, rect, &positions);
            }
            VisualizationType::HexagonalLattice => {
                let positions = viz::generate_hexagonal_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_hexagonal(self, mp, rect, &positions));
                viz::draw_hexagonal(self, ui, rect, &positions);
            }
            VisualizationType::TriangularLattice => {
                let positions = viz::generate_triangular_positions(self.config.max_number);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_triangular(self, mp, rect, &positions));
                viz::draw_triangular(self, ui, rect, &positions);
            }
            VisualizationType::PrimeWheel => {
                let positions =
                    viz::generate_prime_wheel_positions(self.config.max_number, self.config.modulo);
                self.hovered_number = mouse_pos
                    .filter(|_| self.config.visualization.supports_hover())
                    .and_then(|mp| viz::find_hovered_prime_wheel(self, mp, rect, &positions));
                viz::draw_prime_wheel(self, ui, rect);
            }
            VisualizationType::PrimeDensity => viz::draw_prime_density(self, ui, rect),
            VisualizationType::RiemannZeta => viz::draw_riemann(self, ui, rect),
            VisualizationType::SacksMobiusSpiral => viz::draw_sacks_mobius(self, ui, rect),
            VisualizationType::UlamMobiusSpiral => viz::draw_ulam_mobius(self, ui, rect),
            VisualizationType::PrimeDensityGradient => viz::draw_density_gradient(self, ui, rect),
            VisualizationType::Helix3D => viz::draw_helix_3d(self, ui, rect),
            VisualizationType::Sphere3D => viz::draw_sphere_3d(self, ui, rect),
            VisualizationType::Torus3D => viz::draw_torus_3d(self, ui, rect),
            VisualizationType::Cone3D => viz::draw_cone_3d(self, ui, rect),
            VisualizationType::Cylinder3D => viz::draw_cylinder_3d(self, ui, rect),
            VisualizationType::Cube3D => viz::draw_cube_3d(self, ui, rect),
            VisualizationType::Mobius3D => viz::draw_mobius_3d(self, ui, rect),
            VisualizationType::Klein3D => viz::draw_klein_3d(self, ui, rect),
            VisualizationType::Pyramid3D => viz::draw_pyramid_3d(self, ui, rect),
            VisualizationType::Dodecahedron3D => viz::draw_dodecahedron_3d(self, ui, rect),
            VisualizationType::Icosahedron3D => viz::draw_icosahedron_3d(self, ui, rect),
            VisualizationType::Trefoil3D => viz::draw_trefoil_3d(self, ui, rect),
        }
    }
}

impl eframe::App for NumberVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls")
            .min_width(SIDE_PANEL_MIN_WIDTH)
            .show(ctx, |ui| {
                ui.heading("Number Visualizer");

                ui.separator();

                ui.label("Series:");
                egui::ComboBox::from_id_salt("series_type")
                    .selected_text(format!("{}", self.series_type))
                    .show_ui(ui, |ui| {
                        for series in SeriesType::ALL {
                            ui.selectable_value(
                                &mut self.series_type,
                                *series,
                                format!("{}", series),
                            );
                        }
                    });

                ui.separator();

                ui.label("Visualization:");
                egui::ComboBox::from_id_salt("viz_type")
                    .selected_text(format!("{}", self.config.visualization))
                    .show_ui(ui, |ui| {
                        for viz_type in VisualizationType::available_for(self.series_type) {
                            ui.selectable_value(
                                &mut self.config.visualization,
                                *viz_type,
                                format!("{}", viz_type),
                            );
                        }
                    });

                ui.separator();
                ui.label(self.config.visualization.description());

                ui.separator();

                ui.add_enabled_ui(true, |ui| {
                    ui.label("Max Number:");
                    ui.add(
                        egui::Slider::new(
                            &mut self.config.max_number,
                            MAX_NUMBER_MIN..=MAX_NUMBER_MAX,
                        )
                        .text("n"),
                    );
                });

                ui.separator();
                ui.label("Display");

                if self.config.visualization.uses_point_rendering() {
                    ui.label("Highlight size:");
                    ui.add(egui::Slider::new(&mut self.config.highlight_size, 1..=20));

                    ui.label("Non-highlight size:");
                    ui.add(egui::Slider::new(
                        &mut self.config.non_highlight_size,
                        0..=10,
                    ));

                    ui.checkbox(&mut self.config.show_numbers, "Show numbers");

                    if self.series_type == SeriesType::Primes {
                        ui.separator();
                        ui.label("Prime Pairs");

                        ui.checkbox(&mut self.config.show_twin_primes, "Twin primes");
                        if self.config.show_twin_primes {
                            ui.color_edit_button_srgba(&mut self.config.twin_color);
                        }

                        ui.checkbox(&mut self.config.show_cousin_primes, "Cousin primes");
                        if self.config.show_cousin_primes {
                            ui.color_edit_button_srgba(&mut self.config.cousin_color);
                        }

                        ui.checkbox(&mut self.config.show_sexy_primes, "Sexy primes");
                        if self.config.show_sexy_primes {
                            ui.color_edit_button_srgba(&mut self.config.sexy_color);
                        }
                    }
                }

                ui.separator();
                ui.label("Colors");

                ui.label("Highlight:");
                ui.color_edit_button_srgba(&mut self.config.highlight_color);

                ui.label("Non-highlight:");
                ui.color_edit_button_srgba(&mut self.config.non_highlight_color);

                ui.label("Background:");
                ui.color_edit_button_srgba(&mut self.config.background_color);

                if self.config.visualization.uses_modulo() {
                    ui.separator();
                    ui.label("Prime Wheel");
                    ui.add(
                        egui::Slider::new(&mut self.config.modulo, MODULO_MIN..=MODULO_MAX)
                            .text("Modulo"),
                    );
                }

                if self.config.visualization.uses_grid_size() {
                    ui.separator();
                    ui.label("Density Grid");
                    ui.add(
                        egui::Slider::new(
                            &mut self.config.grid_size,
                            GRID_SIZE_MIN..=GRID_SIZE_MAX,
                        )
                        .text("Grid size"),
                    );
                }

                if self.config.visualization.uses_num_zeros() {
                    ui.separator();
                    ui.label("Riemann Zeta");
                    ui.add(
                        egui::Slider::new(
                            &mut self.config.num_zeros,
                            NUM_ZEROS_MIN..=NUM_ZEROS_MAX,
                        )
                        .text("Zeros to show"),
                    );
                }

                ui.separator();
                if ui.button("Reset to Defaults").clicked() {
                    self.config = VisualizerConfig::default();
                    self.series_type = SeriesType::default();
                }
            });

        self.ensure_series_loaded();

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();

            if let Some(ref error) = self.error_message {
                let error_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(rect.left() + 5.0, rect.top() + 5.0),
                    egui::vec2(rect.width() - 10.0, 30.0),
                );
                ui.painter().rect_filled(
                    error_rect,
                    2.0,
                    egui::Color32::from_rgba_unmultiplied(80, 20, 20, 200),
                );
                ui.painter().text(
                    error_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    error.clone(),
                    egui::FontId::proportional(14.0),
                    egui::Color32::from_rgba_unmultiplied(255, 100, 100, 255),
                );
            }

            self.draw_visualization(ui, rect);

            if let Some(hovered) = self.hovered_number {
                let is_highlighted = self.contains(hovered);
                let text = if is_highlighted {
                    format!("{} ({})", hovered, self.series_name())
                } else {
                    format!("{}", hovered)
                };
                ui.painter().text(
                    egui::Pos2::new(rect.left() + 5.0, rect.bottom() - 20.0),
                    egui::Align2::LEFT_BOTTOM,
                    text,
                    egui::FontId::proportional(14.0),
                    if is_highlighted {
                        self.config.highlight_color
                    } else {
                        self.config.non_highlight_color
                    },
                );
            }
        });
    }
}
