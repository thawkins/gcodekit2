//! GCodeKit Integration Tests
//!
//! Test suite organized by module hierarchy:
//! - tests/communication/ - GRBL protocol and serial communication
//! - tests/designer/ - CAM functions, shapes, and toolpath generation
//! - tests/jobs/ - Job scheduling and queue management
//! - tests/materials/ - Material database operations
//! - tests/widgets/ - UI widget tests

mod communication;
mod designer;
mod jobs;
mod materials;
mod widgets;
