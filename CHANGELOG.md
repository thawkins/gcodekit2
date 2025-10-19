# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha] - 2025-10-19

### Added
- Initial MVP with core GRBL communication
- Phase 2 MVP implementation complete
- System theme support (Light/Dark mode)
  - Automatic OS theme detection (Windows, macOS, Linux)
  - Color palettes with WCAG AA compliance
  - Dynamic theme switching without restart
  - Persistent theme preferences
- Comprehensive test organization (77 tests in tests/ hierarchy)
- Theme infrastructure with manager, detector, and palettes
- Debug and release builds

### Changed
- Program name officially set to `gcodekit2`
- Tests reorganized to `tests/` folder with module hierarchy
- AGENTS.md updated with test organization standards

### Fixed
- Project structure aligned with best practices
- All tests passing (77/77)

## [0.1.0-alpha]

### Initial Release
- Foundation for GRBL desktop application
- Basic machine control
- CAM functionality
- Job management system
