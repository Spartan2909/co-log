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
}

use TokenType::*;

/// A token of the user's source code.
/// This represents a single keyword, identifier, or piece of punctuation, or the end of the file.
#[derive(Debug, Eq, Hash, Clone)]
pub struct Token {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) start: usize,
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

/// Finds the next newline after position i in the given source string.
fn find_newline(source: String, mut i: usize) -> usize {
    while &source[i..i + 1] != "\n" {
        i += 1
    }

    i
}

/// Scans the given source string, converting it into a series of tokens.
pub fn scan(source: String) -> Vec<Token> {
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
                i = find_newline(source.clone(), i);
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
            _ => todo!("unrecognised character"),
        }

        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, "", i));
    tokens
}

#[cfg(test)]
mod tests;
