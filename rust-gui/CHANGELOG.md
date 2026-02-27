# Changelog

All notable changes to this project will be documented in this file.

## [1.0.1] - 2026-02-26

### Added
- Prime pair coloring support for 3D visualizations
- Hover support for Prime Wheel visualization
- Error message display in UI when prime generation fails
- Position caching optimization: compute positions once per frame instead of twice

### Changed
- Unified cache helper methods (`get_or_compute_series_result` + `get_or_compute_series_vec` → `get_or_compute_series`)
- Replaced `OnceLock` with `LazyLock` for empty collection singletons
- Refactored visualization modules to use shared `compute_layout` helpers

### Fixed
- 3D visualizations now correctly respond to prime pair selections

### Refactored
- Extracted magic numbers to constants:
  - `config.rs`: `MAX_NUMBER_MIN`, `MAX_NUMBER_MAX`, `SHOW_NUMBERS_MAX`, `DENSITY_INTERVALS`, `SIDE_PANEL_MIN_WIDTH`
  - `prime_wheel.rs`: `MODULO_MIN`, `MODULO_MAX`
  - `density_gradient.rs`: `GRID_SIZE_MIN`, `GRID_SIZE_MAX`
  - `riemann.rs`: `NUM_ZEROS_MIN`, `NUM_ZEROS_MAX`
  - `helix_3d.rs`: `ROTATION_X_DEFAULT`
  - `draw_number.rs`: `MIN_CIRCLE_RADIUS`, `MIN_SIZE_FOR_TEXT`, `TEXT_SIZE_FACTOR`
  - `prime_density.rs`: `MIN_MAX_N`
- Removed unused `_ctx` parameter from `NumberVisualizerApp::new()`

## [1.0.0] - 2026-02-19

### Summary

- initial release

