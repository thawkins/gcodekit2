# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha] - 2025-10-19

### Added (Session: MVP Documentation & Phase 2 Planning)
- Comprehensive implementation plan with phase breakdown (`docs/IMPLEMENTATION_PLAN.md`)
- Phase structure documentation for all 16+ planned phases
- Clear deliverables and success criteria for each phase
- Dependency tracking and risk mitigation strategies
- Program name confirmed as `gcodekit2` in Cargo.toml
- Both debug (224MB) and release (13MB) binaries verified and working
- UI theme appearance requirement added to SPEC.md (requirement #16)

### Changed
- SPEC.md updated with current phase status (Phase 15: MVP Implementation & Phase 2 Release)
- AGENTS.md enhanced with explicit CHANGELOG management requirements
- AGENTS.md clarified documentation file location standards (docs/ folder except 4 root files)
- CHANGELOG.md designated as required before each push to remote
- Test organization verified complete in `tests/` folder with hierarchy matching `src/`

### Documentation
- Created `docs/IMPLEMENTATION_PLAN.md` with comprehensive phase breakdown
- All markdown documentation properly organized in docs/ folder
- SPEC.md, AGENTS.md, README.md, CHANGELOG.md remain in project root as required
- Implementation plan references all completed phases (1-14) and current Phase 15 work

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
