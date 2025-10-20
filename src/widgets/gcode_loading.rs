//! G-code Loading Widget - File selection, loading, and queued sending

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// G-code file info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeFile {
    pub name: String,
    pub path: String,
    pub size: usize,
    pub lines: usize,
    pub content: String,
}

impl GcodeFile {
    /// Create a new G-code file
    pub fn new(name: String, path: String, content: String) -> Self {
        let lines = content.lines().count();
        let size = content.len();
        GcodeFile {
            name,
            path,
            size,
            lines,
            content,
        }
    }

    /// Get file size in KB
    pub fn size_kb(&self) -> f64 {
        self.size as f64 / 1024.0
    }

    /// Validate G-code
    pub fn validate(&self) -> Result<(), String> {
        if self.content.is_empty() {
            return Err("Empty G-code file".to_string());
        }

        let has_g_code = self.content.contains("G0")
            || self.content.contains("G1")
            || self.content.contains("G2")
            || self.content.contains("G3");

        if !has_g_code {
            return Err("No G-code commands found".to_string());
        }

        Ok(())
    }

    /// Get clean G-code without comments and empty lines
    pub fn get_clean_gcode(&self) -> String {
        self.content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && !trimmed.starts_with(';')
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// G-code loading widget state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcodeLoading {
    pub loaded_file: Option<GcodeFile>,
    pub queue: VecDeque<GcodeFile>,
    pub sent_lines: usize,
    pub file_loaded_at: Option<String>,
}

impl GcodeLoading {
    /// Create a new G-code loading widget
    pub fn new() -> Self {
        GcodeLoading {
            loaded_file: None,
            queue: VecDeque::new(),
            sent_lines: 0,
            file_loaded_at: None,
        }
    }

    /// Load a G-code file
    pub fn load_file(&mut self, file: GcodeFile) -> Result<(), String> {
        file.validate()?;
        self.loaded_file = Some(file);
        self.sent_lines = 0;
        self.file_loaded_at = Some(chrono::Utc::now().to_rfc3339());
        Ok(())
    }

    /// Queue a file for loading
    pub fn queue_file(&mut self, file: GcodeFile) {
        self.queue.push_back(file);
    }

    /// Get next file from queue
    pub fn get_next_queued(&mut self) -> Option<GcodeFile> {
        self.queue.pop_front()
    }

    /// Get next line from current file
    pub fn get_next_line(&mut self) -> Option<String> {
        if let Some(ref file) = self.loaded_file {
            let clean = file.get_clean_gcode();
            let lines: Vec<&str> = clean.lines().collect();
            if self.sent_lines < lines.len() {
                let line = lines[self.sent_lines].to_string();
                self.sent_lines += 1;
                return Some(line);
            }
        }
        None
    }

    /// Get progress as percentage
    pub fn get_progress(&self) -> f64 {
        if let Some(ref file) = self.loaded_file {
            if file.lines > 0 {
                (self.sent_lines as f64 / file.lines as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Reset progress
    pub fn reset(&mut self) {
        self.sent_lines = 0;
    }

    /// Clear loaded file
    pub fn clear(&mut self) {
        self.loaded_file = None;
        self.sent_lines = 0;
        self.file_loaded_at = None;
    }

    /// Get queue length
    pub fn queue_length(&self) -> usize {
        self.queue.len()
    }

    /// Get status message
    pub fn get_status(&self) -> String {
        if let Some(ref file) = self.loaded_file {
            format!(
                "{} - {:.1}% sent ({}/{})",
                file.name, self.get_progress(), self.sent_lines, file.lines
            )
        } else {
            "No file loaded".to_string()
        }
    }
}

impl Default for GcodeLoading {
    fn default() -> Self {
        Self::new()
    }
}
