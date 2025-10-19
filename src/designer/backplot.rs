//! Back-plotting module for visual G-code simulation.
//!
//! Provides step-through execution of G-code with real-time position tracking,
//! visualization support, and full simulation control (forward, backward, jump, pause/resume).

use anyhow::{anyhow, Result};
use std::collections::VecDeque;

/// Represents a single step in G-code execution
#[derive(Clone, Debug)]
pub struct BackPlotStep {
    /// G-code line number
    pub line_number: usize,
    /// Starting position (X, Y, Z)
    pub start_pos: [f32; 3],
    /// Ending position (X, Y, Z)
    pub end_pos: [f32; 3],
    /// G-code command (G0, G1, G2, G3, etc.)
    pub gcode_command: String,
    /// Feed rate in mm/min (0 for rapid moves)
    pub feed_rate: f32,
    /// Spindle/laser speed in RPM or power percentage
    pub spindle_speed: f32,
    /// Move type indicator
    pub move_type: MoveType,
}

/// Type of G-code move
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum MoveType {
    /// Rapid move (G0)
    Rapid,
    /// Linear feed move (G1)
    Linear,
    /// Clockwise arc (G2)
    ArcCW,
    /// Counter-clockwise arc (G3)
    ArcCCW,
    /// Dwell/pause (G4)
    Dwell,
    /// Other command
    Other,
}

/// Simulation state
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum BackPlotState {
    /// Simulation not started
    Idle,
    /// Simulation running
    Running,
    /// Simulation paused
    Paused,
    /// Simulation completed
    Completed,
}

/// Back-plotter for G-code visualization
#[derive(Clone, Debug)]
pub struct BackPlotter {
    /// All parsed steps
    steps: Vec<BackPlotStep>,
    /// Current step index
    current_step: usize,
    /// Simulation state
    state: BackPlotState,
    /// Step history for undo capability
    history: VecDeque<usize>,
    /// Maximum history size
    max_history: usize,
    /// Current machine position
    current_pos: [f32; 3],
}

impl BackPlotter {
    /// Create a new BackPlotter with given steps
    ///
    /// # Arguments
    /// * `steps` - Vector of BackPlotStep representing the G-code program
    ///
    /// # Returns
    /// New BackPlotter instance initialized to step 0
    pub fn new(steps: Vec<BackPlotStep>) -> Result<Self> {
        if steps.is_empty() {
            return Err(anyhow!("Cannot create BackPlotter with empty steps"));
        }

        let initial_pos = if steps.is_empty() {
            [0.0; 3]
        } else {
            steps[0].start_pos
        };

        Ok(Self {
            steps,
            current_step: 0,
            state: BackPlotState::Idle,
            history: VecDeque::new(),
            max_history: 1000,
            current_pos: initial_pos,
        })
    }

    /// Step forward one command
    ///
    /// # Returns
    /// Reference to the next step if available
    pub fn step_forward(&mut self) -> Option<&BackPlotStep> {
        if self.current_step >= self.steps.len() {
            self.state = BackPlotState::Completed;
            return None;
        }

        let step = &self.steps[self.current_step];
        self.current_pos = step.end_pos;
        self.history.push_back(self.current_step);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
        
        self.current_step += 1;
        
        // Check if we've reached the end
        if self.current_step >= self.steps.len() {
            self.state = BackPlotState::Completed;
        } else {
            self.state = BackPlotState::Running;
        }

        self.steps.get(self.current_step - 1)
    }

    /// Step backward one command
    ///
    /// # Returns
    /// Reference to the previous step if available
    pub fn step_backward(&mut self) -> Option<&BackPlotStep> {
        if self.current_step == 0 {
            self.state = BackPlotState::Idle;
            return None;
        }

        self.current_step -= 1;
        let step = &self.steps[self.current_step];
        self.current_pos = step.start_pos;

        // Don't use history for backward - just decrement step
        self.steps.get(self.current_step)
    }

    /// Jump to a specific step number
    ///
    /// # Arguments
    /// * `step_number` - Target step number (0-based)
    ///
    /// # Returns
    /// Reference to the target step if valid
    pub fn jump_to_step(&mut self, step_number: usize) -> Result<&BackPlotStep> {
        if step_number >= self.steps.len() {
            return Err(anyhow!(
                "Step {} out of range (max {})",
                step_number,
                self.steps.len() - 1
            ));
        }

        // Calculate position at the end of the step before the target
        let mut current_pos = self.steps[0].start_pos;
        for i in 0..step_number {
            current_pos = self.steps[i].end_pos;
        }

        self.current_step = step_number;
        self.current_pos = current_pos;
        self.state = BackPlotState::Running;

        Ok(&self.steps[step_number])
    }

    /// Pause the simulation
    pub fn pause(&mut self) {
        if self.state == BackPlotState::Running {
            self.state = BackPlotState::Paused;
        }
    }

    /// Resume the simulation
    pub fn resume(&mut self) {
        if self.state == BackPlotState::Paused {
            self.state = BackPlotState::Running;
        }
    }

    /// Stop and reset simulation to beginning
    pub fn stop(&mut self) {
        self.current_step = 0;
        self.current_pos = self.steps[0].start_pos;
        self.state = BackPlotState::Idle;
        self.history.clear();
    }

    /// Get current simulation state
    pub fn get_state(&self) -> BackPlotState {
        self.state
    }

    /// Get current position
    pub fn get_position(&self) -> [f32; 3] {
        self.current_pos
    }

    /// Get current step index
    pub fn get_current_step(&self) -> usize {
        self.current_step
    }

    /// Get total number of steps
    pub fn get_total_steps(&self) -> usize {
        self.steps.len()
    }

    /// Get progress as percentage (0-100)
    pub fn get_progress(&self) -> f32 {
        if self.steps.is_empty() {
            100.0
        } else {
            (self.current_step as f32 / self.steps.len() as f32) * 100.0
        }
    }

    /// Get reference to current step
    pub fn get_current_step_ref(&self) -> Option<&BackPlotStep> {
        if self.current_step == 0 {
            return None;
        }
        self.steps.get(self.current_step - 1)
    }

    /// Get reference to all steps
    pub fn get_steps(&self) -> &[BackPlotStep] {
        &self.steps
    }

    /// Get reference to specific step
    pub fn get_step(&self, index: usize) -> Option<&BackPlotStep> {
        self.steps.get(index)
    }

    /// Reset to beginning
    pub fn reset(&mut self) {
        self.current_step = 0;
        self.current_pos = self.steps[0].start_pos;
        self.state = BackPlotState::Idle;
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_steps() -> Vec<BackPlotStep> {
        vec![
            BackPlotStep {
                line_number: 1,
                start_pos: [0.0, 0.0, 0.0],
                end_pos: [10.0, 0.0, 0.0],
                gcode_command: "G0 X10".to_string(),
                feed_rate: 0.0,
                spindle_speed: 0.0,
                move_type: MoveType::Rapid,
            },
            BackPlotStep {
                line_number: 2,
                start_pos: [10.0, 0.0, 0.0],
                end_pos: [10.0, 10.0, 0.0],
                gcode_command: "G1 Y10 F1000".to_string(),
                feed_rate: 1000.0,
                spindle_speed: 5000.0,
                move_type: MoveType::Linear,
            },
            BackPlotStep {
                line_number: 3,
                start_pos: [10.0, 10.0, 0.0],
                end_pos: [0.0, 0.0, 0.0],
                gcode_command: "G1 X0 Y0".to_string(),
                feed_rate: 1000.0,
                spindle_speed: 5000.0,
                move_type: MoveType::Linear,
            },
        ]
    }

    #[test]
    fn test_backplotter_creation() {
        let steps = create_test_steps();
        let bp = BackPlotter::new(steps).unwrap();
        assert_eq!(bp.get_current_step(), 0);
        assert_eq!(bp.get_state(), BackPlotState::Idle);
        assert_eq!(bp.get_total_steps(), 3);
    }

    #[test]
    fn test_step_forward() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        let step = bp.step_forward().unwrap();
        assert_eq!(step.line_number, 1);
        assert_eq!(bp.get_current_step(), 1);
        assert_eq!(bp.get_state(), BackPlotState::Running);
        assert_eq!(bp.get_position(), [10.0, 0.0, 0.0]);
    }

    #[test]
    fn test_step_backward() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        let prev = bp.step_backward();
        assert!(prev.is_some());
        assert_eq!(bp.get_current_step(), 1);
    }

    #[test]
    fn test_multiple_steps() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        assert_eq!(bp.get_current_step(), 3);
        assert_eq!(bp.get_state(), BackPlotState::Completed);
    }

    #[test]
    fn test_jump_to_step() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        let step = bp.jump_to_step(2).unwrap();
        assert_eq!(step.line_number, 3);
        assert_eq!(bp.get_current_step(), 2);
    }

    #[test]
    fn test_jump_invalid_step() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        let result = bp.jump_to_step(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_pause_resume() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        assert_eq!(bp.get_state(), BackPlotState::Running);
        bp.pause();
        assert_eq!(bp.get_state(), BackPlotState::Paused);
        bp.resume();
        assert_eq!(bp.get_state(), BackPlotState::Running);
    }

    #[test]
    fn test_progress_tracking() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        assert_eq!(bp.get_progress(), 0.0);
        bp.step_forward().unwrap();
        let expected = (1.0 / 3.0) * 100.0;
        assert!((bp.get_progress() - expected).abs() < 0.1);
    }

    #[test]
    fn test_stop_and_reset() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        bp.stop();
        assert_eq!(bp.get_current_step(), 0);
        assert_eq!(bp.get_state(), BackPlotState::Idle);
    }

    #[test]
    fn test_get_current_step_ref() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        let current = bp.get_current_step_ref().unwrap();
        assert_eq!(current.line_number, 1);
    }

    #[test]
    fn test_backward_at_start() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        let result = bp.step_backward();
        assert!(result.is_none());
        assert_eq!(bp.get_state(), BackPlotState::Idle);
    }

    #[test]
    fn test_forward_at_end() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        let result = bp.step_forward();
        assert!(result.is_none());
    }

    #[test]
    fn test_position_tracking() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        assert_eq!(bp.get_position(), [0.0, 0.0, 0.0]);
        bp.step_forward().unwrap();
        assert_eq!(bp.get_position(), [10.0, 0.0, 0.0]);
        bp.step_forward().unwrap();
        assert_eq!(bp.get_position(), [10.0, 10.0, 0.0]);
    }

    #[test]
    fn test_get_steps() {
        let steps = create_test_steps();
        let bp = BackPlotter::new(steps.clone()).unwrap();

        let all_steps = bp.get_steps();
        assert_eq!(all_steps.len(), 3);
        assert_eq!(all_steps[0].line_number, 1);
    }

    #[test]
    fn test_get_specific_step() {
        let steps = create_test_steps();
        let bp = BackPlotter::new(steps).unwrap();

        let step = bp.get_step(1).unwrap();
        assert_eq!(step.line_number, 2);

        let invalid = bp.get_step(10);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_reset() {
        let steps = create_test_steps();
        let mut bp = BackPlotter::new(steps).unwrap();

        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        bp.reset();
        assert_eq!(bp.get_current_step(), 0);
        assert_eq!(bp.get_position(), [0.0, 0.0, 0.0]);
        assert_eq!(bp.get_state(), BackPlotState::Idle);
    }

    #[test]
    fn test_empty_steps_error() {
        let result = BackPlotter::new(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_move_types() {
        let steps = vec![
            BackPlotStep {
                line_number: 1,
                start_pos: [0.0, 0.0, 0.0],
                end_pos: [10.0, 0.0, 0.0],
                gcode_command: "G0 X10".to_string(),
                feed_rate: 0.0,
                spindle_speed: 0.0,
                move_type: MoveType::Rapid,
            },
            BackPlotStep {
                line_number: 2,
                start_pos: [10.0, 0.0, 0.0],
                end_pos: [10.0, 10.0, 0.0],
                gcode_command: "G1 Y10".to_string(),
                feed_rate: 1000.0,
                spindle_speed: 5000.0,
                move_type: MoveType::Linear,
            },
            BackPlotStep {
                line_number: 3,
                start_pos: [10.0, 10.0, 0.0],
                end_pos: [15.0, 15.0, 0.0],
                gcode_command: "G2 X15 Y15".to_string(),
                feed_rate: 1000.0,
                spindle_speed: 5000.0,
                move_type: MoveType::ArcCW,
            },
        ];

        let mut bp = BackPlotter::new(steps).unwrap();
        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        bp.step_forward().unwrap();
        let arc_step = bp.get_current_step_ref().unwrap();
        assert_eq!(arc_step.move_type, MoveType::ArcCW);
    }
}
