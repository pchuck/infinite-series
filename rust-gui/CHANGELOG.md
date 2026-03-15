# Changelog

All notable changes to this project will be documented in this file.

## [1.1.0] - 2026-03-10

### Added
- Two new 3D cube visualizations: "3D Cube Simple" and "3D Cube Quadratic"
- Spike distance slider (0-10) in control panel for all 3D visualizations

### Changed
- Fixed 3D cube rendering: points now distributed on cube faces instead of floating in space
- Fixed perspective/depth: closer points now appear larger (was reversed)
- Rotation now persists when changing max number

## [1.0.6] - 2026-03-09

### Added
- Trait-based visualization plugin system with `Visualizer` trait
- `VisualizationRegistry` for runtime lookup of visualizations
- `VizParams` struct for flexible visualization configuration
- 4 new registry tests for validation

### Changed
- Refactored all 25 visualization modules to implement `Visualizer` trait directly
- Simplified `app.rs`: replaced 30+ case match with registry lookup (~30 lines → ~10 lines)
- Registry reduced from ~1100 lines to ~100 lines (imports + register calls)

### Fixed
- 3D visualization crash: app no longer tries to retrieve cached positions for visualizations that don't use 2D positions

### Removed
- Unused methods from `VisualizationType`: `uses_modulo()`, `uses_num_zeros()`, `uses_grid_size()`, `supports_hover()`
- Unused `std::cell::RefCell` import from app.rs

### Documentation
- Added "Adding New Visualizations" section to README with step-by-step guide

## [1.0.5] - 2026-03-06

### Added
- 55 new unit tests (36 → 91 total) covering `compute_layout`, `find_hovered`, and 3D utilities across all visualization modules
- `draw_3d_scene()` shared helper in `shared_3d.rs` that handles drag, projection, depth sort, scale fitting, and rendering for all 3D visualizations
- `ensure_positions_cached()` and `cached_positions()` methods replacing the cloning `get_or_compute_positions()`

### Changed
- Position cache no longer clones the entire `Vec` on every frame; callers now get a zero-copy `&[...]` reference
- Hover detection uses a fixed screen-pixel threshold (2px default, 4px for sparse layouts) instead of a scale-proportional threshold that shrank to sub-pixel at large N
- `prime_wheel::draw()` now accepts and uses cached positions like all other 2D visualization modules
- `HOVER_THRESHOLD_DEFAULT` changed from 1.2 (logical units) to 2.0 (screen pixels)
- `HOVER_THRESHOLD_LARGE` changed from 1.5 (logical units) to 4.0 (screen pixels)

### Fixed
- Hover detection in Row and Prime Wheel visualizations now works at large N values (previously the scale-proportional threshold shrank below 1 pixel, making hover impossible)
- `console_error_panic_hook` removed from native dependencies (was compiled but never used outside WASM)
- `PrimePairColors::get_color()` now uses `debug_assert!` for unreachable empty-slice and >3-element cases instead of silently falling back

### Removed
- Dead code: `invalidate_position_cache()`, `invalidate_positions()`, `get_positions()` methods
- All `#[allow(dead_code)]` annotations (code was either actively used or truly dead and removed)

### Refactored
- Reduced 3D visualization code by 42% (1,649 → 961 lines): extracted ~50 lines of identical boilerplate from each of 12 modules into `draw_3d_scene()`
- Each 3D module now provides only a geometry closure to `draw_3d_scene()`, eliminating duplicated drag handling, depth sorting, scale calculation, and render loops

## [1.0.4] - 2026-03-04

### Added
- Web/WASM support: application can now run in a web browser
- `index.html` - HTML entry point for WASM builds
- `Trunk.toml` - Trunk build configuration
- Makefile targets: `web-build`, `web-serve`
- Platform-specific `MAX_N` constant: full range for native, limited to 2^32-1 for WASM

### Changed
- Restructured `main.rs` with `#[cfg(target_arch = "wasm32")]` conditionals for dual native/web builds
- Added `wasm-bindgen` and `wasm-bindgen-futures` dependencies for WASM
- Added `console_error_panic_hook` for better error reporting in browser console

### Build Requirements
- Native desktop: Standard Rust build (see README)
- Web/WASM: Requires `trunk` (`cargo install trunk`) and `wasm32-unknown-unknown` target

### Added
- Prime pair color blending: primes can now belong to multiple pair types (twin, cousin, sexy) simultaneously
- Pre-computed blended colors for all 7 combinations (3 single + 3 two-way + 1 three-way)
- Color picker UI integration with automatic blend recomputation when colors change
- Added `PrimePairType` enum for tracking which pair types a prime belongs to
- Added `PrimePairColors` struct with methods for color retrieval and recomputation

### Changed
- Removed exclusivity between prime pair types: primes can now be highlighted with blended colors representing all applicable pair types
- `get_prime_pair_color()` now returns blended colors based on all matching pair types instead of priority-based selection
- Prime pair detection logic changed from exclusive (twin > cousin > sexy) to inclusive (all matching types)

### Fixed
- Prime 7 and other primes that belong to multiple pair types now correctly display blended colors
- Removed priority ordering where twin primes would mask cousin/sexy status

### Refactored
- Extracted color blending logic to `PrimePairColors` struct in config module
- Simplified `draw_number()` by removing inline pair type detection logic
- Added 7 unit tests for color blending functions and prime pair color retrieval

## [1.0.2] - 2026-03-01

### Added
- Detailed error messages for prime generation failures (includes specific error reason)
- 26 unit tests covering helpers, position generators, and types

### Changed
- Improved error handling to display underlying `PrimeGenError` details

### Refactored
- Extracted hover detection logic to reusable helpers (`find_hovered_center_based`, `find_hovered_offset_based`, `find_hovered_row`, `find_hovered_center_flip_y`, `find_hovered_centered`)
- Extracted UI layout constants (`UI_MARGIN`, `ERROR_BOX_HEIGHT`, `HOVER_TEXT_OFFSET_Y`, `FONT_SIZE_DEFAULT`)
- Added `Default` derive to `PerVisualizationConfig`
- Clarified `find_hovered_offset_based` documentation
- Updated Makefile with `test`, `test-release`, `lint`, and `fmt` targets

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
- Extracted magic numbers to constants
- Removed unused `_ctx` parameter from `NumberVisualizerApp::new()`

## [1.0.0] - 2026-02-19

### Summary

- initial release
