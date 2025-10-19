# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha] - 2025-10-19 (Extended Session Update)

### Added (Session: Extended Phase 2 Implementation)
- **Documentation & Configuration Requirements**:
  - AGENTS.md updated with test organization requirements (tests/ hierarchy)
  - AGENTS.md updated with CHANGELOG management requirement (update before each push)
  - All markdown documentation files now organized in docs/ (except SPEC.md, AGENTS.md, README.md, CHANGELOG.md)
  - CHANGELOG.md must be maintained and updated before each remote push
- **Test Organization Verification**: All 108 tests passing in tests/ hierarchy mirroring src/ structure
- **Build Status**: Both debug and release builds successful and verified
- **Theme Support**: System theme support fully implemented (Light/Dark modes, auto-detection, WCAG AA compliance)
- **MVP Foundation**: Core GRBL communication, CAM functions, multi-axis support, and theme infrastructure complete

### Changed
- AGENTS.md: Enhanced documentation with test organization and CHANGELOG management requirements
- SPEC.md: Clarified theme support requirements (#15-16) with system detection and WCAG AA compliance details
- Build system verified with both debug and release optimized binaries
- All tests passing (108/108 - 100% pass rate)

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
