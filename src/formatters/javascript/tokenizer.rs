//! JavaScript tokenizer implementation

use crate::core::tokens::{Token, Tokenizer};

/// JavaScript tokenizer implementation
pub struct JavaScriptTokenizer;

impl JavaScriptTokenizer {
    /// Create a new JavaScript tokenizer
    pub fn new() -> Self {
        Self
    }
}

impl Tokenizer for JavaScriptTokenizer {
    fn tokenize(&self, content: &str) -> Vec<Token> {
        tokenize_javascript(content)
    }
}

/// Tokenize JavaScript code
pub(crate) fn tokenize_javascript(content: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = content.chars().peekable();
    
    let mut in_string = false;
    let mut string_delimiter = '"';
    let mut current_string = String::new();
    
    let mut in_comment = false;
    let mut in_multiline_comment = false;
    let mut current_comment = String::new();
    
    let mut in_identifier = false;
    let mut current_identifier = String::new();
    
    let mut in_number = false;
    let mut current_number = String::new();
    
    let mut in_operator = false;
    let mut current_operator = String::new();
    
    let keywords = [
        "var", "let", "const", "function", "return", "if", "else", "for", "while", "do",
        "switch", "case", "default", "break", "continue", "try", "catch", "finally", "throw",
        "new", "delete", "typeof", "instanceof", "in", "this", "super", "class", "extends",
        "import", "export", "from", "as", "async", "await", "yield", "true", "false", "null",
        "undefined", "void"
    ];
    
    while let Some(c) = chars.next() {
        // Handle string literals
        if in_string {
            if c == string_delimiter && chars.peek() != Some(&'\\') {
                in_string = false;
                tokens.push(Token::StringLiteral(current_string.clone()));
                current_string.clear();
            } else {
                current_string.push(c);
            }
            continue;
        }
        
        // Handle comments
        if in_comment {
            if c == '\n' {
                in_comment = false;
                tokens.push(Token::Comment(current_comment.clone()));
                tokens.push(Token::Newline);
                current_comment.clear();
            } else {
                current_comment.push(c);
            }
            continue;
        }
        
        if in_multiline_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next(); // Consume the '/'
                in_multiline_comment = false;
                tokens.push(Token::Comment(current_comment.clone()));
                current_comment.clear();
            } else {
                current_comment.push(c);
                if c == '\n' {
                    tokens.push(Token::Newline);
                }
            }
            continue;
        }
        
        // Handle identifiers
        if in_identifier {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                current_identifier.push(c);
            } else {
                in_identifier = false;
                if keywords.contains(&current_identifier.as_str()) {
                    tokens.push(Token::Keyword(current_identifier.clone()));
                } else {
                    tokens.push(Token::Identifier(current_identifier.clone()));
                }
                current_identifier.clear();
                
                // Process the current character
                process_char(c, &mut chars, &mut tokens, 
                             &mut in_string, &mut string_delimiter, &mut current_string,
                             &mut in_comment, &mut in_multiline_comment, &mut current_comment,
                             &mut in_number, &mut current_number,
                             &mut in_operator, &mut current_operator,
                             &mut in_identifier, &mut current_identifier);
            }
            continue;
        }
        
        // Handle numbers
        if in_number {
            if c.is_digit(10) || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                current_number.push(c);
            } else {
                in_number = false;
                tokens.push(Token::NumberLiteral(current_number.clone()));
                current_number.clear();
                
                // Process the current character
                process_char(c, &mut chars, &mut tokens, 
                             &mut in_string, &mut string_delimiter, &mut current_string,
                             &mut in_comment, &mut in_multiline_comment, &mut current_comment,
                             &mut in_number, &mut current_number,
                             &mut in_operator, &mut current_operator,
                             &mut in_identifier, &mut current_identifier);
            }
            continue;
        }
        
        // Handle operators
        if in_operator {
            if is_operator_char(c) {
                current_operator.push(c);
            } else {
                in_operator = false;
                tokens.push(Token::Operator(current_operator.clone()));
                current_operator.clear();
                
                // Process the current character
                process_char(c, &mut chars, &mut tokens, 
                             &mut in_string, &mut string_delimiter, &mut current_string,
                             &mut in_comment, &mut in_multiline_comment, &mut current_comment,
                             &mut in_number, &mut current_number,
                             &mut in_operator, &mut current_operator,
                             &mut in_identifier, &mut current_identifier);
            }
            continue;
        }
        
        // Start of new tokens
        process_char(c, &mut chars, &mut tokens, 
                     &mut in_string, &mut string_delimiter, &mut current_string,
                     &mut in_comment, &mut in_multiline_comment, &mut current_comment,
                     &mut in_number, &mut current_number,
                     &mut in_operator, &mut current_operator,
                     &mut in_identifier, &mut current_identifier);
    }
    
    // Handle any remaining tokens
    if in_identifier {
        if keywords.contains(&current_identifier.as_str()) {
            tokens.push(Token::Keyword(current_identifier));
        } else {
            tokens.push(Token::Identifier(current_identifier));
        }
    }
    
    if in_number {
        tokens.push(Token::NumberLiteral(current_number));
    }
    
    if in_operator {
        tokens.push(Token::Operator(current_operator));
    }
    
    tokens
}

/// Process a single character during tokenization
#[allow(clippy::too_many_arguments)]
fn process_char(
    c: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    tokens: &mut Vec<Token>,
    in_string: &mut bool,
    string_delimiter: &mut char,
    _current_string: &mut String,
    in_comment: &mut bool,
    in_multiline_comment: &mut bool,
    _current_comment: &mut String,
    in_number: &mut bool,
    current_number: &mut String,
    in_operator: &mut bool,
    current_operator: &mut String,
    in_identifier: &mut bool,
    current_identifier: &mut String,
) {
    match c {
        '{' => tokens.push(Token::OpenBrace),
        '}' => tokens.push(Token::CloseBrace),
        '(' => tokens.push(Token::OpenParen),
        ')' => tokens.push(Token::CloseParen),
        '[' => tokens.push(Token::OpenBracket),
        ']' => tokens.push(Token::CloseBracket),
        ';' => tokens.push(Token::Semicolon),
        ':' => tokens.push(Token::Colon),
        ',' => tokens.push(Token::Comma),
        '.' => {
            if chars.peek().map_or(false, |&next| next.is_digit(10)) {
                *in_number = true;
                current_number.push(c);
            } else {
                tokens.push(Token::Dot);
            }
        },
        '"' | '\'' => {
            *in_string = true;
            *string_delimiter = c;
        },
        '/' => {
            if chars.peek() == Some(&'/') {
                chars.next(); // Consume the second '/'
                *in_comment = true;
            } else if chars.peek() == Some(&'*') {
                chars.next(); // Consume the '*'
                *in_multiline_comment = true;
            } else if is_operator_char(c) {
                *in_operator = true;
                current_operator.push(c);
            } else {
                tokens.push(Token::Other(c));
            }
        },
        '0'..='9' => {
            *in_number = true;
            current_number.push(c);
        },
        'a'..='z' | 'A'..='Z' | '_' | '$' => {
            *in_identifier = true;
            current_identifier.push(c);
        },
        ' ' | '\t' => {
            tokens.push(Token::Whitespace(c.to_string()));
        },
        '\n' => {
            tokens.push(Token::Newline);
        },
        _ => {
            if is_operator_char(c) {
                *in_operator = true;
                current_operator.push(c);
            } else {
                tokens.push(Token::Other(c));
            }
        }
    }
}

/// Check if a character is part of an operator
fn is_operator_char(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~' | '?')
}
