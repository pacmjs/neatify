//! JavaScript formatter implementation

use crate::core::tokens::Token;
use crate::core::parser;
use super::tokenizer::JavaScriptTokenizer;

/// Format JavaScript code
pub(crate) fn format_javascript(content: &str) -> String {
    let tokenizer = JavaScriptTokenizer::new();
    let tokens = match parser::parse(content, &tokenizer) {
        Ok(tokens) => tokens,
        Err(_) => return content.to_string(), // Return original content on error
    };
    let formatted = format_tokens(&tokens);

    let normalized_content = normalize_whitespace(content);
    let normalized_formatted = normalize_whitespace(&formatted);

    if normalized_content == normalized_formatted {
        return content.to_string();
    }

    formatted
}

/// Normalize whitespace for comparison
fn normalize_whitespace(s: &str) -> String {
    // Replace multiple whitespace with a single space
    let mut result = String::new();
    let mut last_was_whitespace = false;

    for c in s.chars() {
        if c.is_whitespace() {
            if !last_was_whitespace {
                result.push(' ');
                last_was_whitespace = true;
            }
        } else {
            result.push(c);
            last_was_whitespace = false;
        }
    }

    result.trim().to_string()
}

/// Format tokens into a string
fn format_tokens(tokens: &[Token]) -> String {
    let mut result = String::new();
    let mut indent_level = 0;
    let mut at_line_start = true;

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::OpenBrace => {
                // Add space before brace if not at line start and not already preceded by a space
                if !at_line_start && !result.ends_with(' ') {
                    result.push(' ');
                }
                result.push('{');
                indent_level += 1;

                // Add newline after opening brace
                result.push('\n');
                at_line_start = true;
            }
            Token::CloseBrace => {
                // Add newline before closing brace if not at line start
                if !at_line_start {
                    result.push('\n');
                }

                // Decrease indent level
                if indent_level > 0 {
                    indent_level -= 1;
                }

                // Add indentation
                for _ in 0..indent_level {
                    result.push_str("  ");
                }

                result.push('}');

                // Add newline after closing brace unless followed by specific tokens
                let next_token = tokens.get(i + 1);
                if !matches!(
                    next_token,
                    Some(Token::Semicolon) | Some(Token::Comma) | Some(Token::CloseParen)
                ) {
                    result.push('\n');
                    at_line_start = true;
                } else {
                    at_line_start = false;
                }
            }
            Token::OpenParen => {
                result.push('(');
                at_line_start = false;
            }
            Token::CloseParen => {
                result.push(')');
                at_line_start = false;
            }
            Token::OpenBracket => {
                result.push('[');
                at_line_start = false;
            }
            Token::CloseBracket => {
                result.push(']');
                at_line_start = false;
            }
            Token::Semicolon => {
                result.push(';');

                // Add newline after semicolon
                result.push('\n');
                at_line_start = true;
            }
            Token::Colon => {
                result.push(':');
                result.push(' '); // Add space after colon
                at_line_start = false;
            }
            Token::Comma => {
                result.push(',');
                result.push(' '); // Add space after comma
                at_line_start = false;
            }
            Token::Dot => {
                result.push('.');
                at_line_start = false;
            }
            Token::Operator(op) => {
                // Add space before operator unless it's a unary operator
                let prev_token = if i > 0 { tokens.get(i - 1) } else { None };
                let is_unary = matches!(
                    prev_token,
                    Some(Token::OpenParen)
                        | Some(Token::OpenBrace)
                        | Some(Token::OpenBracket)
                        | Some(Token::Comma)
                        | Some(Token::Semicolon)
                        | Some(Token::Colon)
                        | Some(Token::Operator(_))
                        | None
                );

                if !is_unary && !at_line_start {
                    result.push(' ');
                }

                result.push_str(op);

                // Add space after operator
                let next_token = tokens.get(i + 1);
                if !matches!(
                    next_token,
                    Some(Token::Semicolon) | Some(Token::Comma) | Some(Token::CloseParen)
                ) {
                    result.push(' ');
                }

                at_line_start = false;
            }
            Token::Keyword(keyword) => {
                // Add indentation at line start
                if at_line_start {
                    for _ in 0..indent_level {
                        result.push_str("  ");
                    }
                }

                result.push_str(keyword);

                // Add space after keyword
                let next_token = tokens.get(i + 1);
                if !matches!(
                    next_token,
                    Some(Token::Semicolon) | Some(Token::Comma) | Some(Token::Dot)
                ) {
                    result.push(' ');
                }

                at_line_start = false;
            }
            Token::Identifier(ident) => {
                // Add indentation at line start
                if at_line_start {
                    for _ in 0..indent_level {
                        result.push_str("  ");
                    }
                }

                result.push_str(ident);
                at_line_start = false;
            }
            Token::StringLiteral(s) => {
                result.push('"');
                result.push_str(s);
                result.push('"');
                at_line_start = false;
            }
            Token::NumberLiteral(n) => {
                result.push_str(n);
                at_line_start = false;
            }
            Token::Comment(c) => {
                // Add indentation at line start
                if at_line_start {
                    for _ in 0..indent_level {
                        result.push_str("  ");
                    }
                } else {
                    result.push(' '); // Add space before inline comment
                }

                result.push_str("//");
                result.push_str(c);
                at_line_start = false;
            }
            Token::Whitespace(_ws) => {
                // Skip whitespace tokens - we'll add spaces where needed
                // This prevents extra spaces from being added
            }
            Token::Newline => {
                result.push('\n');
                at_line_start = true;
            }
            Token::Other(c) => {
                result.push(*c);
                at_line_start = false;
            }
        }
    }

    // Ensure the file ends with a newline
    if !result.ends_with('\n') {
        result.push('\n');
    }

    result
}
