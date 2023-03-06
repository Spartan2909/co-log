use self::TokenType::*;

/// The type of token that an instance of Token represents.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

/// A token of the user's source code.
/// This represents a single keyword, identifier, or piece of punctuation, or the end of the file.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub start: usize,
    pub length: usize,
}

impl Token {
    /// Construct a new token with the given TokenType, lexeme, start, and length
    fn new(kind: TokenType, lexeme: &str, start: usize, length: usize) -> Self {
        Token {
            kind,
            lexeme: String::from(lexeme),
            start,
            length,
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
                i += 1;
                continue;
            }
            '(' => tokens.push(Token::new(LeftParen, "(", i, 1)),
            ')' => tokens.push(Token::new(RightParen, ")", i, 1)),
            '.' => tokens.push(Token::new(FullStop, ".", i, 1)),
            '?' => tokens.push(Token::new(QuestionMark, "?", i, 1)),
            '#' => {
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
                        if lexeme.chars().nth(length - 1).unwrap().is_lowercase() {
                            Literal
                        } else {
                            Variable
                        }
                    }
                };
                tokens.push(Token::new(kind, &lexeme, i, length));

                i += length;
                continue;
            }
            _ => todo!("unrecognised character"),
        }

        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, "", i, 0));
    tokens
}

#[cfg(test)]
mod tests;
