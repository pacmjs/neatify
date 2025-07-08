//! JavaScript formatter implementation

mod formatter;
mod tokenizer;

use crate::core::formatter::Formatter;
use std::path::Path;

/// JavaScript formatter
pub struct JavaScriptFormatter;

impl JavaScriptFormatter {
    /// Create a new JavaScript formatter
    pub fn new() -> Self {
        Self
    }
}

impl Formatter for JavaScriptFormatter {
    fn format(&self, content: &str) -> String {
        formatter::format_javascript(content)
    }

    fn is_supported(&self, file_path: &Path) -> bool {
        if let Some(extension) = file_path.extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            return ext == "js" || ext == "mjs" || ext == "cjs";
        }
        false
    }
}
