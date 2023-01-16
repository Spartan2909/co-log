use std::error::Error;
use std::fmt;

use super::scanner;
use super::scanner::TokenType;

mod ast;

#[derive(Debug)]
pub struct ParseError {
    token: scanner::Token,
    expected: Vec<TokenType>
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.expected.len() == 1 {
            write!(f, "Expected {:?}, found {:?}", self.expected[0], self.token.kind)
        } else {
            write!(f, "Expected one of {:?}, found {:?}", self.expected, self.token.kind)
        }
    }
}

impl Error for ParseError {}

fn check(tokens: &Vec<scanner::Token>, i: usize, expected: TokenType) -> bool {
    tokens[i + 1].kind == expected
}

fn next_terminator(tokens: &Vec<scanner::Token>, mut i: usize) -> (scanner::Token, usize) {
    while i < tokens.len() {
        if tokens[i].is_terminator() {break}
        i += 1
    }

    (tokens[i].clone(), i)
}



impl ast::Clause {

}

impl ast::Stmt {
    fn contains(tokens: &Vec<scanner::Token>, mut i: usize, kind: TokenType) -> bool {
        let mut found = false;
    
        while i < tokens.len() {
            if tokens[i].is_terminator() {break}
    
            if tokens[i].kind == kind {found = true; break}

            i += 1
        }
    
        found
    }

    fn collapse_articles(tokens: &Vec<scanner::Token>, mut i: usize) -> (Vec<scanner::Token>, Vec<Option<scanner::Token>>) {
        let mut result = Vec::new();
        let mut articles = Vec::new();
        let mut article_last_iter = false;

        while i < tokens.len() && !tokens[i].is_terminator() {
            if tokens[i].kind == TokenType::Article {
                articles.push(Some(tokens[i].clone()));
                article_last_iter = true
            } else {
                if [TokenType::Literal, TokenType::Variable].contains(&tokens[i].kind) && !article_last_iter {
                    articles.push(None)
                }

                result.push(tokens[i].clone());
                article_last_iter = false;
            }

            i += 1;
        }

        (result, articles)
    }

    pub fn new(tokens: Vec<scanner::Token>, i: usize) -> Result<(Self, usize), ParseError> {
        let next_term = next_terminator(&tokens, i);
        let binary = Self::contains(&tokens, i, TokenType::Prepostion);
        dbg!(binary);
        let collapsed = Self::collapse_articles(&tokens, i);
        match next_term.0.kind {
            // Fact or rule
            TokenType::FullStop => {
                let mut left = ast::Identifier::try_from(collapsed.0[0].clone())?;
                if collapsed.1[0].is_some() {
                    left.article = Some(collapsed.1[0].clone().unwrap().lexeme);
                }

                let mut relationship = ast::Identifier::try_from(collapsed.0[2].clone())?;
                if collapsed.1[1].is_some() {
                    relationship.article = Some(collapsed.1[1].clone().unwrap().lexeme);
                }

                let mut stmt = Self { kind: ast::StmtType::Fact, left, relationship, right: None, condition: None };

                if binary {
                    let preposition = collapsed.0[3].lexeme.clone();
                    let mut right = ast::Identifier::try_from(collapsed.0[4].clone())?;
                    right.preposition = Some(preposition);
                    stmt.right = Some(right)
                }

                println!("{:?}", stmt);

                // Fact
                if !Self::contains(&tokens, i, TokenType::If) {
                    return Ok((stmt, next_term.1))
                }

                // Rule
                todo!()
            },
            // Query
            TokenType::QuestionMark => {
                todo!()
            }
            _ => {Err(ParseError { token: next_term.0, expected: Vec::new() } )}
        }
    }
}

pub fn parse(tokens: Vec<scanner::Token>) -> Result<Vec<ast::Stmt>, ParseError> {
    let mut trees: Vec<ast::Stmt> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        use TokenType::*;
        match tokens[i].kind {
            EOF => break,
            Article | Literal | Variable | Pronoun => {
                let tree = ast::Stmt::new(tokens.clone(), i)?;
                trees.push(tree.0);
                i = tree.1
            },
            _ => return Err(ParseError { token: tokens[i].clone(), expected: Vec::from([Article, Literal, Variable, Pronoun]) })
        }

        i += 1;
    }

    Ok(trees)
}
