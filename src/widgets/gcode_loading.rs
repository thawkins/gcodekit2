//! G-code loading widget for file selection and queuing

use std::path::PathBuf;

/// G-code loading widget for file management
pub struct GcodeLoadingWidget {
    pub selected_file: Option<PathBuf>,
    pub file_queue: Vec<PathBuf>,
}

impl GcodeLoadingWidget {
    /// Create a new G-code loading widget
    pub fn new() -> Self {
        Self {
            selected_file: None,
            file_queue: Vec::new(),
        }
    }

    /// Load a G-code file
    pub fn load_file(&mut self, path: PathBuf) -> anyhow::Result<String> {
        let content = std::fs::read_to_string(&path)?;
        self.selected_file = Some(path);
        Ok(content)
    }

    /// Add file to queue
    pub fn queue_file(&mut self, path: PathBuf) {
        self.file_queue.push(path);
    }

    /// Get next queued file
    pub fn next_queued(&mut self) -> Option<PathBuf> {
        if self.file_queue.is_empty() {
            None
        } else {
            Some(self.file_queue.remove(0))
        }
    }
}

impl Default for GcodeLoadingWidget {
    fn default() -> Self {
        Self::new()
    }
}
