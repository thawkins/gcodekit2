//! Communication module tests

use gcodekit2::communication::{GrblController, MachineState, Position};

#[tokio::test]
async fn test_grbl_controller_creation() {
    let controller = GrblController::new();
    assert!(!controller.is_connected().await);
}

#[test]
fn test_machine_state_parsing() {
    assert_eq!(MachineState::from_str("IDLE"), MachineState::Idle);
    assert_eq!(MachineState::from_str("run"), MachineState::Run);
    assert_eq!(MachineState::from_str("ALARM"), MachineState::Alarm);
    assert_eq!(MachineState::from_str("Hold"), MachineState::Hold);
}

#[test]
fn test_position_creation() {
    let pos = Position { x: 10.5, y: 20.3, z: 5.1 };
    assert_eq!(pos.x, 10.5);
    assert_eq!(pos.y, 20.3);
    assert_eq!(pos.z, 5.1);
}

#[tokio::test]
async fn test_version_detection() {
    let controller = GrblController::new();
    let version = controller.detect_version().await.unwrap();
    assert_eq!(version, "GRBL v1.1h");
}

#[tokio::test]
async fn test_status_update() {
    let controller = GrblController::new();
    let mpos = Position { x: 1.0, y: 2.0, z: 3.0 };
    let wpos = Position { x: 0.0, y: 0.0, z: 0.0 };
    controller
        .update_status(MachineState::Idle, mpos, wpos, 1000, 0)
        .await;
    let status = controller.get_status().await.unwrap();
    assert_eq!(status.state, MachineState::Idle);
    assert_eq!(status.mpos.x, 1.0);
}

#[tokio::test]
async fn test_command_queue() {
    let controller = GrblController::new();
    controller.send_command("G0 X10 Y10").await.unwrap();
    controller.send_command("G1 Z-5 F100").await.unwrap();
    
    let cmd1 = controller.get_next_command().await;
    assert_eq!(cmd1, Some("G0 X10 Y10".to_string()));
    
    let cmd2 = controller.get_next_command().await;
    assert_eq!(cmd2, Some("G1 Z-5 F100".to_string()));
}

#[tokio::test]
async fn test_response_logging() {
    let controller = GrblController::new();
    controller.log_response("ok".to_string()).await;
    controller.log_response("error: Unknown command".to_string()).await;
    
    let log = controller.get_response_log().await;
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "ok");
}

#[tokio::test]
async fn test_recovery_config() {
    use gcodekit2::communication::RecoveryConfig;
    let controller = GrblController::new();
    let config = RecoveryConfig {
        max_retries: 5,
        retry_delay_ms: 1000,
        auto_reconnect: true,
        reconnect_delay_ms: 3000,
    };
    controller.set_recovery_config(config.clone()).await;
    let retrieved = controller.get_recovery_config().await;
    assert_eq!(retrieved.max_retries, 5);
}

#[tokio::test]
async fn test_emergency_stop() {
    let controller = GrblController::new();
    controller.emergency_stop().await.unwrap();
    let status = controller.get_status().await.unwrap();
    assert_eq!(status.state, MachineState::Alarm);
}

#[test]
fn test_machine_state_colors() {
    assert_eq!(MachineState::Idle.color(), "#00AA00");
    assert_eq!(MachineState::Run.color(), "#0000FF");
    assert_eq!(MachineState::Alarm.color(), "#FF0000");
}
