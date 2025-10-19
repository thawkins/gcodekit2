//! Back-plotting integration tests

use gcodekit2::designer::{BackPlotStep, BackPlotter, BackPlotState, MoveType};

/// Create test steps for integration testing
fn create_complex_program() -> Vec<BackPlotStep> {
    vec![
        // Rapid move to origin
        BackPlotStep {
            line_number: 1,
            start_pos: [0.0, 0.0, 5.0],
            end_pos: [0.0, 0.0, 0.0],
            gcode_command: "G0 Z0".to_string(),
            feed_rate: 0.0,
            spindle_speed: 0.0,
            move_type: MoveType::Rapid,
        },
        // Rapid move to start position
        BackPlotStep {
            line_number: 2,
            start_pos: [0.0, 0.0, 0.0],
            end_pos: [10.0, 10.0, 0.0],
            gcode_command: "G0 X10 Y10".to_string(),
            feed_rate: 0.0,
            spindle_speed: 0.0,
            move_type: MoveType::Rapid,
        },
        // Linear feed move
        BackPlotStep {
            line_number: 3,
            start_pos: [10.0, 10.0, 0.0],
            end_pos: [20.0, 10.0, 0.0],
            gcode_command: "G1 X20 F1000 S5000".to_string(),
            feed_rate: 1000.0,
            spindle_speed: 5000.0,
            move_type: MoveType::Linear,
        },
        // Clockwise arc
        BackPlotStep {
            line_number: 4,
            start_pos: [20.0, 10.0, 0.0],
            end_pos: [30.0, 20.0, 0.0],
            gcode_command: "G2 X30 Y20 I5 J5".to_string(),
            feed_rate: 1000.0,
            spindle_speed: 5000.0,
            move_type: MoveType::ArcCW,
        },
        // Counter-clockwise arc
        BackPlotStep {
            line_number: 5,
            start_pos: [30.0, 20.0, 0.0],
            end_pos: [20.0, 30.0, 0.0],
            gcode_command: "G3 X20 Y30 I-5 J5".to_string(),
            feed_rate: 1000.0,
            spindle_speed: 5000.0,
            move_type: MoveType::ArcCCW,
        },
        // Linear move back to start
        BackPlotStep {
            line_number: 6,
            start_pos: [20.0, 30.0, 0.0],
            end_pos: [10.0, 10.0, 0.0],
            gcode_command: "G1 X10 Y10 F1000".to_string(),
            feed_rate: 1000.0,
            spindle_speed: 5000.0,
            move_type: MoveType::Linear,
        },
        // Rapid to safe height
        BackPlotStep {
            line_number: 7,
            start_pos: [10.0, 10.0, 0.0],
            end_pos: [10.0, 10.0, 5.0],
            gcode_command: "G0 Z5".to_string(),
            feed_rate: 0.0,
            spindle_speed: 0.0,
            move_type: MoveType::Rapid,
        },
    ]
}

#[test]
fn test_backplotter_full_simulation() {
    let steps = create_complex_program();
    let bp = BackPlotter::new(steps).unwrap();

    assert_eq!(bp.get_total_steps(), 7);
    assert_eq!(bp.get_current_step(), 0);
    assert_eq!(bp.get_state(), BackPlotState::Idle);
    assert_eq!(bp.get_position(), [0.0, 0.0, 5.0]);
}

#[test]
fn test_step_through_entire_program() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    // Step through each command
    for i in 1..=7 {
        let step = bp.step_forward().unwrap();
        assert_eq!(step.line_number, i);
        if i < 7 {
            assert_eq!(bp.get_state(), BackPlotState::Running);
        }
    }

    // Should be completed after stepping through all
    assert_eq!(bp.get_state(), BackPlotState::Completed);
    assert!(bp.step_forward().is_none());
}

#[test]
fn test_backward_navigation() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    // Step forward 3 times
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    assert_eq!(bp.get_current_step(), 3);

    // Step backward twice
    let prev = bp.step_backward();
    assert!(prev.is_some());
    assert_eq!(bp.get_current_step(), 2);

    let prev2 = bp.step_backward();
    assert!(prev2.is_some());
    assert_eq!(bp.get_current_step(), 1);
}

#[test]
fn test_position_tracking_through_program() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    assert_eq!(bp.get_position(), [0.0, 0.0, 5.0]);

    bp.step_forward().unwrap();
    assert_eq!(bp.get_position(), [0.0, 0.0, 0.0]);

    bp.step_forward().unwrap();
    assert_eq!(bp.get_position(), [10.0, 10.0, 0.0]);

    bp.step_forward().unwrap();
    assert_eq!(bp.get_position(), [20.0, 10.0, 0.0]);

    bp.step_forward().unwrap();
    assert_eq!(bp.get_position(), [30.0, 20.0, 0.0]);
}

#[test]
fn test_jump_to_middle() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    let step = bp.jump_to_step(3).unwrap();
    assert_eq!(step.line_number, 4);
    assert_eq!(bp.get_current_step(), 3);
    // Position should be at the end of step 3
    assert_eq!(bp.get_position(), [20.0, 10.0, 0.0]);
}

#[test]
fn test_pause_and_resume_workflow() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    bp.step_forward().unwrap();
    assert_eq!(bp.get_state(), BackPlotState::Running);

    bp.pause();
    assert_eq!(bp.get_state(), BackPlotState::Paused);

    // Can still step forward while paused
    bp.step_forward().unwrap();

    bp.resume();
    assert_eq!(bp.get_state(), BackPlotState::Running);
}

#[test]
fn test_progress_calculation() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    assert_eq!(bp.get_progress(), 0.0);

    for _ in 0..3 {
        bp.step_forward().unwrap();
    }
    let progress = bp.get_progress();
    assert!(progress > 0.0 && progress < 100.0);

    for _ in 0..4 {
        bp.step_forward().unwrap();
    }
    assert_eq!(bp.get_state(), BackPlotState::Completed);
}

#[test]
fn test_move_type_classification() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    // Test rapid move (G0)
    let step1 = bp.step_forward().unwrap();
    assert_eq!(step1.move_type, MoveType::Rapid);

    // Test linear move (G1)
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    let step3 = bp.get_current_step_ref().unwrap();
    assert_eq!(step3.move_type, MoveType::Linear);

    // Test arc CW (G2)
    bp.step_forward().unwrap();
    let step4 = bp.get_current_step_ref().unwrap();
    assert_eq!(step4.move_type, MoveType::ArcCW);

    // Test arc CCW (G3)
    bp.step_forward().unwrap();
    let step5 = bp.get_current_step_ref().unwrap();
    assert_eq!(step5.move_type, MoveType::ArcCCW);
}

#[test]
fn test_speed_and_spindle_tracking() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    // Rapid moves have 0 feed rate
    let step = bp.step_forward().unwrap();
    assert_eq!(step.feed_rate, 0.0);

    // Linear move has feed rate
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    let linear = bp.get_current_step_ref().unwrap();
    assert_eq!(linear.feed_rate, 1000.0);
    assert_eq!(linear.spindle_speed, 5000.0);
}

#[test]
fn test_reset_functionality() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    // Advance simulation
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();

    assert_eq!(bp.get_current_step(), 3);
    assert_ne!(bp.get_position(), [0.0, 0.0, 5.0]);

    // Reset
    bp.reset();

    assert_eq!(bp.get_current_step(), 0);
    assert_eq!(bp.get_position(), [0.0, 0.0, 5.0]);
    assert_eq!(bp.get_state(), BackPlotState::Idle);
}

#[test]
fn test_stop_functionality() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    bp.step_forward().unwrap();
    bp.step_forward().unwrap();

    bp.stop();

    assert_eq!(bp.get_current_step(), 0);
    assert_eq!(bp.get_state(), BackPlotState::Idle);
    assert_eq!(bp.get_position(), [0.0, 0.0, 5.0]);
}

#[test]
fn test_all_steps_accessible() {
    let steps = create_complex_program();
    let bp = BackPlotter::new(steps).unwrap();

    let all_steps = bp.get_steps();
    assert_eq!(all_steps.len(), 7);

    for i in 0..7 {
        let step = bp.get_step(i);
        assert!(step.is_some());
        assert_eq!(step.unwrap().line_number, (i + 1) as usize);
    }

    // Out of bounds access
    assert!(bp.get_step(7).is_none());
    assert!(bp.get_step(100).is_none());
}

#[test]
fn test_complex_navigation_pattern() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    // Step forward to middle
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    bp.step_forward().unwrap();
    assert_eq!(bp.get_current_step(), 3);

    // Jump forward
    bp.jump_to_step(5).unwrap();
    assert_eq!(bp.get_current_step(), 5);

    // Step backward
    bp.step_backward().unwrap();
    assert_eq!(bp.get_current_step(), 4);

    // Jump to start
    bp.jump_to_step(0).unwrap();
    assert_eq!(bp.get_current_step(), 0);
}

#[test]
fn test_gcode_command_preservation() {
    let steps = create_complex_program();
    let mut bp = BackPlotter::new(steps).unwrap();

    bp.step_forward().unwrap();
    let step1 = bp.get_current_step_ref().unwrap();
    assert_eq!(step1.gcode_command, "G0 Z0");

    bp.step_forward().unwrap();
    let step2 = bp.get_current_step_ref().unwrap();
    assert_eq!(step2.gcode_command, "G0 X10 Y10");

    bp.step_forward().unwrap();
    let step3 = bp.get_current_step_ref().unwrap();
    assert_eq!(step3.gcode_command, "G1 X20 F1000 S5000");
}
