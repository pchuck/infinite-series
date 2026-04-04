# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2026-04-03

### Summary
- Modularized library into separate modules (error, utils, classic, segmented, parallel, dispatcher)
- Fixed documentation inconsistency: parallel threshold is 5M (not 100M)
- Added test for n > MAX_N error path

## [1.0.1] - 2026-02-26

### Summary
- Classic Sieve of Eratosthenes implementation (odd-only)
- Segmented Sieve of Eratosthenes (odd-only)
- Parallel Segmented Sieve for large inputs (n >= 5M)
- CLI with progress bar support
- Benchmark suite using Criterion
- Integration tests for CLI
- Auto-selection of algorithm based on input size

## [1.0.0] - 2026-02-19

### Summary

- initial release
