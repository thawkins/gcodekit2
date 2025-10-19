//! Job Management Module
//!
//! Provides job queuing, priority scheduling, progress tracking, and automatic
//! resumption capabilities for machining operations.

use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Priority levels for job scheduling (1-10, where 10 is highest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Priority(pub u32);

impl Priority {
    /// Create a priority (1-10 scale)
    pub fn new(level: u32) -> Self {
        Priority(level.clamp(1, 10))
    }

    /// Normal priority (5)
    pub fn normal() -> Self {
        Priority(5)
    }

    /// High priority (8)
    pub fn high() -> Self {
        Priority(8)
    }

    /// Low priority (2)
    pub fn low() -> Self {
        Priority(2)
    }
}

/// Job state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// A machining job with G-code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub gcode: String,
    pub state: JobState,
    pub priority: Priority,
    pub progress: f64,        // 0.0 to 1.0
    pub current_line: usize,  // For resumption
    pub total_lines: usize,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub error_message: Option<String>,
}

impl Job {
    /// Create a new job
    pub fn new(name: String, gcode: String, priority: Priority) -> Self {
        let total_lines = gcode.lines().filter(|l| !l.trim().is_empty() && !l.trim().starts_with(';')).count();
        
        Job {
            id: Uuid::new_v4().to_string(),
            name,
            gcode,
            state: JobState::Pending,
            priority,
            progress: 0.0,
            current_line: 0,
            total_lines,
            created_at: Utc::now().to_rfc3339(),
            started_at: None,
            completed_at: None,
            error_message: None,
        }
    }

    /// Update job progress
    pub fn update_progress(&mut self, lines_completed: usize) {
        self.current_line = lines_completed;
        self.progress = if self.total_lines > 0 {
            (lines_completed as f64) / (self.total_lines as f64)
        } else {
            1.0
        };
    }

    /// Mark job as started
    pub fn start(&mut self) {
        self.state = JobState::Running;
        self.started_at = Some(Utc::now().to_rfc3339());
    }

    /// Mark job as completed
    pub fn complete(&mut self) {
        self.state = JobState::Completed;
        self.completed_at = Some(Utc::now().to_rfc3339());
        self.progress = 1.0;
    }

    /// Mark job as failed
    pub fn fail(&mut self, error: String) {
        self.state = JobState::Failed;
        self.error_message = Some(error);
        self.completed_at = Some(Utc::now().to_rfc3339());
    }

    /// Pause the job
    pub fn pause(&mut self) {
        if self.state == JobState::Running {
            self.state = JobState::Paused;
        }
    }

    /// Resume the job
    pub fn resume(&mut self) {
        if self.state == JobState::Paused {
            self.state = JobState::Running;
        }
    }

    /// Get current G-code line
    pub fn get_current_line(&self) -> Option<&str> {
        self.gcode
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with(';'))
            .nth(self.current_line)
    }

    /// Get remaining G-code
    pub fn get_remaining_gcode(&self) -> String {
        self.gcode
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with(';'))
            .skip(self.current_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Job wrapper for priority queue
#[derive(Clone)]
struct QueuedJob {
    priority: Priority,
    sequence: u64,
    job: Box<Job>,
}

impl Ord for QueuedJob {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note: reverse order for max-heap behavior
        other.priority.cmp(&self.priority)
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

impl PartialOrd for QueuedJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueuedJob {
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

impl Eq for QueuedJob {}

/// Job Manager for handling job queuing and execution
pub struct JobManager {
    queue: BinaryHeap<QueuedJob>,
    sequence: u64,
    active_job: Option<Box<Job>>,
    completed_jobs: Vec<Job>,
}

impl JobManager {
    /// Create a new job manager
    pub fn new() -> Self {
        JobManager {
            queue: BinaryHeap::new(),
            sequence: 0,
            active_job: None,
            completed_jobs: Vec::new(),
        }
    }

    /// Add a job to the queue
    pub fn queue_job(&mut self, job: Job) {
        self.queue.push(QueuedJob {
            priority: job.priority,
            sequence: self.sequence,
            job: Box::new(job),
        });
        self.sequence += 1;
    }

    /// Get the next job from the queue
    pub fn get_next_job(&mut self) -> Option<Job> {
        self.queue.pop().map(|qj| *qj.job)
    }

    /// Set active job
    pub fn set_active_job(&mut self, job: Job) {
        self.active_job = Some(Box::new(job));
    }

    /// Get active job
    pub fn get_active_job(&mut self) -> Option<&mut Job> {
        self.active_job.as_mut().map(|b| &mut **b)
    }

    /// Complete active job
    pub fn complete_active_job(&mut self) {
        if let Some(job) = self.active_job.take() {
            let mut j = *job;
            j.complete();
            self.completed_jobs.push(j);
        }
    }

    /// Fail active job
    pub fn fail_active_job(&mut self, error: String) {
        if let Some(job) = self.active_job.take() {
            let mut j = *job;
            j.fail(error);
            self.completed_jobs.push(j);
        }
    }

    /// Get queue length
    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }

    /// Get completed jobs count
    pub fn completed_count(&self) -> usize {
        self.completed_jobs.len()
    }

    /// Get completed jobs
    pub fn get_completed_jobs(&self) -> &[Job] {
        &self.completed_jobs
    }

    /// Clear completed jobs
    pub fn clear_completed(&mut self) {
        self.completed_jobs.clear();
    }

    /// Resume active job from last saved position
    pub fn resume_active_job(&mut self) -> bool {
        if let Some(job) = self.get_active_job() {
            job.resume();
            true
        } else {
            false
        }
    }
}

impl Default for JobManager {
    fn default() -> Self {
        Self::new()
    }
}
