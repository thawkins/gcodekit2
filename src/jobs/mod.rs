//! Job management system
//!
//! Provides job queuing, priority scheduling, progress tracking,
//! and automatic resumption after errors.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Priority level for job scheduling (1-10, where 10 is highest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct JobPriority(pub u8);

impl JobPriority {
    pub fn new(level: u8) -> Self {
        Self(level.clamp(1, 10))
    }
}

impl Default for JobPriority {
    fn default() -> Self {
        Self(5)
    }
}

/// Represents the state of a job
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
}

/// Represents a single job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub name: String,
    pub gcode: Vec<String>,
    pub priority: JobPriority,
    pub state: JobState,
    pub progress: f64, // 0.0 to 1.0
    pub current_line: usize,
}

impl Job {
    /// Create a new job
    pub fn new(name: String, gcode: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            gcode,
            priority: JobPriority::default(),
            state: JobState::Queued,
            progress: 0.0,
            current_line: 0,
        }
    }

    /// Get the next G-code line to execute
    pub fn next_line(&mut self) -> Option<String> {
        if self.current_line >= self.gcode.len() {
            self.state = JobState::Completed;
            self.progress = 1.0;
            None
        } else {
            let line = self.gcode[self.current_line].clone();
            self.current_line += 1;
            self.progress = self.current_line as f64 / self.gcode.len() as f64;
            Some(line)
        }
    }

    /// Resume from a specific line
    pub fn resume_from(&mut self, line_number: usize) {
        self.current_line = line_number.min(self.gcode.len());
        self.state = JobState::Running;
        self.progress = self.current_line as f64 / self.gcode.len() as f64;
    }
}

/// Job queue manager
pub struct JobQueue {
    jobs: Vec<Job>,
}

impl JobQueue {
    /// Create a new job queue
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    /// Add a job to the queue
    pub fn enqueue(&mut self, mut job: Job) {
        job.state = JobState::Queued;
        self.jobs.push(job);
        self.sort_by_priority();
    }

    /// Get the next job to execute
    pub fn next_job(&mut self) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.state == JobState::Queued)
    }

    /// Sort jobs by priority (highest first)
    fn sort_by_priority(&mut self) {
        self.jobs.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Get all jobs
    pub fn jobs(&self) -> &[Job] {
        &self.jobs
    }

    /// Clear completed jobs
    pub fn clear_completed(&mut self) {
        self.jobs.retain(|j| j.state != JobState::Completed);
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_creation() {
        let job = Job::new("test".to_string(), vec!["G0 X0 Y0".to_string()]);
        assert_eq!(job.state, JobState::Queued);
        assert_eq!(job.progress, 0.0);
    }

    #[test]
    fn test_job_progress() {
        let mut job = Job::new(
            "test".to_string(),
            vec!["G0 X0 Y0".to_string(), "G0 X10 Y10".to_string()],
        );
        job.state = JobState::Running;
        job.next_line();
        assert_eq!(job.progress, 0.5);
    }

    #[test]
    fn test_job_queue_priority() {
        let mut queue = JobQueue::new();
        let mut job1 = Job::new("job1".to_string(), vec![]);
        job1.priority = JobPriority::new(3);
        queue.enqueue(job1);

        let mut job2 = Job::new("job2".to_string(), vec![]);
        job2.priority = JobPriority::new(8);
        queue.enqueue(job2);

        assert_eq!(queue.jobs[0].name, "job2");
        assert_eq!(queue.jobs[1].name, "job1");
    }
}
