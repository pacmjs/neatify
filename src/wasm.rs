//! WebAssembly bindings for neatify

use crate::core::formatter::FormattingStats as RustFormattingStats;
use crate::{format_directory, format_file};
use serde::{Deserialize, Serialize};
use std::path::Path;
use wasm_bindgen::prelude::*;

/// WebAssembly-compatible error type
#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct NeatifyError {
    message: String,
    error_type: String,
}

#[wasm_bindgen]
impl NeatifyError {
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, error_type: String) -> NeatifyError {
        NeatifyError {
            message,
            error_type,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn error_type(&self) -> String {
        self.error_type.clone()
    }
}

/// WebAssembly-compatible formatting statistics
#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct FormattingStats {
    formatted_files: usize,
    files_needing_formatting: usize,
    total_files: usize,
}

#[wasm_bindgen]
impl FormattingStats {
    #[wasm_bindgen(constructor)]
    pub fn new(
        formatted_files: usize,
        files_needing_formatting: usize,
        total_files: usize,
    ) -> FormattingStats {
        FormattingStats {
            formatted_files,
            files_needing_formatting,
            total_files,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn formatted_files(&self) -> usize {
        self.formatted_files
    }

    #[wasm_bindgen(getter)]
    pub fn files_needing_formatting(&self) -> usize {
        self.files_needing_formatting
    }

    #[wasm_bindgen(getter)]
    pub fn total_files(&self) -> usize {
        self.total_files
    }
}

impl From<RustFormattingStats> for FormattingStats {
    fn from(stats: RustFormattingStats) -> Self {
        FormattingStats {
            formatted_files: stats.formatted_files,
            files_needing_formatting: stats.files_needing_formatting,
            total_files: stats.total_files,
        }
    }
}

/// Format JavaScript code content directly
#[wasm_bindgen]
pub fn format_code(content: &str) -> Result<String, JsValue> {
    use crate::core::formatter::Formatter;
    use crate::formatters::javascript::JavaScriptFormatter;

    let formatter = JavaScriptFormatter::new();
    Ok(formatter.format(content))
}

/// Check if a file is supported by neatify
#[wasm_bindgen]
pub fn is_supported(file_path: &str) -> bool {
    use crate::core::formatter::Formatter;
    use crate::formatters::javascript::JavaScriptFormatter;

    let formatter = JavaScriptFormatter::new();
    formatter.is_supported(Path::new(file_path))
}

/// Format a single file
#[wasm_bindgen]
pub fn format(file_path: &str, write: bool) -> Result<bool, JsValue> {
    match format_file(file_path, write) {
        Ok(result) => Ok(result),
        Err(e) => Err(JsValue::from_str(&format!("Error formatting file: {}", e))),
    }
}

/// Format all supported files in a directory
#[wasm_bindgen]
pub fn format_dir(dir_path: &str, write: bool) -> Result<FormattingStats, JsValue> {
    match format_directory(dir_path, write) {
        Ok(stats) => Ok(stats.into()),
        Err(e) => Err(JsValue::from_str(&format!(
            "Error formatting directory: {}",
            e
        ))),
    }
}

/// Initialize the WebAssembly module
#[wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error reporting
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
