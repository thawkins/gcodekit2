//! UI widgets module
//!
//! Contains all modular UI widgets for machine control and CAM functions.

pub mod connection;
pub mod gcode_loading;
pub mod jog;
pub mod overrides;

pub use connection::ConnectionWidget;
pub use jog::JogWidget;
