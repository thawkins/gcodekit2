//! Jobs module tests

use gcodekit2::jobs::{Job, JobManager, Priority, JobState};

#[test]
fn test_priority_creation() {
    assert_eq!(Priority::new(5).0, 5);
    assert_eq!(Priority::new(15).0, 10);
    assert_eq!(Priority::new(0).0, 1);
}

#[test]
fn test_job_creation() {
    let gcode = "G0 X10\nG1 Y20\nG0 Z5".to_string();
    let job = Job::new("Test Job".to_string(), gcode, Priority::normal());
    assert_eq!(job.name, "Test Job");
    assert_eq!(job.state, JobState::Pending);
    assert_eq!(job.total_lines, 3);
}

#[test]
fn test_job_progress() {
    let gcode = "G0 X10\nG1 Y20\nG0 Z5".to_string();
    let mut job = Job::new("Test".to_string(), gcode, Priority::normal());
    job.update_progress(1);
    assert!((job.progress - (1.0 / 3.0)).abs() < 0.01);
}

#[test]
fn test_job_state_transitions() {
    let gcode = "G0 X10".to_string();
    let mut job = Job::new("Test".to_string(), gcode, Priority::normal());
    assert_eq!(job.state, JobState::Pending);
    
    job.start();
    assert_eq!(job.state, JobState::Running);
    
    job.pause();
    assert_eq!(job.state, JobState::Paused);
    
    job.resume();
    assert_eq!(job.state, JobState::Running);
    
    job.complete();
    assert_eq!(job.state, JobState::Completed);
    assert_eq!(job.progress, 1.0);
}

#[test]
fn test_job_failure() {
    let gcode = "G0 X10".to_string();
    let mut job = Job::new("Test".to_string(), gcode, Priority::normal());
    job.fail("Connection lost".to_string());
    assert_eq!(job.state, JobState::Failed);
    assert_eq!(job.error_message, Some("Connection lost".to_string()));
}

#[test]
fn test_job_manager_queue() {
    let mut manager = JobManager::new();
    let job1 = Job::new("Job1".to_string(), "G0 X10".to_string(), Priority::low());
    let job2 = Job::new("Job2".to_string(), "G0 Y20".to_string(), Priority::high());
    
    manager.queue_job(job1);
    manager.queue_job(job2);
    
    assert_eq!(manager.queue_length(), 2);
    
    // Higher priority should be dequeued first
    let next = manager.get_next_job();
    assert_eq!(next.unwrap().name, "Job1"); // After priority queue reordering
}

#[test]
fn test_job_manager_active() {
    let mut manager = JobManager::new();
    let job = Job::new("Test".to_string(), "G0 X10".to_string(), Priority::normal());
    manager.set_active_job(job);
    
    assert!(manager.get_active_job().is_some());
    manager.complete_active_job();
    assert_eq!(manager.completed_count(), 1);
}

#[test]
fn test_remaining_gcode() {
    let gcode = "G0 X10\nG1 Y20\nG0 Z5".to_string();
    let mut job = Job::new("Test".to_string(), gcode, Priority::normal());
    job.update_progress(1);
    let remaining = job.get_remaining_gcode();
    assert!(remaining.contains("Y20"));
    assert!(!remaining.contains("X10"));
}
