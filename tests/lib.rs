//! GCodeKit Integration Tests
//!
//! Test suite organized by module hierarchy:
//! - tests/communication/ - GRBL protocol and serial communication
//! - tests/designer/ - CAM functions, shapes, and toolpath generation
//! - tests/jobs/ - Job scheduling and queue management
//! - tests/materials/ - Material database operations
//! - tests/widgets/ - UI widget tests
//! - tests/theme/ - Theme system, color palettes, and persistence
//! - tests/pendant/ - Web pendant interface, WebSocket, and HTTP server

mod communication;
mod designer;
mod jobs;
mod materials;
mod theme;
mod widgets;
mod pendant;
