use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Hash)]
enum TokenType {
    // Single character tokens
    FullStop, QuestionMark, LeftParen, RightParen,

    // Reserved words
    Article, Operator, Prepostion, Verb,
    
    // Identifiers
    Literal, Variable,

    EOF
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Token {
    kind: TokenType,
    lexeme: String,
    start: usize,
    length: usize,
}

impl Token {
    fn new(kind: TokenType, lexeme: String, start: usize, length: usize) -> Self {
        Token { kind, lexeme, start, length }
    }
}

pub fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

fn valid_iden(c: char) -> bool {
    c.is_alphabetic() || c == '-' || c == '_'
}

fn get_word(source: &str, start: usize) -> String {
    let mut end = start;
    while end < source.len() && valid_iden(source.chars().nth(end).unwrap()) {
        end += 1;
    }

    String::from(&source[start..end])
}

fn scan(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut i: usize = 0;
    while i < source.len() {
        let c = source.chars().nth(i).unwrap();
        match c {
            c if c.is_whitespace() => {i += 1; continue},
            '(' => tokens.push(Token::new(TokenType::LeftParen, String::from("("), i, 1)),
            ')' => tokens.push(Token::new(TokenType::RightParen, String::from(")"), i, 1)),
            '.' => tokens.push(Token::new(TokenType::FullStop, String::from("."), i, 1)),
            '?' => tokens.push(Token::new(TokenType::QuestionMark, String::from("?"), i, 1)),
            c if c.is_alphabetic() => {
                let lexeme = get_word(&source, i);
                let length = lexeme.len();
                let kind: TokenType;
                match lexeme.to_lowercase().as_str() {
                    "a" | "an" | "the" => kind = TokenType::Article,
                    "and" | "or" => kind = TokenType::Operator,
                    "of" | "to" => kind = TokenType::Prepostion,
                    "is" | "are" => kind = TokenType::Verb,
                    _ => {
                        if lexeme.chars().nth(length-1).unwrap().is_lowercase() {
                            kind = TokenType::Literal;
                        } else {
                            kind = TokenType::Variable;
                        }
                    }
                }
                tokens.push(Token::new(kind, lexeme, i, length));

                i += length;
                continue
            },
            _ => todo!("unrecognised character"),
        }

        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, String::from(""), i, 0));
    tokens
}

pub fn cl_to_prolog(source: String) -> String {
    let tokens = scan(source);
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use crate::TokenType::*;

    #[test]
    fn scan_test() {
        assert_eq!(HashSet::from_iter(scan(String::from("A hamster is a mammal."))),
            HashSet::from([
                Token { kind: Article, lexeme: String::from("A"), start: 0, length: 1 },
                Token { kind: Literal, lexeme: String::from("hamster"), start: 2, length: 7 },
                Token { kind: Verb, lexeme: String::from("is"), start: 10, length: 2 },
                Token { kind: Article, lexeme: String::from("a"), start: 13, length: 1 },
                Token { kind: Literal, lexeme: String::from("mammal"), start: 15, length: 6 },
                Token { kind: FullStop, lexeme: String::from("."), start: 21, length: 1 },
                Token { kind: EOF, lexeme: String::from(""), start: 22, length: 0 }
            ])
        )
    }
}
