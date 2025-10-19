//! Overrides widget for real-time control adjustments

/// Overrides widget for spindle/laser and feed rate adjustments
pub struct OverridesWidget {
    pub spindle_power: f64,  // 0.0 to 1.0
    pub feed_rate: f64,      // 0.0 to 2.0 (percentage)
}

impl OverridesWidget {
    /// Create a new overrides widget
    pub fn new() -> Self {
        Self {
            spindle_power: 1.0,
            feed_rate: 1.0,
        }
    }

    /// Set spindle/laser power (0.0 to 1.0)
    pub fn set_spindle_power(&mut self, power: f64) {
        self.spindle_power = power.clamp(0.0, 1.0);
    }

    /// Set feed rate override (0.0 to 2.0)
    pub fn set_feed_rate(&mut self, rate: f64) {
        self.feed_rate = rate.clamp(0.0, 2.0);
    }

    /// Increase feed rate by 10%
    pub fn increase_feed_rate(&mut self) {
        self.set_feed_rate(self.feed_rate + 0.1);
    }

    /// Decrease feed rate by 10%
    pub fn decrease_feed_rate(&mut self) {
        self.set_feed_rate(self.feed_rate - 0.1);
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
        assert_eq!(widget.spindle_power, 1.0);
        assert_eq!(widget.feed_rate, 1.0);
    }

    #[test]
    fn test_spindle_power_clamping() {
        let mut widget = OverridesWidget::new();
        widget.set_spindle_power(1.5);
        assert_eq!(widget.spindle_power, 1.0);
        widget.set_spindle_power(-0.5);
        assert_eq!(widget.spindle_power, 0.0);
    }

    #[test]
    fn test_feed_rate_adjustment() {
        let mut widget = OverridesWidget::new();
        widget.increase_feed_rate();
        assert_eq!(widget.feed_rate, 1.1);
        widget.decrease_feed_rate();
        assert_eq!(widget.feed_rate, 1.0);
    }
}
