use std::fmt;

use super::parser::ParseError;

/// The type of token that an instance of Token represents.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    // Single character tokens
    FullStop,
    QuestionMark,
    LeftParen,
    RightParen,

    // Reserved words
    Article,
    Operator,
    Prepostion,
    Verb,
    If,
    Pronoun,
    Not,

    // Identifiers
    Literal,
    Variable,

    EOF,

    Error,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            FullStop => "full stop",
            QuestionMark => "question mark",
            LeftParen => "left parenthesis",
            RightParen => "right parenthesis",
            Article => "article",
            Operator => "operator",
            Prepostion => "preposition",
            Verb => "verb",
            If => "'if'",
            Pronoun => "pronoun",
            Not => "'not'",
            Literal => "literal",
            Variable => "variable",
            EOF => "end of file",
            Error => "error",
        };

        write!(f, "{text}")
    }
}

use TokenType::*;

/// A token of the user's source code.
/// This represents a single keyword, identifier, or piece of punctuation, or the end of the file.
#[derive(Debug, Eq, Hash, Clone)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    start: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.lexeme.is_empty() {
            write!(f, "{}", self.kind)
        } else {
            write!(f, "{}", self.lexeme)
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.lexeme == other.lexeme
    }
}

impl Token {
    /// Construct a new token with the given TokenType, lexeme, start, and length
    fn new(kind: TokenType, lexeme: &str, start: usize) -> Self {
        Token {
            kind,
            lexeme: String::from(lexeme),
            start,
        }
    }

    /// Determines whether the token is a terminator i.e. '.' or '?'
    pub fn is_terminator(&self) -> bool {
        [FullStop, QuestionMark].contains(&self.kind)
    }

    /// Determines whether the token is a user-defined identifier.
    pub fn is_identifier(&self) -> bool {
        [Literal, Variable].contains(&self.kind)
    }

    pub fn kind(&self) -> TokenType {
        self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn start(&self) -> usize {
        self.start
    }
}

/// Determines whether the given character is valid in an identifier.
fn valid_iden(c: char) -> bool {
    c.is_alphabetic() || c == '-' || c == '_'
}

/// Extracts a single word from the given source string, starting at 'start'.
fn get_word(source: &str, start: usize) -> String {
    let mut end = start;
    while end < source.len() && valid_iden(source.chars().nth(end).unwrap()) {
        end += 1;
    }

    String::from(&source[start..end])
}

/// Finds the next newline in the given source string.
fn find_newline(source: &str) -> usize {
    let mut i = 0;
    while &source[i..i + 1] != "\n" {
        i += 1
    }

    i
}

/// Scans the given source string, converting it into a series of tokens.
pub fn scan(source: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();

    let mut i = 0;
    while i < source.len() {
        let c = source.chars().nth(i).unwrap();
        match c {
            c if c.is_whitespace() => {
                // Ignore and move to next character
                i += 1;
                continue;
            }
            '(' => tokens.push(Token::new(LeftParen, "(", i)),
            ')' => tokens.push(Token::new(RightParen, ")", i)),
            '.' => tokens.push(Token::new(FullStop, ".", i)),
            '?' => tokens.push(Token::new(QuestionMark, "?", i)),
            '#' => {
                // Comment, skip to next line
                i = find_newline(&source[i..]);
                continue;
            }
            c if c.is_alphabetic() => {
                let lexeme = get_word(&source, i);
                let length = lexeme.len();
                let kind = match lexeme.to_lowercase().as_str() {
                    "a" | "an" | "the" => Article,
                    "and" | "or" => Operator,
                    "of" | "to" => Prepostion,
                    "is" | "are" => Verb,
                    "if" => If,
                    "who" | "what" => Pronoun,
                    "not" => Not,
                    _ => {
                        // If the last character is lowercase
                        if lexeme.chars().nth(length - 1).unwrap().is_lowercase() {
                            Literal
                        } else {
                            Variable
                        }
                    }
                };
                tokens.push(Token::new(kind, &lexeme, i));

                i += lexeme.len();
                continue;
            }
            _ => Err(ParseError::from(Token::new(
                TokenType::Error,
                &c.to_string(),
                i,
            )))?,
        }

        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, "", i));
    Ok(tokens)
}

#[cfg(test)]
mod tests;
