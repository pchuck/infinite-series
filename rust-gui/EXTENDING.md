## Adding New Visualizations

The codebase uses a trait-based plugin system. To add a new visualization:

### 1. Create the module file

Create `src/visualizations/my_viz.rs`:

```rust
//! My custom visualization

use crate::app::NumberVisualizerApp;
use crate::draw_number::draw_number;
use crate::helpers::MARGIN_SMALL;
use crate::types::{SeriesType, VisualizationType};
use crate::visualizations::params::VizParams;
use crate::visualizations::traits::Visualizer;
use eframe::egui;

/// Generate (number, x, y) positions
pub fn generate_positions(max_n: usize) -> Vec<(usize, f32, f32)> {
    // Your algorithm here
    (1..=max_n).map(|n| (n, n as f32, 0.0)).collect()
}

/// Draw the visualization
pub fn draw(
    app: &NumberVisualizerApp,
    ui: &mut egui::Ui,
    rect: egui::Rect,
    positions: &[(usize, f32, f32)],
) {
    // Your rendering code here
    // Access app.primes_set() or app.highlights() for highlighted numbers
    // Access app.config for colors and sizes
}

pub struct MyViz;

impl Visualizer for MyViz {
    fn viz_type(&self) -> VisualizationType { VisualizationType::MyViz }
    fn name(&self) -> &'static str { "My Visualization" }
    fn description(&self) -> &'static str { "Description here" }
    fn supports_series(&self, _series: SeriesType) -> bool { true } // or false for primes-only
    fn supports_hover(&self) -> bool { true }  // if you implement find_hovered
    fn uses_point_rendering(&self) -> bool { true }  // false for graph/heatmap types

    fn generate_positions(&self, max_n: usize, _params: &VizParams) -> Vec<(usize, f32, f32)> {
        generate_positions(max_n)
    }

    fn draw(&self, app: &mut NumberVisualizerApp, ui: &mut egui::Ui, rect: egui::Rect, positions: &[(usize, f32, f32)]) {
        draw(app, ui, rect, positions);
    }

    fn find_hovered(&self, app: &NumberVisualizerApp, mouse_pos: egui::Pos2, rect: egui::Rect, positions: &[(usize, f32, f32)]) -> Option<usize> {
        // Use helper functions from helpers.rs
        None
    }

    // Optional: add config sliders
    // fn config_ui(&self, ui: &mut egui::Ui, config: &mut VisualizerConfig, _series: SeriesType) {
    //     ui.label("My Viz Settings");
    //     ui.add(egui::Slider::new(&mut config.some_field, 1..=100));
    // }
}
```

### 2. Add the VisualizationType enum variant

In `src/types.rs`, add to `VisualizationType`:

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
pub enum VisualizationType {
    // ... existing
    MyViz,
}
```

Add the description in `impl VisualizationType`:

```rust
pub fn description(self) -> &'static str {
    match self {
        // ... existing
        Self::MyViz => "My description",
    }
}
```

### 3. Register in mod.rs and registry.rs

In `src/visualizations/mod.rs`:

```rust
pub mod my_viz;
// ...
pub use my_viz::{draw as draw_my_viz, MyViz};
```

In `src/visualizations/registry.rs`:

```rust
use crate::visualizations::MyViz;
// ...
registry.register(MyViz);
```

That's it! The app will automatically pick it up.

### Key Points

- Implement `Visualizer` trait for 2D, `Visualizer3D` for 3D
- Use helpers from `helpers.rs` for layout/hover detection
- Access highlighted numbers via `app.highlights()` (HashSet) or `app.primes_set()` (HashSet)
- Use `app.config` for colors, sizes, and custom parameters
- Return empty `Vec` from `generate_positions()` for non-point visualizations (graphs, heatmaps)

