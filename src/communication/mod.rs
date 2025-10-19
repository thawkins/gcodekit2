//! GRBL communication module
//!
//! Handles serial communication with GRBL firmware, including connection
//! management, command sending, response parsing, and status monitoring.

pub mod grbl;

pub use grbl::GrblController;
