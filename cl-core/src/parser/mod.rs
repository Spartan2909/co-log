use std::error::Error;
use std::fmt;

use super::scanner;
use super::scanner::TokenType;

pub mod ast;

#[derive(Debug)]
struct ParseError {
    token_index: usize
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "Unexpected token")
    }
}

impl Error for ParseError {}

fn check(tokens: &Vec<scanner::Token>, i: usize, expected: TokenType) -> bool {
    tokens[i + 1].kind == expected
}

fn next_terminator(tokens: &Vec<scanner::Token>, mut i: usize) -> scanner::Token {
    while i < tokens.len() {
        if [TokenType::FullStop, TokenType::QuestionMark].contains(&tokens[i].kind) {break}
        i += 1
    }

    tokens[i].clone()
}

pub fn parse(tokens: Vec<scanner::Token>) -> Vec<Box<dyn ast::Stmt>> {
    let trees: Vec<Box<dyn ast::Stmt>> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].kind {
            TokenType::EOF => break,
            TokenType::Article => if !(check(&tokens, i, TokenType::Literal) || check(&tokens, i, TokenType::Variable)) {panic!()},
            TokenType::Literal => {
                let next_term = next_terminator(&tokens, i);
                if next_term.kind == TokenType::FullStop {

                } else {

                }
            },
            _ => {}
        }

        i += 1;
    }

    return trees
}
