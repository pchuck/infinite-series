//! Main application and UI

use eframe::egui;
use primes::generate_primes;
use series::{
    generate_catalan_up_to, generate_collatz_times_up_to, generate_fibonacci_up_to,
    generate_happy_up_to, generate_hexagonal_up_to, generate_lucas_up_to,
    generate_powers_of_2_up_to, generate_triangular_up_to,
};
use std::collections::HashSet;
use std::sync::OnceLock;

use crate::config::VisualizerConfig;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations as viz;

static EMPTY_SET: OnceLock<HashSet<usize>> = OnceLock::new();
static EMPTY_VEC: OnceLock<Vec<usize>> = OnceLock::new();

fn empty_set() -> &'static HashSet<usize> {
    EMPTY_SET.get_or_init(HashSet::new)
}

fn empty_vec() -> &'static Vec<usize> {
    EMPTY_VEC.get_or_init(Vec::new)
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
}

impl NumberVisualizerApp {
    pub fn new(config: VisualizerConfig, _ctx: &egui::Context) -> Self {
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
            helix_rotation_x: 0.4,
            helix_rotation_y: 0.0,
        }
    }

    fn get_or_compute_series<F>(
        cache: &mut Option<(Vec<usize>, HashSet<usize>)>,
        max_number: usize,
        generator: F,
    ) -> &(Vec<usize>, HashSet<usize>)
    where
        F: FnOnce(usize) -> Vec<usize>,
    {
        if cache.is_none() {
            let vec = generator(max_number);
            let set: HashSet<usize> = vec.iter().copied().collect();
            *cache = Some((vec, set));
        }
        cache.as_ref().unwrap()
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

        let max_number = self.config.max_number;
        match self.series_type {
            SeriesType::Primes => {
                Self::get_or_compute_series(&mut self.primes, max_number, |n| {
                    generate_primes(n, false, None, None, None)
                });
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

        if let Some(mouse_pos) = mouse_pos {
            if self.config.visualization.supports_hover() {
                self.hovered_number = Self::get_hovered(self, mouse_pos, rect);
            }
        }

        match self.config.visualization {
            VisualizationType::UlamSpiral => viz::draw_ulam(self, ui, rect),
            VisualizationType::SacksSpiral => viz::draw_sacks(self, ui, rect),
            VisualizationType::Grid => viz::draw_grid(self, ui, rect),
            VisualizationType::Row => viz::draw_row(self, ui, rect),
            VisualizationType::PrimeWheel => viz::draw_prime_wheel(self, ui, rect),
            VisualizationType::PrimeDensity => viz::draw_prime_density(self, ui, rect),
            VisualizationType::RiemannZeta => viz::draw_riemann(self, ui, rect),
            VisualizationType::HexagonalLattice => viz::draw_hexagonal(self, ui, rect),
            VisualizationType::TriangularLattice => viz::draw_triangular(self, ui, rect),
            VisualizationType::FermatsSpiral => viz::draw_fermats(self, ui, rect),
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

    fn get_hovered(
        app: &NumberVisualizerApp,
        mouse_pos: egui::Pos2,
        rect: egui::Rect,
    ) -> Option<usize> {
        match app.config.visualization {
            VisualizationType::UlamSpiral => viz::find_hovered_ulam(app, mouse_pos, rect),
            VisualizationType::SacksSpiral => viz::find_hovered_sacks(app, mouse_pos, rect),
            VisualizationType::Grid => viz::find_hovered_grid(app, mouse_pos, rect),
            VisualizationType::Row => viz::find_hovered_row(app, mouse_pos, rect),
            VisualizationType::FermatsSpiral => viz::find_hovered_fermats(app, mouse_pos, rect),
            VisualizationType::HexagonalLattice => {
                viz::find_hovered_hexagonal(app, mouse_pos, rect)
            }
            VisualizationType::TriangularLattice => {
                viz::find_hovered_triangular(app, mouse_pos, rect)
            }
            _ => None,
        }
    }
}

impl eframe::App for NumberVisualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls")
            .min_width(250.0)
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
                    ui.add(egui::Slider::new(&mut self.config.max_number, 100..=100000).text("n"));
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
                    ui.add(egui::Slider::new(&mut self.config.modulo, 2..=60).text("Modulo"));
                }

                if self.config.visualization.uses_grid_size() {
                    ui.separator();
                    ui.label("Density Grid");
                    ui.add(
                        egui::Slider::new(&mut self.config.grid_size, 10..=100).text("Grid size"),
                    );
                }

                if self.config.visualization.uses_num_zeros() {
                    ui.separator();
                    ui.label("Riemann Zeta");
                    ui.add(
                        egui::Slider::new(&mut self.config.num_zeros, 1..=20).text("Zeros to show"),
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
