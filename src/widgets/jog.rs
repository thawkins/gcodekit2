//! Jog widget for real-time axis control

/// Step size options for jogging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepSize {
    ZeroPointOne,
    One,
    Ten,
    Fifty,
}

impl StepSize {
    /// Get the step size in mm
    pub fn value(&self) -> f64 {
        match self {
            Self::ZeroPointOne => 0.1,
            Self::One => 1.0,
            Self::Ten => 10.0,
            Self::Fifty => 50.0,
        }
    }
}

impl Default for StepSize {
    fn default() -> Self {
        Self::One
    }
}

/// Jog widget for axis control
pub struct JogWidget {
    pub current_step: StepSize,
    pub x_position: f64,
    pub y_position: f64,
    pub z_position: f64,
}

impl JogWidget {
    /// Create a new jog widget
    pub fn new() -> Self {
        Self {
            current_step: StepSize::default(),
            x_position: 0.0,
            y_position: 0.0,
            z_position: 0.0,
        }
    }

    /// Jog X axis
    pub fn jog_x(&mut self, direction: i32) {
        let delta = self.current_step.value() * direction as f64;
        self.x_position += delta;
    }

    /// Jog Y axis
    pub fn jog_y(&mut self, direction: i32) {
        let delta = self.current_step.value() * direction as f64;
        self.y_position += delta;
    }

    /// Jog Z axis
    pub fn jog_z(&mut self, direction: i32) {
        let delta = self.current_step.value() * direction as f64;
        self.z_position += delta;
    }

    /// Set step size
    pub fn set_step_size(&mut self, step: StepSize) {
        self.current_step = step;
    }
}

impl Default for JogWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_size_values() {
        assert_eq!(StepSize::ZeroPointOne.value(), 0.1);
        assert_eq!(StepSize::One.value(), 1.0);
        assert_eq!(StepSize::Ten.value(), 10.0);
        assert_eq!(StepSize::Fifty.value(), 50.0);
    }

    #[test]
    fn test_jog_widget_creation() {
        let widget = JogWidget::new();
        assert_eq!(widget.x_position, 0.0);
        assert_eq!(widget.current_step, StepSize::One);
    }

    #[test]
    fn test_jog_x_positive() {
        let mut widget = JogWidget::new();
        widget.jog_x(1);
        assert_eq!(widget.x_position, 1.0);
    }

    #[test]
    fn test_jog_x_negative() {
        let mut widget = JogWidget::new();
        widget.jog_x(-1);
        assert_eq!(widget.x_position, -1.0);
    }
}
