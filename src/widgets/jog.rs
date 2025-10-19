//! Jog Widget - Real-time axis control (X/Y/Z)

use serde::{Deserialize, Serialize};
use tracing::info;

/// Step sizes for jogging (in mm)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum JogStepSize {
    Small,   // 0.1 mm
    Normal,  // 1.0 mm
    Large,   // 10 mm
    Huge,    // 50 mm
}

impl JogStepSize {
    /// Get numeric value
    pub fn value(&self) -> f64 {
        match self {
            JogStepSize::Small => 0.1,
            JogStepSize::Normal => 1.0,
            JogStepSize::Large => 10.0,
            JogStepSize::Huge => 50.0,
        }
    }

    /// Get as string
    pub fn as_str(&self) -> &'static str {
        match self {
            JogStepSize::Small => "0.1mm",
            JogStepSize::Normal => "1.0mm",
            JogStepSize::Large => "10mm",
            JogStepSize::Huge => "50mm",
        }
    }
}

/// Jog widget state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JogWidget {
    pub step_size: JogStepSize,
    pub last_x_move: f64,
    pub last_y_move: f64,
    pub last_z_move: f64,
    pub current_command: Option<String>,
}

impl JogWidget {
    /// Create a new jog widget
    pub fn new() -> Self {
        JogWidget {
            step_size: JogStepSize::Normal,
            last_x_move: 0.0,
            last_y_move: 0.0,
            last_z_move: 0.0,
            current_command: None,
        }
    }

    /// Jog X axis positive
    pub fn jog_x_positive(&mut self) -> String {
        let step = self.step_size.value();
        self.last_x_move = step;
        let cmd = format!("$J=G91 G21 X{:.2} F600", step);
        self.current_command = Some(cmd.clone());
        info!("Jog X+: {}", cmd);
        cmd
    }

    /// Jog X axis negative
    pub fn jog_x_negative(&mut self) -> String {
        let step = -self.step_size.value();
        self.last_x_move = step;
        let cmd = format!("$J=G91 G21 X{:.2} F600", step);
        self.current_command = Some(cmd.clone());
        info!("Jog X-: {}", cmd);
        cmd
    }

    /// Jog Y axis positive
    pub fn jog_y_positive(&mut self) -> String {
        let step = self.step_size.value();
        self.last_y_move = step;
        let cmd = format!("$J=G91 G21 Y{:.2} F600", step);
        self.current_command = Some(cmd.clone());
        info!("Jog Y+: {}", cmd);
        cmd
    }

    /// Jog Y axis negative
    pub fn jog_y_negative(&mut self) -> String {
        let step = -self.step_size.value();
        self.last_y_move = step;
        let cmd = format!("$J=G91 G21 Y{:.2} F600", step);
        self.current_command = Some(cmd.clone());
        info!("Jog Y-: {}", cmd);
        cmd
    }

    /// Jog Z axis positive
    pub fn jog_z_positive(&mut self) -> String {
        let step = self.step_size.value();
        self.last_z_move = step;
        let cmd = format!("$J=G91 G21 Z{:.2} F300", step);
        self.current_command = Some(cmd.clone());
        info!("Jog Z+: {}", cmd);
        cmd
    }

    /// Jog Z axis negative
    pub fn jog_z_negative(&mut self) -> String {
        let step = -self.step_size.value();
        self.last_z_move = step;
        let cmd = format!("$J=G91 G21 Z{:.2} F300", step);
        self.current_command = Some(cmd.clone());
        info!("Jog Z-: {}", cmd);
        cmd
    }

    /// Set step size
    pub fn set_step_size(&mut self, size: JogStepSize) {
        self.step_size = size;
        info!("Jog step size set to: {}", size.as_str());
    }

    /// Get available step sizes
    pub fn step_sizes() -> Vec<JogStepSize> {
        vec![
            JogStepSize::Small,
            JogStepSize::Normal,
            JogStepSize::Large,
            JogStepSize::Huge,
        ]
    }

    /// Unlock machine (alarm reset)
    pub fn unlock(&self) -> String {
        "$X".to_string()
    }

    /// Resume from hold
    pub fn resume(&self) -> String {
        "~".to_string()
    }
}

impl Default for JogWidget {
    fn default() -> Self {
        Self::new()
    }
}
