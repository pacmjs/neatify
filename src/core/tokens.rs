//! Token definitions for code formatting

/// Token types for code formatting
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Operator(String),
    Keyword(String),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(String),
    Comment(String),
    Whitespace(String),
    Newline,
    Other(char),
}

/// Trait for language-specific tokenizers
pub trait Tokenizer: std::panic::RefUnwindSafe {
    /// Tokenize source code into tokens
    fn tokenize(&self, content: &str) -> Vec<Token>;
}

/// Parse source code into tokens
pub fn parse(content: &str, tokenizer: &dyn Tokenizer) -> Vec<Token> {
    tokenizer.tokenize(content)
}
