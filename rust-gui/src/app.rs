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

use crate::config::{
    PerVisualizationConfig, VisualizerConfig, ERROR_BOX_HEIGHT, FONT_SIZE_DEFAULT,
    HOVER_TEXT_OFFSET_Y, MAX_NUMBER_MAX, MAX_NUMBER_MIN, SIDE_PANEL_MIN_WIDTH, UI_MARGIN,
};
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::VizParams;
use crate::visualizations::REGISTRY;

static EMPTY_SET: LazyLock<HashSet<usize>> = LazyLock::new(HashSet::new);
static EMPTY_VEC: LazyLock<Vec<usize>> = LazyLock::new(Vec::new);

static ERROR_BG_COLOR: LazyLock<egui::Color32> =
    LazyLock::new(|| egui::Color32::from_rgba_unmultiplied(80, 20, 20, 200));
static ERROR_TEXT_COLOR: LazyLock<egui::Color32> =
    LazyLock::new(|| egui::Color32::from_rgba_unmultiplied(255, 100, 100, 255));

fn empty_set() -> &'static HashSet<usize> {
    &EMPTY_SET
}

fn empty_vec() -> &'static Vec<usize> {
    &EMPTY_VEC
}

/// Main application state for the Number Sequence Visualizer.
pub struct NumberVisualizerApp {
    pub config: VisualizerConfig,
    pub series_type: SeriesType,
    per_viz_config: PerVisualizationConfig,
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
    pub error_message: Option<String>,
}

impl NumberVisualizerApp {
    pub fn new(config: VisualizerConfig) -> Self {
        Self {
            config,
            series_type: SeriesType::default(),
            per_viz_config: PerVisualizationConfig::default(),
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
            error_message: None,
        }
    }

    pub fn get_rotation(&self) -> (f32, f32) {
        let settings = self.per_viz_config.get(self.config.visualization);
        (settings.rotation_x, settings.rotation_y)
    }

    pub fn set_rotation(&mut self, rotation_x: f32, rotation_y: f32) {
        let mut settings = self.per_viz_config.get(self.config.visualization);
        settings.rotation_x = rotation_x;
        settings.rotation_y = rotation_y;
        self.per_viz_config.set(self.config.visualization, settings);
    }

    pub fn invalidate_rotation_cache(&mut self) {
        self.per_viz_config.set(
            self.config.visualization,
            crate::config::VisualizationSettings::default(),
        );
    }

    /// Ensure positions are computed and cached for the given visualization type.
    ///
    /// After calling this, retrieve the cached positions via `cached_positions()`.
    pub fn ensure_positions_cached(
        &mut self,
        viz_type: VisualizationType,
        max_number: usize,
        modulo: usize,
        generate_fn: impl FnOnce(usize, usize) -> Vec<(usize, f32, f32)>,
    ) {
        let needs_compute = !matches!(
            self.per_viz_config.position_cache.get(&viz_type),
            Some(cached) if cached.max_number == max_number && cached.modulo == modulo
        );

        if needs_compute {
            let positions = generate_fn(max_number, modulo);
            self.per_viz_config
                .set_positions(viz_type, positions, max_number, modulo);
        }
    }

    /// Get a reference to the cached positions for the given visualization type.
    ///
    /// Panics if positions have not been cached for this type.
    /// Always call `ensure_positions_cached()` first.
    pub fn cached_positions(&self, viz_type: VisualizationType) -> &[(usize, f32, f32)] {
        &self.per_viz_config.position_cache[&viz_type].positions
    }

    pub fn recompute_prime_pair_colors(&mut self) {
        self.config.prime_pair_colors.recompute(
            self.config.twin_color,
            self.config.cousin_color,
            self.config.sexy_color,
        );
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

    fn series_is_loaded(&self) -> bool {
        match self.series_type {
            SeriesType::Primes => self.primes.is_some(),
            SeriesType::Fibonacci => self.fibs.is_some(),
            SeriesType::Lucas => self.lucas.is_some(),
            SeriesType::Triangular => self.triangular.is_some(),
            SeriesType::Collatz => self.collatz.is_some(),
            SeriesType::PowersOf2 => self.powers.is_some(),
            SeriesType::Catalan => self.catalan.is_some(),
            SeriesType::Hexagonal => self.hexagonal.is_some(),
            SeriesType::Happy => self.happy.is_some(),
        }
    }

    pub fn ensure_series_loaded(&mut self) {
        let needs_load =
            self.config.max_number != self.cached_max_number || !self.series_is_loaded();

        if !needs_load {
            return;
        }

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
            self.per_viz_config.invalidate_all_positions();
            self.invalidate_rotation_cache();
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

        let viz_type = self.config.visualization;
        let max_n = self.config.max_number;
        let modulo = self.config.modulo;

        let (needs_positions, supports_hover, generate_fn) = {
            let viz = match REGISTRY.get(viz_type) {
                Some(v) => v,
                None => return,
            };

            let params = VizParams::default()
                .with_modulo(modulo)
                .with_grid_size(self.config.grid_size)
                .with_num_zeros(self.config.num_zeros);

            let needs = !viz.generate_positions(max_n, &params).is_empty();
            let hover = viz.supports_hover();
            (
                needs,
                hover,
                Box::new(move |max_n: usize, mod_val: usize| {
                    let p = VizParams::default().with_modulo(mod_val);
                    REGISTRY
                        .get(viz_type)
                        .unwrap()
                        .generate_positions(max_n, &p)
                }) as Box<dyn FnOnce(usize, usize) -> Vec<(usize, f32, f32)>>,
            )
        };

        if needs_positions {
            self.ensure_positions_cached(viz_type, max_n, modulo, generate_fn);
            let positions = self.cached_positions(viz_type);

            let hovered = mouse_pos.filter(|_| supports_hover).and_then(|mp| {
                REGISTRY
                    .get(viz_type)
                    .unwrap()
                    .find_hovered(self, mp, rect, positions)
            });
            self.hovered_number = hovered;
        }

        let positions = if needs_positions {
            self.cached_positions(viz_type).to_vec()
        } else {
            Vec::new()
        };
        REGISTRY
            .get(viz_type)
            .unwrap()
            .draw(self, ui, rect, &positions);
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
                            self.recompute_prime_pair_colors();
                        }

                        ui.checkbox(&mut self.config.show_cousin_primes, "Cousin primes");
                        if self.config.show_cousin_primes {
                            ui.color_edit_button_srgba(&mut self.config.cousin_color);
                            self.recompute_prime_pair_colors();
                        }

                        ui.checkbox(&mut self.config.show_sexy_primes, "Sexy primes");
                        if self.config.show_sexy_primes {
                            ui.color_edit_button_srgba(&mut self.config.sexy_color);
                            self.recompute_prime_pair_colors();
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

                if let Some(viz) = REGISTRY.get(self.config.visualization) {
                    ui.separator();
                    viz.config_ui(ui, &mut self.config, self.series_type);
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
                    egui::Pos2::new(rect.left() + UI_MARGIN, rect.top() + UI_MARGIN),
                    egui::vec2(rect.width() - 2.0 * UI_MARGIN, ERROR_BOX_HEIGHT),
                );
                ui.painter().rect_filled(error_rect, 2.0, *ERROR_BG_COLOR);
                ui.painter().text(
                    error_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    error.clone(),
                    egui::FontId::proportional(FONT_SIZE_DEFAULT),
                    *ERROR_TEXT_COLOR,
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
                    egui::Pos2::new(rect.left() + UI_MARGIN, rect.bottom() - HOVER_TEXT_OFFSET_Y),
                    egui::Align2::LEFT_BOTTOM,
                    text,
                    egui::FontId::proportional(FONT_SIZE_DEFAULT),
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
