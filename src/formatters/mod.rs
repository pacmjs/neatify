//! Language-specific formatters

pub mod javascript;

use crate::core::formatter::{Formatter, FormattingStats};
use anyhow::Result;
use std::path::Path;
use crate::core::error::NeatifyError;

/// Format a file with the appropriate formatter
pub fn format_file<P: AsRef<Path>>(file_path: P, write: bool) -> Result<bool> {
    let file_path = file_path.as_ref();
    
    // Get the appropriate formatter for the file
    let formatter = get_formatter_for_file(file_path);
    
    match formatter {
        Some(formatter) => formatter.format_file(file_path, write),
        None => Err(NeatifyError::UnsupportedFile(file_path.display().to_string()).into()),
    }
}

/// Format all supported files in a directory
pub fn format_directory<P: AsRef<Path>>(dir_path: P, write: bool) -> Result<FormattingStats> {
    let dir_path = dir_path.as_ref();
    
    // Check if the directory exists
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(NeatifyError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory does not exist: {}", dir_path.display())
        )).into());
    }
    
    let mut stats = FormattingStats::new();
    format_directory_recursive(dir_path, write, &mut stats)?;
    
    Ok(stats)
}

/// Recursively format all supported files in a directory
fn format_directory_recursive(
    dir_path: &Path,
    write: bool,
    stats: &mut FormattingStats,
) -> Result<()> {
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Skip node_modules directory
            if path
                .file_name()
                .map_or(false, |name| name == "node_modules")
            {
                continue;
            }

            // Recursively process subdirectories
            format_directory_recursive(&path, write, stats)?;
        } else if path.is_file() {
            // Try to format the file if there's a formatter available
            if let Some(formatter) = get_formatter_for_file(&path) {
                stats.total_files += 1;

                match formatter.format_file(&path, write) {
                    Ok(true) => {
                        if write {
                            stats.formatted_files += 1;
                        } else {
                            stats.files_needing_formatting += 1;
                        }
                    }
                    Ok(false) => {
                        // File was already formatted
                    }
                    Err(e) => {
                        // Return error with proper error type
                        return Err(NeatifyError::FormattingError(
                            format!("Error formatting {}: {}", path.display(), e)
                        ).into());
                    }
                }
            }
        }
    }

    Ok(())
}

/// Get the appropriate formatter for a file
fn get_formatter_for_file(file_path: &Path) -> Option<Box<dyn Formatter>> {
    // Create instances of all available formatters
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(javascript::JavaScriptFormatter::new()),
    ];

    // Find the first formatter that supports this file
    for formatter in formatters {
        if formatter.is_supported(file_path) {
            return Some(formatter);
        }
    }

    None
}
