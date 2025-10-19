//! Overrides Widget - Real-time spindle/laser power and feed rate adjustments

use serde::{Deserialize, Serialize};
use tracing::info;

/// Overrides widget state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverridesWidget {
    pub feed_rate_override: u32,      // 0-200%
    pub spindle_power_override: u32,  // 0-100%
    pub laser_mode: bool,             // laser vs spindle
}

impl OverridesWidget {
    /// Create a new overrides widget
    pub fn new() -> Self {
        OverridesWidget {
            feed_rate_override: 100,
            spindle_power_override: 100,
            laser_mode: false,
        }
    }

    /// Adjust feed rate override
    pub fn set_feed_rate(&mut self, percentage: u32) {
        self.feed_rate_override = percentage.clamp(50, 200);
        info!("Feed rate override set to: {}%", self.feed_rate_override);
    }

    /// Increase feed rate by 10%
    pub fn increase_feed_rate(&mut self) {
        self.set_feed_rate(self.feed_rate_override + 10);
    }

    /// Decrease feed rate by 10%
    pub fn decrease_feed_rate(&mut self) {
        self.set_feed_rate(self.feed_rate_override.saturating_sub(10));
    }

    /// Set spindle/laser power override
    pub fn set_spindle_power(&mut self, percentage: u32) {
        self.spindle_power_override = percentage.clamp(0, 100);
        info!("Spindle power override set to: {}%", self.spindle_power_override);
    }

    /// Increase spindle power by 5%
    pub fn increase_spindle_power(&mut self) {
        self.set_spindle_power(self.spindle_power_override + 5);
    }

    /// Decrease spindle power by 5%
    pub fn decrease_spindle_power(&mut self) {
        self.set_spindle_power(self.spindle_power_override.saturating_sub(5));
    }

    /// Toggle laser mode
    pub fn toggle_laser_mode(&mut self) {
        self.laser_mode = !self.laser_mode;
        info!("Laser mode: {}", self.laser_mode);
    }

    /// Get GRBL override command for feed rate
    pub fn get_feed_rate_command(&self) -> String {
        match self.feed_rate_override {
            150..=200 => "0x91".to_string(), // Feed rate + 10%
            100..=149 => "0x90".to_string(), // Normal
            _ => "0x8F".to_string(),         // Feed rate - 10%
        }
    }

    /// Get GRBL override command for spindle power
    pub fn get_spindle_power_command(&self) -> String {
        match self.spindle_power_override {
            76..=100 => "0x99".to_string(), // Spindle + 10%
            51..=75 => "0x98".to_string(),  // Normal
            _ => "0x97".to_string(),        // Spindle - 10%
        }
    }

    /// Get description of current state
    pub fn get_status(&self) -> String {
        format!(
            "Feed: {}% | Power: {}% | Mode: {}",
            self.feed_rate_override,
            self.spindle_power_override,
            if self.laser_mode { "Laser" } else { "Spindle" }
        )
    }
}

impl Default for OverridesWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overrides_creation() {
        let widget = OverridesWidget::new();
        assert_eq!(widget.feed_rate_override, 100);
        assert_eq!(widget.spindle_power_override, 100);
        assert!(!widget.laser_mode);
    }

    #[test]
    fn test_feed_rate_adjustment() {
        let mut widget = OverridesWidget::new();
        widget.set_feed_rate(150);
        assert_eq!(widget.feed_rate_override, 150);
    }

    #[test]
    fn test_feed_rate_clamping() {
        let mut widget = OverridesWidget::new();
        widget.set_feed_rate(250);
        assert_eq!(widget.feed_rate_override, 200);
        
        widget.set_feed_rate(10);
        assert_eq!(widget.feed_rate_override, 50);
    }

    #[test]
    fn test_spindle_power_adjustment() {
        let mut widget = OverridesWidget::new();
        widget.set_spindle_power(75);
        assert_eq!(widget.spindle_power_override, 75);
    }

    #[test]
    fn test_spindle_power_clamping() {
        let mut widget = OverridesWidget::new();
        widget.set_spindle_power(150);
        assert_eq!(widget.spindle_power_override, 100);
        
        widget.set_spindle_power(0);
        assert_eq!(widget.spindle_power_override, 0);
    }

    #[test]
    fn test_increase_feed_rate() {
        let mut widget = OverridesWidget::new();
        widget.increase_feed_rate();
        assert_eq!(widget.feed_rate_override, 110);
    }

    #[test]
    fn test_decrease_feed_rate() {
        let mut widget = OverridesWidget::new();
        widget.feed_rate_override = 120;
        widget.decrease_feed_rate();
        assert_eq!(widget.feed_rate_override, 110);
    }

    #[test]
    fn test_toggle_laser_mode() {
        let mut widget = OverridesWidget::new();
        assert!(!widget.laser_mode);
        widget.toggle_laser_mode();
        assert!(widget.laser_mode);
    }

    #[test]
    fn test_status_description() {
        let widget = OverridesWidget::new();
        let status = widget.get_status();
        assert!(status.contains("Feed: 100%"));
        assert!(status.contains("Power: 100%"));
    }
}
