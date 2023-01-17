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

impl ParseError {
    fn new(token: scanner::Token, expected: TokenType) -> Self {
        Self { token, expected: Vec::from([expected]) }
    }
}

impl Error for ParseError {}

fn index_unwrapped_operator(tokens: &Vec<scanner::Token>, mut i: usize, next_term: usize, search_kind: TokenType) -> Option<usize> {
    let mut open_paren = 0;

    while i < tokens.len() && i < next_term {
        dbg!(&search_kind, &tokens[i]);
        match &tokens[i].kind {
            kind if kind == &search_kind => {
                dbg!(open_paren);
                if open_paren == 0 {
                    return Some(i)
                }
            },
            TokenType::LeftParen => open_paren += 1,
            TokenType::RightParen => open_paren -= 1,
            _ => {}
        }

        i += 1
    }
    
    None
}

fn find_close(tokens: &Vec<scanner::Token>, i: usize, next_term: usize) -> Result<usize, usize> {
    while i < next_term {
        if tokens[i].kind == TokenType::RightParen {
            return Ok(i)
        }
    }

    Err(i)
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

fn type_between(tokens: &Vec<scanner::Token>, start: usize, end: usize, kind: TokenType, stop_at: TokenType) -> bool {
    let mut i = start;

    while i < tokens.len() && i < end {
        if tokens[i].kind == kind {return true}
        else if tokens[i].kind == stop_at || tokens[i].is_terminator() {return false}
        i += 1
    }

    false
}

impl ast::Clause {
    fn new(tokens: Vec<scanner::Token>, i: usize, next_term: usize) -> Result<Self, ParseError> {
        dbg!(tokens[i].start);
        let (collapsed, articles) = collapse_articles(&tokens, i);
        if let Some(op_index) = index_unwrapped_operator(&collapsed, 0, next_term, TokenType::Operator) {
            println!("detected compound");
            let operator = collapsed[op_index].clone();
            let left_clause = Some(Box::new(Self::new(tokens.clone(), i, op_index)?));
            let right_clause = Some(Box::new(Self::new(tokens.clone(), op_index + 1, next_term)?));

            let op_type = Some(if operator.lexeme.to_lowercase() == "and" {
                ast::OperatorType::And
            } else {
                ast::OperatorType::Or
            });

            return Ok(Self { kind: ast::ClauseType::Operator, negated: false, op_type, left_clause,
                right_clause, left_iden: None, relationship: None, right_iden: None })
        }

        if collapsed[0].kind == TokenType::LeftParen {
            println!("detected parenthesis");
            let close_paren = find_close(&collapsed, 0, next_term);

            match close_paren {
                Ok(close) => return Self::new(tokens, i + 1, close),
                Err(terminator) => return Err(ParseError::new(collapsed[terminator].clone(),  TokenType::RightParen))
            }
        }

        if collapsed[0].is_identifier() {
            println!("detected simple");
            let negated = collapsed[2].kind == TokenType::Not;
            let mut normalised: Vec<scanner::Token> = collapsed.clone();
            if negated {
                normalised.remove(2);
            }

            let binary 
                = type_between(&normalised, i, next_term, TokenType::Prepostion, TokenType::FullStop);

            let mut left = ast::Identifier::try_from(normalised[0].clone())?;
            if articles[0].is_some() {
                left.article = Some(articles[0].clone().unwrap().lexeme)
            }

            let mut relationship = ast::Identifier::try_from(normalised[2].clone())?;
            if articles[1].is_some() {
                relationship.article = Some(articles[1].clone().unwrap().lexeme)
            }

            let right;
            if binary {
                relationship.preposition = Some(normalised[3].lexeme.clone());
                let mut right_tmp = ast::Identifier::try_from(normalised[4].clone())?;
                if articles[2].is_some() {
                    right_tmp.article = Some(articles[2].clone().unwrap().lexeme)
                }
                right = Some(right_tmp);
            } else if relationship.kind == ast::IdenType::Variable {
                // special case for `X is not? Y`
                right = Some(relationship);
                relationship = ast::Identifier { kind: ast::IdenType::Literal, article: None, lexeme: String::from("equal"), preposition: None };
            } else {
                right = None;
            }

            let clause = Self {
                kind: ast::ClauseType::Simple, negated, op_type: None, left_clause: None, right_clause: None,
                left_iden: Some(left), relationship: Some(relationship), right_iden: right
            };

            return Ok(clause)
        }

        Err(ParseError { token: collapsed[i].clone(), expected: Vec::from(
            [TokenType::LeftParen, TokenType::Article, TokenType::Literal, TokenType::Variable]
        ) })
    }
}

impl ast::Stmt {
    fn contains(tokens: &Vec<scanner::Token>, mut i: usize, kind: TokenType) -> bool {
        while i < tokens.len() {
            if tokens[i].is_terminator() {break}
    
            if tokens[i].kind == kind {return true}

            i += 1
        }
    
        false
    }

    fn next_terminator(tokens: &Vec<scanner::Token>, mut i: usize) -> (scanner::Token, usize) {
        while i < tokens.len() {
            if tokens[i].is_terminator() {break}
            i += 1
        }
    
        (tokens[i].clone(), i)
    }

    fn find_next(tokens: &Vec<scanner::Token>, mut i: usize, kind: TokenType) -> usize {
        while i < tokens.len() {
            if tokens[i].kind == kind {break}
            i += 1
        }

        i
    }

    fn new(tokens: Vec<scanner::Token>, i: usize) -> Result<(Self, usize), ParseError> {
        let next_term = Self::next_terminator(&tokens, i);
        let binary = type_between(&tokens, i, tokens.len(), TokenType::Prepostion, TokenType::If);
        let collapsed = collapse_articles(&tokens, i);
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
                    stmt.relationship.preposition = Some(preposition);
                    let right = ast::Identifier::try_from(collapsed.0[4].clone())?;
                    stmt.right = Some(right)
                }

                // Rule
                if Self::contains(&tokens, i, TokenType::If) {
                    stmt.kind = ast::StmtType::Rule;
                    let clause_start = Self::find_next(&tokens, i, TokenType::If) + 1;
                    dbg!(clause_start);
                    stmt.condition = Some(ast::Clause::new(collapsed.0, clause_start, next_term.1)?);
                }

                Ok((stmt, next_term.1))
            },
            // Query
            TokenType::QuestionMark => {
                todo!()
            }
            _ => unimplemented!()
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
                let (tree, end) = ast::Stmt::new(tokens.clone(), i)?;
                dbg!(&tree);
                trees.push(tree);
                i = end
            },
            _ => return Err(ParseError { token: tokens[i].clone(), expected: Vec::from([Article, Literal, Variable, Pronoun]) })
        }

        i += 1;
    }

    Ok(trees)
}
