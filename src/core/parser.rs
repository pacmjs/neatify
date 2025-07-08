//! Parser functionality for code formatting

use crate::core::error::NeatifyError;
use crate::core::tokens::{Token, Tokenizer};
use anyhow::Result;

/// Parser for source code
pub struct Parser<'a> {
    tokenizer: &'a dyn Tokenizer,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given tokenizer
    pub fn new(tokenizer: &'a dyn Tokenizer) -> Self {
        Self { tokenizer }
    }

    /// Parse source code into tokens
    pub fn parse(&self, content: &str) -> Vec<Token> {
        self.tokenizer.tokenize(content)
    }

    /// Parse source code into tokens with error handling
    pub fn parse_with_error_handling(&self, content: &str) -> Result<Vec<Token>> {
        match std::panic::catch_unwind(|| self.tokenizer.tokenize(content)) {
            Ok(tokens) => Ok(tokens),
            Err(_) => {
                Err(NeatifyError::FormattingError("Error parsing source code".to_string()).into())
            }
        }
    }
}

/// Parse source code with the given tokenizer
pub fn parse(content: &str, tokenizer: &dyn Tokenizer) -> Result<Vec<Token>> {
    let parser = Parser::new(tokenizer);
    parser.parse_with_error_handling(content)
}
