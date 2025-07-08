//! Core formatter functionality

use std::fs;
use std::path::Path;
use anyhow::Result;
use crate::core::error::NeatifyError;

/// Statistics about formatting operations
#[derive(Debug, Default, Clone)]
pub struct FormattingStats {
    /// Number of files that were formatted
    pub formatted_files: usize,
    /// Number of files that needed formatting but were not modified (dry run)
    pub files_needing_formatting: usize,
    /// Total number of files processed
    pub total_files: usize,
}

impl FormattingStats {
    /// Create a new empty FormattingStats
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge another FormattingStats into this one
    pub fn merge(&mut self, other: &FormattingStats) {
        self.formatted_files += other.formatted_files;
        self.files_needing_formatting += other.files_needing_formatting;
        self.total_files += other.total_files;
    }
}

/// Trait for language-specific formatters
pub trait Formatter {
    /// Format content according to language-specific rules
    fn format(&self, content: &str) -> String;

    /// Check if a file is supported by this formatter
    fn is_supported(&self, file_path: &Path) -> bool;

    /// Format a file
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to format
    /// * `write` - Whether to write the formatted output back to the file
    ///
    /// # Returns
    /// * `Ok(bool)` - `true` if the file needed formatting, `false` if it was already formatted
    /// * `Err` - If formatting failed
    fn format_file(&self, file_path: &Path, write: bool) -> Result<bool> {
        // Check if file exists
        if !file_path.exists() {
            return Err(NeatifyError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File does not exist: {}", file_path.display())
            )).into());
        }

        // Check if file is supported
        if !self.is_supported(file_path) {
            return Err(NeatifyError::UnsupportedFile(file_path.display().to_string()).into());
        }

        // Read the original content
        let content = fs::read_to_string(file_path)?;

        // Format the content
        let formatted = self.format(&content);

        // Check if formatting changed the content
        let needs_formatting = content != formatted;

        // Write the formatted content back to the file if needed
        if needs_formatting && write {
            fs::write(file_path, formatted)?;
        }

        Ok(needs_formatting)
    }
}
