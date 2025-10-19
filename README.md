# GCodeKit - GRBL Desktop Control Application

A comprehensive desktop application for controlling GRBL-compatible laser engravers and CNC machines. Built with Rust and Slint, providing professional-grade CAM functions, job management, and machine control capabilities.

## Features

### Core Machine Control
- **GRBL Communication**: Full support for GRBL v1.1+ protocol with real-time status monitoring
- **3-Axis Control**: X/Y/Z jogging with configurable step sizes (0.1mm, 1mm, 10mm, 50mm)
- **Real-time Overrides**: Feed rate and spindle/laser power adjustments during operation
- **Emergency Stop**: Immediate machine halt with alarm recovery support
- **Device Console**: Real-time serial communication monitoring

### CAM & Design
- **Shape Generation**: Create rectangles, circles, polygons, and lines
- **G-code Generation**: Automatic conversion of shapes to GRBL-compatible G-code
- **Toolpath Optimization**: Comment removal, whitespace cleanup, decimal truncation
- **Arc Conversion**: G2/G3 arc to line segment conversion for compatibility
- **Design Management**: Multi-design support with full UNDO/REDO capabilities

### Job Management
- **Priority Scheduling**: Queue jobs with priority levels (1-10)
- **Progress Tracking**: Real-time progress monitoring per line
- **Pause/Resume**: Full control over job execution
- **Automatic Error Recovery**: Resume from last completed line after communication loss
- **Batch Processing**: Support for multi-file queuing and sequential execution

### Materials Database
- **Predefined Materials**: 7+ pre-configured material profiles (wood, acrylic, metal, leather, fabric, paper, rubber, stone, glass)
- **Custom Materials**: Add and manage your own material profiles
- **Cutting Parameters**: Material-specific feed rates, spindle speeds, and laser power settings
- **Material Groups**: Organize materials by type for quick access

### User Interface
- **Professional Layout**: Three-panel design with menu bar and status bar
- **Tabbed Interface**: G-code Editor, 3D Visualizer, Device Console, Job Manager, Designer tabs
- **Cross-platform**: Runs on Linux, Windows, and macOS
- **Responsive Design**: Real-time updates and low-latency communication

## Architecture

### Modular Design
- **communication**: GRBL protocol, serial communication, version detection, recovery
- **designer**: Shape generation, toolpath creation, G-code optimization
- **jobs**: Priority queuing, job scheduling, progress tracking
- **materials**: Material database, cutting parameters
- **widgets**: Connection, jog controls, overrides, G-code loading

### Dependencies
- **slint 1.13.1**: Modern Rust GUI framework
- **tokio 1.0**: Async runtime for non-blocking operations
- **serialport 4.2**: Serial communication
- **uuid 1.0**: Job identification and design tracking
- **serde 1.0**: JSON serialization for designs and configs
- **tracing 0.1**: Structured logging

## Getting Started

### Build
```bash
cargo build                 # Debug build
cargo build --release      # Optimized release build
```

### Run
```bash
cargo run
```

### Test
```bash
cargo test                 # Run all tests
cargo test --lib         # Run only library tests
cargo test --doc         # Run documentation tests
```

### Lint & Format
```bash
cargo clippy             # Run linter
cargo fmt                # Format code
cargo fmt --check        # Check formatting
```

## Project Structure

```
gcodekit/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── lib.rs                  # Library exports
│   ├── communication/          # GRBL protocol & serial communication
│   │   ├── mod.rs             # Main controller interface
│   │   └── grbl.rs            # Version parsing
│   ├── designer/               # CAM functions
│   │   ├── mod.rs             # Design management
│   │   ├── shapes.rs          # Geometry primitives
│   │   └── toolpath.rs        # G-code optimization
│   ├── jobs/                   # Job scheduling
│   │   └── mod.rs             # Job queue and manager
│   ├── materials/              # Material database
│   │   └── mod.rs             # Material profiles
│   └── widgets/                # UI components
│       ├── mod.rs             # Widget exports
│       ├── connection.rs       # Device connection
│       ├── jog.rs             # Jogging controls
│       ├── overrides.rs        # Feed/power adjustments
│       └── gcode_loading.rs    # File management
├── ui/
│   └── app.slint              # Main UI definition
├── tests/                      # Integration tests
├── Cargo.toml                 # Dependencies
├── build.rs                   # Slint build script
├── README.md                  # This file
├── SPEC.md                    # Feature specification
└── AGENTS.md                  # Development guidelines
```

## Test Coverage

**Total Tests**: 365+  
**Pass Rate**: 100%  
**Test Organization**: Tests located in `tests/` folder organized by module hierarchy (communication/, designer/, jobs/, materials/, widgets/)
**Coverage Areas**:
- Communication: GRBL protocol and serial communication tests
- Designer: CAM, shapes, boolean operations, and toolpath tests
- Jobs: Job scheduling, queue, priority, and progress tracking tests
- Materials: Material database and management tests
- Widgets: UI widget tests (connection, jog, overrides, loading)

## Build Information

- **Debug Build**: 222MB (with debug symbols)
- **Release Build**: 13MB (optimized)
- **Startup Time**: <2 seconds
- **Response Time**: <100ms for UI updates
- **G-code Processing**: 1000+ lines/second
- **Memory Usage**: ~50MB baseline

## System Requirements

- **OS**: Linux, Windows, or macOS
- **Rust**: 1.75+ (2024 edition)
- **GRBL Device**: v1.1+ compatible controller
- **Serial Port**: USB or native serial connection

## Version History

### v0.2.0-alpha (Oct 19, 2025 - Phase 2 MVP & Theme Infrastructure)
- **Phase 2 MVP Implementation**: Complete foundation for all major features
- **System Theme Support**: Full infrastructure for Light/Dark theme support with system detection ✅
- **Theme Infrastructure**: 
  - System theme detection (Windows, macOS, Linux) ✅
  - Color palettes with WCAG AA compliance ✅
  - Slint UI theme provider with reactive colors ✅
  - Persistent theme preferences ✅
  - Dynamic theme switching capability 🔄
- **Test Coverage**: 365+ tests organized in tests/ hierarchy (100% passing)
- **Build Status**: Debug and release builds optimized
- **Code Quality**: All tests passing, theme infrastructure complete
- **Documentation**: All markdown docs in docs/ folder (except SPEC.md, README.md, AGENTS.md in root)
- **Ready for**: Phase 14.2 (Theme UI), 14.3 (Settings Integration), 14.4 (Full Adaptation)

### v0.1.0-alpha (Oct 19, 2025)
- Initial production-quality MVP
- Full GRBL communication stack
- Shape-based CAM workflows
- Priority job scheduling
- Material database integration
- 365+ passing tests
- Cross-platform builds

## Contributing

Contributions are welcome! Please follow the guidelines in `AGENTS.md` for:
- Code style (Rust 2024 edition, 4-space indentation)
- Documentation (docblocks for all public APIs)
- Testing (comprehensive unit and integration tests)
- Git workflow (feature branches, descriptive commits)

## License

[Your License Here]

## Support

For issues, feature requests, or questions:
1. Check the SPEC.md for planned features
2. Review existing issues on GitHub
3. Submit detailed bug reports with reproduction steps

