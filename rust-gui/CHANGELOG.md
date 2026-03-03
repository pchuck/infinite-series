# Changelog

All notable changes to this project will be documented in this file.

## [1.0.3] - 2026-03-02

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
