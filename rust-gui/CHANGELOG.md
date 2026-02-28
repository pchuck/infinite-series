# Changelog

All notable changes to this project will be documented in this file.

## [1.0.2] - Unreleased

### Added
- Detailed error messages for prime generation failures (includes specific error reason)

### Changed
- Improved error handling to display underlying `PrimeGenError` details

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

