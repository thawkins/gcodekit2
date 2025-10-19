# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha] - 2025-10-19 (Phase 2 MVP - Complete Theme Integration & Web Pendant)

### Added
- **Phase 2 MVP Foundation**: Core GRBL communication, CAM functions, multi-axis support, and theme infrastructure
- **System Theme Support (Light/Dark Mode)**:
  - Automatic OS theme detection (Windows, macOS, Linux)
  - Color palettes with WCAG AA compliance (4.5:1 minimum contrast ratio)
  - Dynamic theme switching without application restart
  - Persistent theme preferences stored in platform-specific config directories
  - Full UI component theming (buttons, panels, text fields, status indicators)
  - Two professional color schemes (Light and Dark themes)
- **Task 10: Web Pendant Interface** (NEW):
  - RESTful API endpoints for machine control (/api/status, /api/jog, /api/override, /api/emergency-stop)
  - WebSocket real-time communication with bidirectional streaming
  - Mobile-responsive HTML5 interface with professional styling
  - Cross-browser compatibility (Chrome, Firefox, Safari, Edge)
  - Responsive design for desktop (1200px+) and mobile (320px+)
  - Light/dark theme support in pendant UI
  - Connection management with concurrent WebSocket support (up to 10 connections)
  - HTTP/HTTPS server with configurable ports and TLS support
  - 20 integration tests for pendant server, API, WebSocket, and UI components
  - Static assets: ui_styles.css (responsive styling), ui_client.js (real-time browser client)
- **Comprehensive Test Organization**:
  - 128 passing tests in `tests/` folder mirroring `src/` structure
  - Communication, Designer, Jobs, Materials, Widgets, Theme, and Pendant test modules
  - 31 theme-specific tests + 20 pendant-specific tests
  - 100% pass rate across all test suites
- **Build Infrastructure**:
  - Debug build: Successful with symbols (~224MB)
  - Release build: Successful with optimizations (~13MB)
  - Both builds verified and ready for deployment
- **Documentation Organization**:
  - All markdown documentation in `docs/` folder (15+ files including TASK_10_WEB_PENDANT.md)
  - SPEC.md, AGENTS.md, README.md, CHANGELOG.md remain in project root
  - Complete implementation phase documentation
  - CHANGELOG management established for tracking changes

### Changed
- Program name officially set to `gcodekit2` (binary and crate naming)
- All tests moved to `tests/` folder with full module hierarchy
- AGENTS.md: Updated with test organization requirements and CHANGELOG management procedures
- SPEC.md: Added theme support requirements (#15-16) with detailed system detection and WCAG compliance specs
- README.md: Updated with v0.2.0-alpha release information and build status
- Test framework: Organized with lib.rs integration root and module-based structure
- Documentation structure: Following best practices with docs/ folder for implementation guides
- Added pendant module to public API exports in lib.rs

### Fixed
- Project structure aligned with Rust best practices
- All tests passing (128/128 - 100% pass rate)
- Build warnings analyzed and documented
- Platform-specific serial port handling for device detection

## [0.1.0-alpha]

### Initial Release
- Foundation for GRBL desktop application
- Basic machine control
- CAM functionality
- Job management system
