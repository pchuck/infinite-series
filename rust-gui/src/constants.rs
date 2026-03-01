//! Application-wide constants organized by category

pub mod limits {
    //! Range limits and bounds for configuration values

    /// Minimum value for max_number slider
    pub const MAX_NUMBER_MIN: usize = 100;
    /// Maximum value for max_number slider
    pub const MAX_NUMBER_MAX: usize = 100000;
    /// Default value for max_number
    pub const MAX_NUMBER_DEFAULT: usize = 10000;
    /// Maximum number of values to show text labels for
    pub const SHOW_NUMBERS_MAX: usize = 10000;
    /// Number of intervals for density calculations
    pub const DENSITY_INTERVALS: usize = 100;

    // Display defaults
    /// Default highlight size
    pub const HIGHLIGHT_SIZE_DEFAULT: usize = 2;
    /// Default non-highlight size
    pub const NON_HIGHLIGHT_SIZE_DEFAULT: usize = 1;
    /// Default modulo for prime wheel
    pub const MODULO_DEFAULT: usize = 30;
    /// Default number of zeros to show in Riemann visualization
    pub const NUM_ZEROS_DEFAULT: usize = 10;
    /// Default grid size for density gradient
    pub const GRID_SIZE_DEFAULT: usize = 40;
}

pub mod layout {
    //! UI layout and spacing constants

    /// Minimum width for the side panel
    pub const SIDE_PANEL_MIN_WIDTH: f32 = 250.0;
    /// Default margin used throughout the UI
    pub const UI_MARGIN: f32 = 5.0;
    /// Height of error message boxes
    pub const ERROR_BOX_HEIGHT: f32 = 30.0;
    /// Vertical offset for hover text display
    pub const HOVER_TEXT_OFFSET_Y: f32 = 20.0;
    /// Default font size for text rendering
    pub const FONT_SIZE_DEFAULT: f32 = 14.0;
}

pub mod visualization {
    //! Visualization rendering constants

    /// Small margin used for visualization bounds
    pub const MARGIN_SMALL: f32 = 20.0;
    /// Default hover threshold for point-based visualizations
    pub const HOVER_THRESHOLD_DEFAULT: f32 = 1.2;
    /// Large hover threshold for sparse visualizations
    pub const HOVER_THRESHOLD_LARGE: f32 = 1.5;
}

pub mod spiral {
    //! Spiral-specific mathematical constants

    /// Sacks spiral angle multiplier (r = sqrt(n), theta = n * multiplier)
    pub const SACKS_THETA_MULTIPLIER: f32 = 0.5;
    /// Sacks Mobius spiral radius multiplier
    pub const SACKS_MOBIUS_RADIUS_MULTIPLIER: f32 = 0.8;
    /// Golden angle in radians for Fermat's spiral (phyllotaxis pattern)
    pub const GOLDEN_ANGLE: f32 = 2.39996_f32;
}

pub mod stroke {
    //! Stroke width constants for line rendering

    /// Tiny stroke width in pixels
    pub const TINY: f32 = 0.5;
    /// Small stroke width in pixels
    pub const SMALL: f32 = 1.0;
    /// Medium stroke width in pixels
    pub const MEDIUM: f32 = 1.5;
    /// Large stroke width in pixels
    pub const LARGE: f32 = 2.0;
    /// Extra large stroke width in pixels
    pub const XLARGE: f32 = 2.5;
}

pub mod gap {
    //! Gap brightness constants for Mobius spiral visualizations

    /// Brightness for twin prime gaps (gap = 2)
    pub const BRIGHTNESS_TWIN: u8 = 255;
    /// Brightness for gap size 4
    pub const BRIGHTNESS_4: u8 = 220;
    /// Brightness for gap size 6
    pub const BRIGHTNESS_6: u8 = 180;
    /// Brightness for gap size 8
    pub const BRIGHTNESS_8: u8 = 150;
    /// Brightness for gap size 10
    pub const BRIGHTNESS_10: u8 = 120;
    /// Brightness for gap size 12
    pub const BRIGHTNESS_12: u8 = 100;
    /// Brightness for gap size 14
    pub const BRIGHTNESS_14: u8 = 85;
    /// Brightness for gap size 16
    pub const BRIGHTNESS_16: u8 = 70;
    /// Brightness for small gaps (17-20)
    pub const BRIGHTNESS_SMALL: u8 = 60;
    /// Brightness for medium gaps (21-30)
    pub const BRIGHTNESS_MEDIUM: u8 = 45;
    /// Brightness for large gaps (31-50)
    pub const BRIGHTNESS_LARGE: u8 = 30;
    /// Brightness for extra large gaps (51+)
    pub const BRIGHTNESS_XLARGE: u8 = 20;
}

pub mod projection {
    //! 3D projection constants

    /// Perspective distance for 3D projection
    pub const PERSPECTIVE: f32 = 500.0;
    /// Offset added to depth before projection
    pub const OFFSET: f32 = 300.0;
    /// Range of depth values for scaling
    pub const DEPTH_RANGE: f32 = 600.0;
    /// Minimum depth factor for brightness adjustment
    pub const MIN_DEPTH_FACTOR: f32 = 0.3;
    /// Maximum depth factor for brightness adjustment
    pub const MAX_DEPTH_FACTOR: f32 = 1.0;
    /// Mouse drag sensitivity for rotation
    pub const DRAG_SENSITIVITY: f32 = 0.01;
}

pub mod drawing {
    //! Number drawing constants

    /// Minimum radius for drawing circles
    pub const MIN_CIRCLE_RADIUS: f32 = 0.5;
    /// Minimum size before showing text labels
    pub const MIN_SIZE_FOR_TEXT: f32 = 6.0;
    /// Factor to multiply size by for text rendering
    pub const TEXT_SIZE_FACTOR: f32 = 0.6;
}

pub mod shapes {
    //! 3D shape dimension constants

    /// Default radius for 3D sphere
    pub const SPHERE_RADIUS: f32 = 100.0;
    /// Major radius for torus (donut)
    pub const TORUS_MAJOR_RADIUS: f32 = 80.0;
    /// Minor radius for torus (tube)
    pub const TORUS_MINOR_RADIUS: f32 = 30.0;
    /// Height of 3D cone
    pub const CONE_HEIGHT: f32 = 200.0;
    /// Base radius of 3D cone
    pub const CONE_BASE_RADIUS: f32 = 100.0;
    /// Number of spiral turns for cone
    pub const CONE_TURNS: f32 = 10.0;
    /// Height of 3D cylinder
    pub const CYLINDER_HEIGHT: f32 = 200.0;
    /// Radius of 3D cylinder
    pub const CYLINDER_RADIUS: f32 = 80.0;
    /// Number of spiral turns for cylinder
    pub const CYLINDER_TURNS: f32 = 8.0;
    /// Size of 3D cube
    pub const CUBE_SIZE: f32 = 80.0;
    /// Major radius for Möbius strip
    pub const MOBIUS_RADIUS: f32 = 80.0;
    /// Width of Möbius strip
    pub const MOBIUS_WIDTH: f32 = 30.0;
    /// Radius of Klein bottle
    pub const KLEIN_RADIUS: f32 = 60.0;
    /// Height of 3D pyramid
    pub const PYRAMID_HEIGHT: f32 = 150.0;
    /// Base size of 3D pyramid
    pub const PYRAMID_BASE: f32 = 120.0;
    /// Scale factor for dodecahedron
    pub const DODECAHEDRON_SCALE: f32 = 70.0;
    /// Scale factor for icosahedron
    pub const ICOSAHEDRON_SCALE: f32 = 80.0;
    /// Radius for trefoil knot
    pub const KNOT_RADIUS: f32 = 80.0;
    /// Tube radius for trefoil knot
    pub const KNOT_TUBE_RADIUS: f32 = 20.0;
}

pub mod helix {
    //! Helix 3D visualization constants

    /// Radius of the helix spiral
    pub const RADIUS: f32 = 100.0;
    /// Height factor for helix elongation
    pub const HEIGHT_FACTOR: f32 = 3.0;
    /// Number of turns in the helix
    pub const TURNS: f32 = 8.0;
}
