
gcodekit is a desktop GUI application that allows users to control laser engravers or CNC machines using GRBL firmware. The application provides comprehensive machine control, advanced CAM (Computer-Aided Manufacturing) functions for G-code generation, and robust error recovery capabilities ensuring 99.9% uptime. The application is multiplatform, working on Linux, Windows, and macOS.

The device should supply the following features:

1. Layout
	a. The application will have status bar that is attached to the bottom of the application window. This will be known as the "Status bar"
	b. The application will have a combined menu and title bar attached to the top of the application window. This will be known as the "Menu Bar"
	c. The application will have a left hand tool pannel dedicated to machine connect and control. all the widgets in this pannel will as wide as the tool pannel and will be stacked ontop of each other. 
	d. The Application will have a right hand tool pannel dedicated to CAM functions, all the widgets in this pannel will be as wide as the tool pannel and will be stacked ontop of each other.
	e. The Application will have a central panel that is tabbed. The tabs will run across the top and they will show "G-code Editor", "3D Visualizer", "Device Console", "Job Manager", and "Designer"


2. Widget functions
1. A lefthand tool panel with modular widgets stacked vertically:
 	a. Connection widget (connection.rs): Device selection and connection management with status display
 	b. G-code loading widget (gcode_loading.rs): File selection, loading, and queued sending to prevent buffer overruns
 	c. Jog widget (jog.rs): Real-time axis control (X/Y/Z) with configurable step sizes (0.1, 1, 10, 50mm) Buttons for Home, X+, X-, Y+, Y-, Z-, Stop, unlock, pause and continue should be provided. 
 	d. Overrides widget (overrides.rs): Real-time spindle/laser power and feed rate adjustments
2. A righthand tool panel dedicated to CAM functions with modular widgets:
 	a. Shape generation widget (shape_generation.rs): Create basic shapes (rectangles, circles) with adjustable dimensions
 	b. Toolpath generation widget (toolpath_generation.rs): Convert shapes to GRBL-compatible G-code with feed rates and spindle/laser controls
 	c. Vector import widget (vector_import.rs): Load SVG/DXF files and convert to G-code for engraving/cutting
 	d. Image engraving widget (image_engraving.rs): Convert bitmap images to GRBL-compatible G-code for laser engraving with adjustable resolution and intensity
 	e. Tabbed box widget (tabbed_box.rs): Generate cutting paths for boxes with interlocking tabs, with adjustable dimensions, tab size, and material thickness
 	f. Jigsaw widget (jigsaw.rs): Generate laser cutting paths for interlocking puzzle pieces with adjustable piece count, size, and complexity
3. Status bar, shows the connection/disconnection status, device state (idle/alarmed), current position (X/Y/Z), and GRBL version when connected.
  4. Communication module (communication/grbl.rs): Handles GRBL protocol communication including serial port management, command sending, response parsing, version detection, and real-time status monitoring.
  5. 3D Visualizer: Interactive G-code visualization with color-coded paths (rapid moves blue, feed moves green, arcs yellow), right-click jog to position, left-click path selection with highlighting, real-time machine position overlay with 3-axis support (XYZ).
  6. Job Manager: Comprehensive job queuing system with priority-based scheduling, progress tracking, pause/resume functionality, and automatic job resumption after communication errors.

Technology: Built with Rust language (2024 edition), using cargo build and cargo test for compilation and testing, with slint 1.13.1 1or the GUI interface. Core dependencies include:
- serialport (4.2) for serial communication
- tokio (1.0) for async runtime
- tracing (0.1) for structured logging
- rfd (0.14) for file dialogs
- anyhow (1.0) for error handling
- serde (1.0) for serialization

- chrono (0.4) for timestamps
- uuid (1.0) for job identification
- usvg (0.37) for SVG parsing
- dxf (0.4) for DXF file parsing
- lyon (1.0) for 2D graphics and path operations
- image (0.24) for bitmap processing

Architecture: Modular design with separate modules for:
- communication: GRBL protocol handling, serial communication, and error recovery
- designer: CAD/CAM design tools and shape manipulation
- jobs: Job management, queuing, and resumption capabilities
- materials: Material database and properties
- widgets: Individual UI components for different functions
- main: Application state and UI orchestration
- tests: Unit and integration tests organized in hierarchy mirroring src/

Development Tools:
- cargo clippy: Linting with clippy
- cargo fmt: Code formatting with rustfmt
- cargo check: Fast compilation checking
- cargo test: Run unit tests and integration tests

System Requirements:
- Rust 1.75+ (2024 edition)
- GRBL v1.1+ compatible device
- Serial port access for device communication

Additional Requirements:
1. GRBL Version Support: Prioritize GRBL v1.1 and v1.2 features including real-time overrides and jogging
2. Device Compatibility: Support GRBL controllers
3. Menu Structure: Follow Universal G-Code Sender (UGS) menu structure with File, Machine, View, Tools, and Help menus
4. Machine Types: Support both laser engraver and CNC machine commands with automatic mode detection
5. G-code Compatibility: Implement only G-code features supported by GRBL firmware
6. CAM Functions: Include basic Computer-Aided Manufacturing capabilities for generating G-code from shapes and images
7. Version Detection: Capture and display GRBL firmware version on the status bar during connection
8. Code Style: Follow Rust formatting (4 spaces, max 100 width), snake_case naming, structured error handling with anyhow
9. Logging: Use tracing for structured logging, avoid println! in production code
10. Modular Architecture: Separate communication logic from UI components for maintainability
11. Testing: Implement comprehensive unit tests for all components using `cargo test`. Tests should cover core functionality, edge cases, and error conditions. Unit tests must pass as part of the build process and CI/CD pipeline. All tests must be organized in the `tests/` folder with hierarchy mirroring the `src/` directory.
12. Error Recovery: Implement 99.9% uptime guarantee through automatic error recovery, job resumption, and comprehensive logging
13. 3-Axis Support: Optimized for GRBL machines with dedicated X/Y/Z axis control
14. Job Management: Advanced job queuing with priority scheduling, progress tracking, and automatic resumption after errors
15. System Theme Support: Support Light and Dark themes with automatic detection of OS theme preference, dynamic theme switching without application restart, and WCAG AA accessibility compliance for all UI components
16. UI Theme Appearance: All UI components including buttons, panels, text fields, status indicators, and menus must adapt their appearance based on the currently selected theme to reflect the system theming preference

 Current Features:
  12. **Advanced Visualizer**: Right-click jog to location ✓, color-coded paths for G0/G1/G2/G3 moves ✓, 3-axis support (XYZ) ✓, real-time machine position overlay ✓, outline gcode functionality (planned)
  13. **Designer Tab Foundation**: Basic shape drawing (Rectangle ✓, Circle ✓, Line ✓) with interactive canvas, shape selection, and G-code export ✓
  14. **Modular Architecture**: Clean separation of concerns with dedicated modules for communication, designer, jobs, materials, and widgets ✓
  15. **Advanced Error Recovery System**: 99.9% uptime guarantee through automatic error recovery, job resumption, and comprehensive logging ✓
  16. **Job Management System**: Priority-based job queuing, progress tracking, pause/resume functionality, and automatic resumption after communication errors ✓
  17. **3-Axis Support**: Optimized for GRBL machines with X/Y/Z axis control and G-code parsing ✓
  18. **Enhanced Communication**: Support for GRBL controllers ✓
  19. **Vector Import**: SVG/DXF file import with automatic G-code conversion ✓
  20. **Boolean Operations**: Shape union operations for combining geometric elements ✓
  21. **Probing Routines**: Z-probing, auto-leveling, and workpiece measurement with G38.x commands ✓
  22. **Tool Management**: Tool length offsets (G43/G49), tool change support, and tool libraries ✓
  23. **Keybinding Customization**: Configurable keyboard shortcuts for all major actions ✓
  24. **UI Stability**: Resolved all duplicate element IDs in slint interface for reliable dropdown menus and button interactions ✓

  26. **Configurable UI System**: Dockable window functionality with toggleable left/right panels via View menu ✓
  27. **Advanced CAM Operations**: Part nesting algorithm using bottom-left fill strategy with rotation support ✓
28. **Test Reorganization**: Tests moved to tests/ folder with hierarchy mirroring src/ ✓
29. **Build Fixes**: Compilation errors resolved and debug binary built ✓
30. **Repository Updates**: Changes committed ✓
31. **Port Filtering**: Serial ports filtered to show only GRBL-compatible devices (/dev/ttyACM*, /dev/ttyUSB*, COM*, /dev/tty.usbserial*) ✓
32. **Issue Templates**: GitHub issue templates added for BUG, FEATURE, and CHANGE requests ✓
33. **Code Quality**: Clippy warnings fixed for improved code maintainability ✓
34. **System Theme Support**: Dynamic UI adaptation to system theme preference (Light/Dark mode) (planned)

## Phase 9: Advanced Error Recovery System (99.9% Uptime Guarantee)

The advanced error recovery system provides comprehensive fault tolerance and automatic recovery capabilities:

### Error Recovery Features:
- **Automatic Reconnection**: Detects connection loss and attempts automatic reconnection with configurable retry limits
- **Command Retry Logic**: Automatically retries failed commands with exponential backoff
- **Critical Error Handling**: Handles alarms and emergency conditions with controller reset capabilities
- **Job Resumption**: Automatically interrupts jobs on errors and enables resumption from the last completed line
- **Comprehensive Logging**: Detailed logging of all recovery attempts, actions taken, and outcomes with timestamps

### Job Management System:
- **Priority-based Queuing**: Jobs are queued with priority levels (1-10) for optimal scheduling
- **Progress Tracking**: Real-time progress updates based on completed G-code lines
- **Pause/Resume Functionality**: Manual and automatic job pausing with resumption capabilities
- **Error Recovery Integration**: Jobs automatically resume from interruption points after communication recovery

### 3-Axis Support:
- **XYZ Axis Support**: Optimized for GRBL machines with dedicated 3-axis control
- **G-code Parsing**: Parser for X/Y/Z G-code commands (rotary commands ignored)
- **Position Tracking**: Real-time position monitoring for X/Y/Z axes
- **Jog Controls**: Theme-aware jog controls with 60×60 buttons and configurable step sizes

### Technical Implementation:
- **Recovery Configuration**: Configurable retry attempts, delays, and auto-recovery settings
- **State Management**: Comprehensive recovery state tracking with action history
- **UI Integration**: Job manager UI with resume controls for interrupted jobs
- **Error Classification**: Intelligent error categorization for appropriate recovery actions

## Phase 9: Advanced Job Scheduling System

The advanced job scheduling system provides enterprise-grade production management capabilities:

### Job Scheduling Features:
- **Time-based Execution**: Schedule jobs to run at specific times with recurring intervals (minutes, hours, days, weeks, months)
- **Dependency Management**: Jobs can depend on completion of other jobs before execution begins
- **Recurring Schedules**: Configurable repeat intervals with optional maximum run limits
- **Priority Integration**: Scheduled jobs respect the existing priority-based queuing system
- **Persistence**: Scheduled jobs are saved to disk and restored on application restart

### Scheduling UI Components:
- **Job Scheduling Widget**: Complete interface for creating and managing scheduled jobs
- **Dependency Selection**: Choose from completed jobs to create execution dependencies
- **Schedule Monitoring**: View upcoming jobs, current schedules, and execution history
- **Manual Execution**: Process scheduled jobs on-demand for testing and immediate execution

### Advanced Features:
- **Complex Scheduling**: Support for complex production workflows with job chains and dependencies
- **Error Recovery Integration**: Scheduled jobs work seamlessly with the 99.9% uptime guarantee
- **3-axis Compatibility**: Full support for XYZ-axis scheduled jobs
- **Performance Tracking**: Monitor execution times, success rates, and scheduling efficiency

## Phase 10: Advanced CAM Features and Controller Support ✅ COMPLETED

The advanced CAM features and controller support phase extends gcodekit's capabilities with professional-grade features:

### Configurable UI System:
- **Dockable Windows**: Toggleable left/right panels via View menu for customizable workflows
- **Flexible Layout**: Highly configurable interface layouts to suit different user preferences

### Advanced CAM Operations:
- **Part Nesting**: Bottom-left fill strategy with rotation support for material optimization
- **Positioned Parts**: Structs for nesting configuration and positioned parts management

### Testing & Validation:
- **Comprehensive Testing**: 41 passing tests covering core functionality and new features
- **Release Build**: Successful optimized release build ensuring production readiness

## Phase 11: Advanced 3D Machining

The advanced 3D machining phase adds sophisticated 3D capabilities:

### Advanced 3D Surface Machining:
- **Waterline Machining**: Horizontal slicing for 3D surface machining
- **Scanline Machining**: Vertical slicing with morphing capabilities
- **3D Profiling**: Complex surface machining strategies

### STL Processing:
- **File Import**: STL mesh import with automatic repair
- **Mesh Processing**: Surface triangulation and optimization
- **3D Visualization**: Real-time 3D rendering at 30+ FPS

### 3-Axis Optimization:
- **XYZ Strategies**: Optimized machining operations for GRBL machines
- **Simplified Architecture**: Removed rotary axis complexity for cleaner codebase

## Phase 12 & 13: Real-Time Machine Status Monitoring & Device Console Integration ✅ COMPLETED

The real-time machine status monitoring and device console integration phases add professional-grade machine monitoring and diagnostic capabilities:

### Phase 12: Real-Time Machine Status Display
- **Status Update Integration**: Enhanced app state with real-time machine status monitoring
- **Bottom Status Bar Redesign**: Displays connection status, machine state (with color coding), machine position (MPos/WPos), feed rate, and spindle speed
- **Color-Coded Status**: Green (Idle), Blue (Run/Jog), Yellow (Hold/Door), Red (Alarm), Gray (Unknown/Sleep/Check)

### Phase 13: Device Console Integration  
- **Enhanced Device Console Tab**: Severity-based filtering (Error, Warning, Info, Debug) with independent toggles
- **Color-Coded Messages**: Type and severity-based message coloring for easy scanning
- **Automatic Filtering**: Status queries and simple "ok" responses automatically excluded from display
- **Message Counts**: Real-time message count display with copy/clear controls

## Phase 14: System Theme Support (Light/Dark Mode) - 

The UI dynamically adapts to the system theme preference, providing a seamless user experience across different environments:

### Theme Support Features 
- **Light Theme**: Professional light color scheme with dark text, light backgrounds, appropriate contrast 
- **Dark Theme**: Professional dark color scheme with light text, dark backgrounds, reduced eye strain 
- **System Theme Detection**: Automatic detection of OS-level theme preference (Windows, macOS, Linux) 
- **Theme Switching**: Real-time theme switching without application restart, reflecting system theme changes automatically 
- **Preferences Storage**: Remember user theme preference across sessions 
- **Accessibility**: Ensure sufficient contrast ratios (WCAG AA minimum 4.5:1) in both themes 
- **Component Styling**: All UI components (buttons, panels, text fields, menus) adapt appearance based on selected theme 

### Theme Implementation:
- **Theme Engine**: Central theme management system with theme definitions
- **Dynamic Colors**: All UI colors must be derived from theme palette for consistency
- **Component Styling**: Buttons, text fields, panels, and other widgets adapt to current theme
- **Icon/Imagery**: Adjust image rendering for visibility in both light and dark modes
- **Status Indicators**: Ensure status colors (green/red/yellow/blue) are visible and distinct in both themes
- **Custom Theme Support**: Allow users to define custom color schemes (future phase)

### Color Palette Guidelines:

**Light Theme**:
- Background: #FFFFFF (white)
- Primary Text: #1A1A1A (near black)
- Secondary Text: #666666 (medium gray)
- Panel Background: #F5F5F5 (light gray)
- Button/Active: #0066CC (blue)
- Accent: #FF6B35 (orange)
- Status Green: #00AA00 (green for idle)
- Status Blue: #0000FF (blue for run)
- Status Red: #FF0000 (red for alarm)
- Status Yellow: #FFAA00 (yellow for hold)

**Dark Theme**:
- Background: #1E1E1E (dark gray)
- Primary Text: #FFFFFF (white)
- Secondary Text: #CCCCCC (light gray)
- Panel Background: #2D2D2D (medium dark gray)
- Button/Active: #4DA6FF (light blue)
- Accent: #FF8C42 (light orange)
- Status Green: #00FF00 (bright green)
- Status Blue: #4DA6FF (bright blue)
- Status Red: #FF3333 (bright red)
- Status Yellow: #FFDD00 (bright yellow)

### User Preferences:
- **Theme Selection**: Settings panel option to select between Light, Dark, and System Default
- **Auto Follow System**: When set to "System Default", automatically follow OS theme changes
- **Transition**: Smooth color transitions when theme changes (200-300ms fade)
- **Per-Component Override**: Advanced users can override specific component colors

### Testing & Validation:
- Visual testing in both light and dark modes
- Contrast ratio validation tools to ensure WCAG compliance
- Cross-platform theme detection (Windows, macOS, Linux)
- Performance testing for theme switches
- User preference persistence testing

---

## Implementation Phases

### Phase 1-8: Core Foundation 
- GRBL communication protocol
- GUI framework with slint
- Basic CAM functions
- Multi-axis support (XYZ)

### Phase 9: Advanced Error Recovery & Job Management 
- 99.9% uptime guarantee
- Job queuing and scheduling
- Automatic error recovery

### Phase 10: Configurable UI & Advanced CAM 
- Dockable windows
- Part nesting with rotation support
- Comprehensive testing (41+ tests)

### Phase 11: Advanced 3D Machining 
- Waterline machining
- STL processing
- 3D visualization

### Phase 12-13: Real-Time Monitoring & Device Console 
- Real-time machine status display
- Device console with filtering
- Color-coded status indicators

### Phase 14: System Theme Support (CURRENT PHASE)
- Light/Dark theme support
- System theme detection
- Real-time theme switching
- Accessibility compliance (WCAG AA)

### Phase 15+: Future Enhancements
- Advanced 3D CAM
- Lathe operations
- Scripting/automation
- Additional features as planned

## Development Status

**Current Phase**: Phase 15 - MVP Implementation & Continuous Enhancement (IN PROGRESS)
**Implementation Status**: MVP v0.2.6-alpha - Device Communications & UI Polish Complete
**Version**: 0.2.6-alpha
**Test Coverage**: 463 passing tests (100%) including device communication and integration tests
**Architecture**: Modular, extensible design with stable UI framework, advanced CAM capabilities, real-time status monitoring, complete theme support, WCAG AA accessibility compliance, remote control capabilities, visual G-code simulation, G-code validation, file optimization, and device communications
**UI Theme Support**:  System detection, dynamic switching, WCAG AA compliance, all components themed, settings panel with theme selection
**Device Communications**: Real serial port integration, GRBL protocol support, async command handling, automatic reconnection, recovery configuration
**Web Pendant Interface**:  RESTful API, WebSocket streaming, mobile-responsive HTML5, theme support, comprehensive tests
**UI Polish**: Auto-sizing widgets, all SPEC-required jog buttons, white text on buttons, responsive layout
**Documentation**: All markdown files organized in docs/ (except SPEC.md, AGENTS.md, README.md, CHANGELOG.md), complete implementation plan with phase breakdown
**Test Organization**: All tests in tests/ folder with hierarchy mirroring src/ directory
**Recent Session Focus**: Device communications implementation, UI button updates, auto-sizing widgets
**Completed Tasks**: Tasks 1, 2, 3 (Validator, Back Plotter, Optimizer)

⚠️ **Alpha Notice**: This software is under active development. While functional and tested, it may contain bugs and the API may change in future releases. Use with appropriate caution.


---


Tasks are organized by module and numbered sequentially. Completed tasks are marked with ✅.




---

# Task Tracking

Tasks are organized by module and numbered sequentially (1-7). Completed tasks are marked with ✅.

## Designer Module - Core Tasks

### Task 1: G-code Validator (Syntax & Semantic Validation) ✅ COMPLETED

Professional-grade G-code validation with comprehensive error reporting and GRBL compatibility checking:

### Core Features:
- **Syntax Validation**: G-code command parsing and structure validation ✅
- **Semantic Validation**: Parameter range checking and value validation ✅
- **GRBL Version Support**: Version-specific validation for GRBL v1.0, v1.1, v1.2 ✅
- **Error Classification**: Severity levels (Info, Warning, Error, Critical) ✅
- **Issue Reporting**: Detailed messages with line numbers and suggestions ✅
- **Comment Handling**: Proper parsing of comments (full-line and inline) ✅
- **Parameter Validation**: Feed rates, spindle speeds, coordinates ✅
- **Configurable Rules**: Enable/disable validation rules per application needs ✅

### Implementation Details:
- **GcodeValidator**: Core validation engine with configurable rules and version support ✅
- **ValidationIssue**: Complete issue reporting with severity, type, message, and suggestions ✅
- **Severity Enum**: Info, Warning, Error, Critical with hashable ordering ✅
- **GrblVersion Enum**: V1_0, V1_1, V1_2 with version comparison support ✅
- **Feed Rate Validation**: Positive values, warnings for > 20000 mm/min ✅
- **Spindle Speed Validation**: Non-negative values, warnings for > 30000 RPM ✅
- **Coordinate Parsing**: Support for decimal coordinates (10.5, -20.75, etc.) ✅
- **Command Parsing**: Multi-character commands (G1, M3, etc.) with parameter extraction ✅

### Test Coverage:
- **Unit Tests**: 17 tests in src/designer/validator.rs covering:
  - Validator creation and configuration ✅
  - Feed rate validation (positive, high values) ✅
  - Spindle speed validation (negative, high values) ✅
  - Program parsing and validation ✅
  - Comment handling (full-line and inline) ✅
  - Version compatibility checks ✅
  - Rule enable/disable ✅
  - Issue summary and statistics ✅
  - Error classification and severity ✅

- **Integration Tests**: 21 tests in tests/designer/validator.rs covering:
  - Complete valid programs ✅
  - Mixed error/warning scenarios ✅
  - Realistic engraving/cutting programs ✅
  - Complex multi-command validation ✅
  - Edge cases (empty lines, whitespace, etc.) ✅
  - Coordinate parsing with decimals ✅
  - Inline comments ✅
  - Version compatibility workflows ✅

- **Test Statistics**:
  - Total Validator Tests: 38 (17 unit + 21 integration)
  - Project Total: 397 tests (all passing)
  - Code Coverage: All public APIs and validation logic fully tested

### API Reference:
```rust
// Create a validator for specific GRBL version
let validator = GcodeValidator::new(GrblVersion::V1_2);

// Validate entire program
let issues = validator.validate_program(gcode_string);

// Validate single line
let issues = validator.validate_line("G1 X10 Y20 F1000", 1);

// Configure validation
validator.set_validate_syntax(true);
validator.set_validate_semantics(true);
validator.set_rule_enabled("rule_name", false);

// Query results
let has_errors = GcodeValidator::has_critical_errors(&issues);
let summary = GcodeValidator::get_summary(&issues);

// Issue details
for issue in issues {
    println!("Line {}: [{}] {} - {}", 
        issue.line_number,
        issue.severity,
        issue.issue_type,
        issue.message
    );
    if let Some(suggestion) = &issue.suggestion {
        println!("  Suggestion: {}", suggestion);
    }
}
```

### Validation Rules (Built-in):
- **Command Availability**:
  - G0 (Rapid): GRBL v1.0+
  - G1 (Linear): GRBL v1.0+
  - G2/G3 (Arcs): GRBL v1.1+
  - G4 (Dwell): GRBL v1.0+
  - G10 (Set Position): GRBL v1.1+
  - G28/G30 (Positioning): GRBL v1.0+ / v1.1+
  - G38.x (Probing): GRBL v1.1+
  - G43/G49 (Tool Offset): GRBL v1.1+

- **M-Code Availability**:
  - M3 (Spindle CW): GRBL v1.0+
  - M4 (Spindle CCW): GRBL v1.1+
  - M5 (Spindle Stop): GRBL v1.0+

- **Parameter Ranges**:
  - Feed Rate (F): 0 < F < 20000 (warning above threshold)
  - Spindle Speed (S): 0 <= S <= 30000 (warning above threshold)
  - Coordinates (X/Y/Z): Any numeric value with decimal support

### Severity Levels:
- **Info**: Informational message, normal operation
- **Warning**: May cause unexpected behavior, review recommended
- **Error**: Command likely to fail or malfunction
- **Critical**: Command will definitely fail or risk equipment damage

### Future Enhancements:
- Complex semantic validation (spindle state consistency, etc.)
- Tool change validation and tracking
- Coordinate limit checking against machine dimensions
- Modal state validation (checking conflicting mode settings)
- Tool length offset tracking and validation
- Custom user-defined validation rules
- Performance optimization for large files (100k+ lines)
- Real-time incremental validation for editor integration



### Task 2: Back Plotting (Visual G-code Simulator) ✅ COMPLETED

Professional-grade visual G-code simulation with step-through execution:

### Core Features:
- **Step-Through Execution**: Forward/backward stepping through G-code with real-time position tracking ✅
- **Jump to Step**: Quick navigation to specific step numbers in G-code sequence ✅
- **Pause/Resume**: Full control over simulation execution with state management ✅
- **Progress Tracking**: Real-time progress calculation showing simulation completion percentage (0-100%) ✅
- **State Management**: Idle/Running/Paused/Completed states with automatic state transitions ✅
- **Step History**: Maintains execution history for undo/redo capabilities ✅

### Implementation Details:
- **BackPlotter**: Core simulator struct with step management, position tracking, and state management ✅
- **BackPlotStep**: Represents individual G-code move with line number, start/end positions (XYZ), feed rate, spindle speed, and G-code command ✅
- **MoveType**: Enum supporting Rapid (G0), Linear (G1), Clockwise Arc (G2), Counter-clockwise Arc (G3), Dwell (G4), Other ✅
- **BackPlotState**: Enum for Idle/Running/Paused/Completed states ✅
- **Position Tracking**: 3-axis XYZ coordinate tracking throughout simulation ✅
- **Command Preservation**: Full G-code command strings preserved for reference and UI display ✅

### Test Coverage:
- **Unit Tests**: 18 comprehensive tests in src/designer/backplot.rs covering:
  - BackPlotter creation and initialization ✅
  - Forward/backward stepping with proper state transitions ✅
  - Jump to arbitrary step numbers ✅
  - Pause/resume functionality ✅
  - Position tracking through complex programs ✅
  - Progress calculation (0-100%) ✅
  - Stop and reset functionality ✅
  - Step history and retrieval ✅
  - Move type classification (G0/G1/G2/G3) ✅
  - Error handling (empty steps, out-of-bounds access) ✅

- **Integration Tests**: 15 comprehensive tests in tests/designer/backplot.rs covering:
  - Full program simulation with 7-step complex program ✅
  - Complete navigation patterns (forward, backward, jump) ✅
  - Position tracking throughout entire program ✅
  - Move type classification for all supported types ✅
  - Speed and spindle speed tracking ✅
  - Reset and stop functionality workflows ✅
  - Complex navigation patterns (step→jump→backward→jump) ✅
  - G-code command preservation ✅

- **Test Statistics**:
  - Total Backplot Tests: 33 (18 unit + 15 integration)
  - Project Total: 282 tests (all passing)
  - Code Coverage: All public APIs and core functionality fully tested

### API Reference:
```rust
// Create a new back-plotter
let mut bp = BackPlotter::new(steps)?;

// Navigate through G-code
bp.step_forward();        // Execute next step
bp.step_backward();       // Undo previous step
bp.jump_to_step(5)?;      // Jump to specific step

// Control simulation
bp.pause();               // Pause execution
bp.resume();              // Resume from pause
bp.stop();                // Stop and reset
bp.reset();               // Reset to beginning

// Query state
bp.get_state();           // Current state (Idle/Running/Paused/Completed)
bp.get_position();        // Current XYZ position
bp.get_current_step();    // Current step number
bp.get_progress();        // Progress 0-100%
bp.get_total_steps();     // Total steps in program
bp.get_current_step_ref(); // Reference to current step
bp.get_steps();           // All steps in program
bp.get_step(index);       // Specific step by index
```

### Future Enhancements:
- Speed control multiplier (0.1x to 5.0x) for playback speed adjustment
- UI widget integration for visualization in Designer tab
- Line highlighting in G-code editor synchronized with back-plot stepping
- Path visualization in 3D visualizer showing executed path
- Real-time feedrate and spindle speed display




### Task 3: Advanced G-code Optimizer ✅ COMPLETED

Professional-grade G-code optimization with multiple advanced techniques for file size reduction and performance improvement.

#### Core Features Implemented:
- **Decimal Precision Truncation**: Reduces numeric values to specified decimal places (0-6) while maintaining accuracy ✅
- **Arc-to-Line Conversion**: Converts G2/G3 arc commands to sequences of G1 line segments (optional) ✅
- **Redundant Whitespace Removal**: Eliminates excessive spacing and empty lines while preserving structure ✅
- **Comment Preservation**: Maintains both inline and full-line comments throughout optimization ✅
- **Multi-line Support**: Handles G-code programs with 100k+ lines efficiently ✅

#### Implementation Details:
- **GcodeOptimizer**: Core optimizer struct with configurable options ✅
- **OptimizerOptions**: Configuration struct with 6 customizable parameters ✅
- **OptimizationStats**: Result tracking with before/after metrics ✅
- **truncate_decimal_precision()**: Intelligent decimal truncation with negative number support ✅
- **convert_arcs_to_lines()**: Arc approximation using chord error method ✅
- **remove_redundant_whitespace()**: Whitespace and empty line cleanup ✅
- **optimize()**: Orchestrates multiple optimizations in one call ✅

#### Test Coverage:
- **Unit Tests**: 17 tests in src/designer/optimizer.rs covering:
  - Optimizer creation and configuration ✅
  - Decimal precision truncation (various decimal places) ✅
  - Whitespace removal and collapsing ✅
  - Comment handling (inline and full-line) ✅
  - G-code and M-code preservation ✅
  - Negative coordinate handling ✅
  - Arc conversion (basic) ✅
  - Parameter validation ✅

- **Integration Tests**: 24 tests in tests/designer/optimizer.rs covering:
  - Complete program optimization ✅
  - Realistic cutting programs ✅
  - Realistic laser engraving programs ✅
  - Decimal place variations (0-3) ✅
  - Mixed integer and decimal values ✅
  - Very small and very large numbers ✅
  - Statistics tracking and validation ✅
  - Comment preservation workflows ✅

- **Test Statistics**:
  - Total Optimizer Tests: 41 (17 unit + 24 integration)
  - Project Total: 455 tests (all passing)
  - Code Coverage: All public APIs and optimization logic fully tested

#### Performance Metrics:
- **File Size Reduction**: 20-40% typical for dense G-code with high precision
- **Supports**: 0-6 decimal places for different precision requirements
- **Arc Tolerance**: Configurable (default 0.05mm) for quality/speed tradeoff
- **Handles**: 100k+ line files efficiently with all G/M codes and comments

#### API Reference:
```rust
// Create optimizer with default options
let optimizer = GcodeOptimizer::new();

// Customize optimization
let mut options = OptimizerOptions::default();
options.decimal_places = 3;
options.convert_arcs = true;
let optimizer = GcodeOptimizer::with_options(options);

// Optimize complete program
let optimized = optimizer.optimize(gcode_string)?;

// Get statistics
let stats = GcodeOptimizer::get_stats(original, &optimized);
println!("Reduction: {}%", stats.size_reduction_percent);
```

#### Optimization Examples:

**Before**: `G0 X10.55555555 Y20.77777777 F1000.99999 S5000.55555`
**After**: `G0 X10.55 Y20.77 F1000.99 S5000.55`
**Reduction**: ~20 bytes (22%)

**Typical Program Reduction**: 20-40% depending on precision and formatting



### Task 4: Advanced CAM Boolean Operations 

Professional-grade geometric boolean operations for advanced part design and CAM workflows:

### Core Boolean Operations:
- **Union**: Combines two polygons, handling both intersecting and non-intersecting cases
- **Subtraction**: Removes one polygon from another, useful for pocket operations and hole generation
- **Intersection**: Finds overlapping region of two polygons for analysis and feature extraction
- **Area Calculation**: Computes polygon area using shoelace formula for accurate material calculations

### Advanced CAM Features:
- **Region Fill Scanlines**: Generates horizontal scanline patterns for pocket machining with configurable spacing
- **Automatic Holding Tabs**: Creates evenly-spaced tabs along part perimeter for retention during cutting, with configurable width/height
- **Polygon Simplification**: Removes collinear points to reduce vertices and improve performance
- **Bounding Box Operations**: Fast geometric checks for optimization and collision detection

### Point-in-Polygon & Geometry:
- **Ray Casting Algorithm**: Accurate point-in-polygon testing for part nesting and toolpath validation
- **Centroid Calculation**: Computes polygon center for angle-based vertex sorting in boolean operations
- **Perimeter Calculation**: Determines total edge length for tab spacing calculations
- **Tangent Slope Computation**: Calculates edge tangents for proper tab orientation

### Toolpath Generation:
- **Boundary Following**: Generates complete toolpath (rapid, plunge, feed, return) from polygon outline
- **Tool Compensation**: Prepares paths for CNC execution with proper Z-height handling
- **Safe Height Integration**: Automatically incorporates safe Z moves between operations

### Test Coverage:
- 19 comprehensive integration tests covering all boolean operations
- Tests include: area calculation, point-in-polygon, bounding boxes, union/subtraction/intersection, region fill, holding tabs, complex workflows
- All tests passing with 399 total tests in project
- Support for overlapping/non-intersecting polygons, small and large parts, complex sequences





## Settings & Configuration Module

### Task 5: Settings Management System 

Comprehensive machine profile and settings management enabling users to:
- **Save/Load GRBL Machine Profiles**: Store custom machine configurations with GRBL parameters
- **Multi-Machine Support**: Switch between different machine configurations instantly
- **Settings Backup/Restore**: Backup all profiles with timestamped directories
- **Import/Export**: Share profiles across machines and platforms
- **Profile Management UI**: Intuitive interface for creating, activating, and deleting profiles

### Implementation Details:
- **ProfileSettings**: Struct with all GRBL machine parameters (step rates, feed rates, acceleration, spindle speeds, soft limits, axis inversions)
- **MachineProfile**: Complete profile with metadata, machine type, port configuration
- **ProfileManager**: In-memory profile management with active profile tracking
- **SettingsStorage**: Persistent JSON-based storage in platform-specific config directories
- **UI Integration**: Settings panel with profile list, creation dialog, delete confirmation
- **Storage Location**: `~/.config/gcodekit/profiles/` (Linux), `%APPDATA%\gcodekit\profiles\` (Windows), `~/Library/Application Support/gcodekit/profiles/` (macOS)

### Test Coverage:
- 16 tests covering profile creation, management, persistence, and UI state
- All tests passing (362 total tests in project)
- Full error handling with anyhow::Result




## Future Enhancement Tasks

### Task 6: Advanced G-code Editor

Built-in G-code editor with syntax highlighting, error checking, and manual modifications.

**Planned Implementation:**
- Custom slint text widget with buffer and cursor management
- GRBL G/M code vocabulary (v1.0, v1.1, v1.2)
- Configurable validation rules
- Incremental tokenizer and parser
- Line numbers, diagnostics, find/replace, code folding
- Auto-completion for G/M codes
- Editor-visualizer line mapping integration
- Real-time validation (<100ms for 1000+ line files)


### Task 7: Speeds and Feeds Calculator

Built-in calculator for optimizing cutting parameters based on material and tool.

**Planned Implementation:**
- Material-based parameter optimization
- Tool specification support
- Feed rate recommendations
- Spindle speed calculations


---

## Feature List Reference

CamBam-Inspired Features:
37. **Advanced G-code Editor**: Built-in G-code editor with syntax highlighting, error checking, and manual modifications

    - Implementation: custom slint text widget (src/gcodeedit/editor.rs) providing buffer, cursor, selection, folding and virtualized line rendering.
    - Vocabulary: GRBL G/M code vocabulary for versions 1.0, 1.1 and 1.2 (src/gcodeedit/vocabulary.rs) used for highlighting, completion and validation.
    - Validation Rules: configurable RuleSet (src/gcodeedit/rules.rs) allowing enable/disable of rules, per-rule severity, and GRBL-version-specific checks.
    - Parser/Tokenizer: incremental tokenizer + lightweight parser service for real-time syntax and semantic validation (debounced background task).
    - Editor Features: line numbers, gutter diagnostics, find/replace, code folding, auto-completion for G/M codes and parameters, keyboard shortcuts, undo/redo.
    - Integration: editor <-> visualizer line mapping for back-plot stepping and line highlighting; APIs for completions and diagnostics.
    - Performance: incremental parsing and token caching to keep validation and highlighting responsive (<100ms) for files 1000+ lines.
    - Tests & Docs: unit tests for tokenizer, rules, and editor buffer; documentation and DOCBLOCKs per project standards.
38. **Back Plotting**: Visual simulation of G-code execution to verify toolpaths before machining

40. **Speeds and Feeds Calculator**: Built-in calculator for optimizing cutting parameters based on material and tool
41. **Bitmap Processing**: Import bitmaps for heightmap generation, edge detection, and vectorization

43. **Region Fill**: Automatic filling of enclosed areas for machining
44. **Part Nesting**: Automatic part nesting for efficient material usage ✓
45. **3D Profiling**: Waterline and scanline machining for 3D surfaces, back-face machining
46. **Lathe Operations**: Turning operations for cylindrical parts (facing, grooving, threading)
47. **Holding Tabs**: Automatic generation of tabs to hold parts during machining
48. **Lead Moves**: Configurable lead-in and lead-out moves to reduce tool wear
49. **Side Profiles**: Machining vertical faces or sides of parts
50. **Advanced CAD Operations**: Boolean operations (union ✓/intersection/difference), polyline/surface editing
51. **CAM Part Management**: Organize multiple machining operations into parts for batch processing
52. **Automation Scripting**: Scripting support for batch processing and workflow automation

LaserGRBL-Inspired Features:
53. **Image Engraving Enhancements**: Grayscale conversion, dithering algorithms, and vectorization for laser engraving
54. **Power and Speed Overrides**: Real-time adjustment of laser power and feed speed during operation
55. **User-Defined Buttons**: Customizable macro buttons for frequently used commands
56. **Configuration Management**: Import/export of GRBL settings and machine profiles

References and competative tools:

1. The existing application called "Candle" written in C++ can be found at: https://github.com/Denvi/Candle
2. The firmware for the GRBL controller which interprets the gcode used on the devices: https://github.com/grbl/grbl
3. A similar app to Candle written in Java = Universal Gcode Sender: https://github.com/winder/Universal-G-Code-Sender
4. Cambam a tool written in C# for managing CNC devices: http://www.cambam.info/doc/1.0/

6. LightBurn Laser Engraver control - https://docs.lightburnsoftware.com/legacy/pdf/document.pdf
7. LaserGRBL Laser Engraver Control - https://lasergrbl.com/usage/
8. TinkerCad simple design modelling - https://skills4am.eu/documents/tinkercad_usermanual.pdf


General Instructions:

When reading PDF or Word files convert the files first to markdown before processing them.



