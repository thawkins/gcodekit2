//! RESTful API endpoints for web pendant
//!
//! Provides HTTP endpoints for machine control, status queries, and job management.
//!
//! # Endpoints
//! - `GET /api/status` - Machine status (position, state, connection)
//! - `GET /api/jobs` - Active and queued jobs
//! - `POST /api/jog` - Send jog command (X, Y, Z axis)
//! - `POST /api/override` - Adjust feed rate or spindle power
//! - `POST /api/emergency-stop` - Emergency stop
//! - `POST /api/connect` - Connect to device
//! - `POST /api/disconnect` - Disconnect from device

use serde::{Deserialize, Serialize};

/// Machine status response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    /// Connection status (connected/disconnected)
    pub connected: bool,
    /// Machine state (idle/running/alarm/hold/check)
    pub state: String,
    /// Current X position (mm)
    pub pos_x: f64,
    /// Current Y position (mm)
    pub pos_y: f64,
    /// Current Z position (mm)
    pub pos_z: f64,
    /// Current feed rate (mm/min)
    pub feed_rate: f64,
    /// Current spindle speed (RPM)
    pub spindle_speed: u16,
    /// GRBL firmware version
    pub firmware_version: String,
}

impl Default for StatusResponse {
    fn default() -> Self {
        Self {
            connected: false,
            state: "disconnected".to_string(),
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            feed_rate: 0.0,
            spindle_speed: 0,
            firmware_version: String::new(),
        }
    }
}

/// Jog command request
#[derive(Clone, Debug, Deserialize)]
pub struct JogRequest {
    /// Axis to move (X, Y, Z)
    pub axis: String,
    /// Distance in mm (positive/negative)
    pub distance: f64,
    /// Feed rate in mm/min
    pub feed_rate: Option<f64>,
}

/// Override adjustment request
#[derive(Clone, Debug, Deserialize)]
pub struct OverrideRequest {
    /// Override type (feed_rate, spindle_speed, laser_power)
    pub override_type: String,
    /// Percentage adjustment (0-200)
    pub value: u8,
}

/// API error response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code
    pub code: u16,
    /// Error message
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_response_default() {
        let status = StatusResponse::default();
        assert!(!status.connected);
        assert_eq!(status.state, "disconnected");
        assert_eq!(status.pos_x, 0.0);
    }

    #[test]
    fn test_jog_request_creation() {
        let req = JogRequest {
            axis: "X".to_string(),
            distance: 10.0,
            feed_rate: Some(100.0),
        };
        assert_eq!(req.axis, "X");
        assert_eq!(req.distance, 10.0);
        assert_eq!(req.feed_rate, Some(100.0));
    }

    #[test]
    fn test_override_request_creation() {
        let req = OverrideRequest {
            override_type: "feed_rate".to_string(),
            value: 110,
        };
        assert_eq!(req.override_type, "feed_rate");
        assert_eq!(req.value, 110);
    }

    #[test]
    fn test_api_error_creation() {
        let error = ApiError {
            code: 400,
            message: "Invalid request".to_string(),
        };
        assert_eq!(error.code, 400);
    }
}
