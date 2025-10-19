# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha] - 2025-10-19

### Added (Session: Phase 14 Completion & Phase 2 Planning)
- **Phase 14: System Theme Support - COMPLETED** ✅
  - Comprehensive theme-provider.slint component with reactive colors
  - Settings panel with theme selection (Light/Dark/Auto System)
  - All 10 color properties in ThemeColors struct
  - 31 comprehensive theme system tests (100% passing)
  - WCAG AA contrast compliance verified (4.5:1+ minimum)
  - Theme persistence across sessions
  - Real-time switching without restart
- Comprehensive implementation plan with phase breakdown (`docs/IMPLEMENTATION_PLAN.md`)
- Phase 14 completion summary (`docs/PHASE_14_COMPLETION.md`)
- Both debug (224MB) and release (13MB) binaries verified

### Changed
- Program name confirmed as `gcodekit2` in Cargo.toml
- SPEC.md: Added UI theme appearance requirement (#16)
- All UI components now using themed colors from ThemeProvider
- Build system verified (zero critical errors, 108/108 tests passing)

### Added
- Initial MVP with core GRBL communication
- Phase 2 MVP implementation complete with Phase 14 system theme support
- System theme support (Light/Dark mode)
  - Automatic OS theme detection (Windows, macOS, Linux)
  - Color palettes with WCAG AA compliance (4.5:1 minimum contrast)
  - Dynamic theme switching without restart
  - Persistent theme preferences
  - Full UI component theming
- Comprehensive theme test suite (31 new tests)
  - Theme manager and switching tests
  - Color palette contrast validation
  - WCAG AA compliance verification
  - Preference persistence tests
  - Theme storage and retrieval tests
- Comprehensive test organization (108 tests in tests/ hierarchy)
  - Communication module tests
  - Designer/CAM tests
  - Jobs/scheduling tests
  - Materials database tests
  - Widgets tests
  - Theme system tests
- Theme infrastructure with manager, detector, palettes, and storage
- Debug and release builds (both successful)

### Changed
- Program name officially set to `gcodekit2`
- Tests reorganized to `tests/` folder with complete module hierarchy
- AGENTS.md updated with test organization and CHANGELOG requirements
- SPEC.md updated with system theme support as requirement #15
- Added Theme module to tests/lib.rs with full integration test coverage

### Fixed
- Project structure aligned with best practices
- All tests passing (108/108 - 100% pass rate)

## [0.1.0-alpha]

### Initial Release
- Foundation for GRBL desktop application
- Basic machine control
- CAM functionality
- Job management system
