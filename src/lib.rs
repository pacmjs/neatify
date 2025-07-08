//! Neatify - A code formatter library
//!
//! Neatify is a library for formatting code in various languages.
//! It provides a API for formatting files and directories.

mod core;
mod formatters;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use core::formatter::FormattingStats;
pub use formatters::{format_directory, format_file};

/// Format a file with the appropriate formatter
///
/// # Arguments
/// * `file_path` - Path to the file to format
/// * `write` - Whether to write the formatted output back to the file
///
/// # Returns
/// * `Ok(bool)` - `true` if the file needed formatting, `false` if it was already formatted
/// * `Err` - If formatting failed
pub fn format<P: AsRef<std::path::Path>>(file_path: P, write: bool) -> anyhow::Result<bool> {
    format_file(file_path, write)
}

/// Format all supported files in a directory
///
/// # Arguments
/// * `dir_path` - Path to the directory to format
/// * `write` - Whether to write the formatted output back to the files
///
/// # Returns
/// * `Ok(FormattingStats)` - Statistics about the formatting operation
/// * `Err` - If formatting failed
pub fn format_dir<P: AsRef<std::path::Path>>(
    dir_path: P,
    write: bool,
) -> anyhow::Result<FormattingStats> {
    format_directory(dir_path, write)
}
