use std::error::Error;
use std::fmt;

use super::scanner;
use super::scanner::TokenType;

pub mod ast;

#[derive(Debug)]
pub struct ParseError {
    token: scanner::Token,
    expected: Vec<TokenType>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.expected.len() == 1 {
            write!(
                f,
                "expected {:?}, found {:?}",
                self.expected[0], self.token.kind
            )
        } else {
            write!(
                f,
                "expected one of {:?}, found {:?}",
                self.expected, self.token.kind
            )
        }
    }
}

impl ParseError {
    fn new(token: scanner::Token, expected: TokenType) -> Self {
        Self {
            token,
            expected: Vec::from([expected]),
        }
    }
}

impl Error for ParseError {}

fn find_unwrapped_operator(
    tokens: &Vec<scanner::Token>,
    mut i: usize,
    next_term: usize,
) -> Option<usize> {
    let mut open_paren = 0;

    while i < tokens.len() && i < next_term {
        match &tokens[i].kind {
            kind if kind == &TokenType::Operator => {
                //dbg!(open_paren);
                if open_paren == 0 {
                    return Some(i);
                }
            }
            TokenType::LeftParen => open_paren += 1,
            TokenType::RightParen => open_paren -= 1,
            _ => {}
        }

        i += 1
    }

    None
}

/// Finds the next closing parenthesis after 'i' and before 'end'.
/// Returns the index of the closing parenthesis if it is found, and 'end' if it isn't.
fn find_close(
    tokens: &Vec<scanner::Token>,
    mut i: usize,
    end: usize,
) -> Option<usize> {
    while i < end {
        if tokens[i].kind == TokenType::RightParen {
            return Some(i);
        }

        i += 1
    }

    None
}

/// Remove the articles from a vec of tokens, starting at index i and ending at a terminator.
/// Returns a copy of the tokens with the articles stripped out, and a vec of articles, with positions 0, 1, and 2 referring to the left identifier, the relationship, and the right identifier respectively.
fn collapse_articles(
    tokens: &Vec<scanner::Token>,
    mut i: usize,
) -> (Vec<scanner::Token>, Vec<Option<scanner::Token>>) {
    let mut result = Vec::new();
    let mut articles = Vec::new();
    let mut article_found_on_last_iteration = false;

    loop {
        if tokens[i].kind == TokenType::Article {
            articles.push(Some(tokens[i].clone()));
            article_found_on_last_iteration = true
        } else {
            if [TokenType::Literal, TokenType::Variable, TokenType::Pronoun]
                .contains(&tokens[i].kind)
                && !article_found_on_last_iteration
            {
                articles.push(None)
            }

            result.push(tokens[i].clone());
            article_found_on_last_iteration = false;
        }

        i += 1;
        if i >= tokens.len() || tokens[i - 1].is_terminator() {
            break;
        }
    }

    (result, articles)
}

/// Check if a token of type 'kind' exists between indexes start and end in tokens, returning false if a token of type 'stop_at' is found.
fn type_between(
    tokens: &Vec<scanner::Token>,
    start: usize,
    end: usize,
    kind: TokenType,
    stop_at: TokenType,
) -> bool {
    let mut i = start;

    while i < tokens.len() && i < end {
        if tokens[i].kind == kind {
            return true;
        } else if tokens[i].kind == stop_at || tokens[i].is_terminator() {
            return false;
        }
        i += 1
    }

    false
}

/// Parse 'tokens' into a clause, starting from 'i' and ending at 'end'.
fn parse_clause(
    tokens: &Vec<scanner::Token>,
    i: usize,
    end: usize,
) -> Result<ast::Clause, ParseError> {
    //dbg!(tokens[i].start);
    //dbg!(&tokens[i..next_term]);
    let (collapsed, articles) = collapse_articles(&tokens, i);
    if let Some(op_index) = find_unwrapped_operator(&tokens, i, end) {
        let operator = &tokens[op_index];
        let left = Box::new(parse_clause(tokens, i, op_index)?);
        let right = Box::new(parse_clause(tokens, op_index + 1, end)?);

        let op_type = if operator.lexeme.to_lowercase() == "and" {
            ast::OperatorType::And
        } else {
            ast::OperatorType::Or
        };

        return Ok(ast::Clause::Operator {
            op_type,
            left,
            right,
        });
    }

    if collapsed[0].kind == TokenType::LeftParen {
        match find_close(&tokens, i, end) {
            Some(close) => return parse_clause(tokens, i + 1, close),
            None => {
                return Err(ParseError::new(
                    collapsed[end].clone(),
                    TokenType::RightParen,
                ))
            }
        }
    }

    if collapsed[0].is_identifier() {
        let negated = collapsed[2].kind == TokenType::Not;
        let mut normalised: Vec<scanner::Token> = collapsed;
        if negated {
            normalised.remove(2);
        }

        let binary = type_between(
            &normalised,
            0,
            end,
            TokenType::Prepostion,
            TokenType::FullStop,
        );

        let mut left = ast::Identifier::try_from(&normalised[0])?;
        if let Some(article) = &articles[0] {
            left.article = Some(article.lexeme.clone())
        }

        let mut relationship = ast::Identifier::try_from(&normalised[2])?;
        if let Some(article) = &articles[1] {
            relationship.article = Some(article.lexeme.clone())
        }

        let right = if binary {
            relationship.preposition = Some(normalised[3].lexeme.clone());
            let mut right_tmp = ast::Identifier::try_from(&normalised[4])?;
            if let Some(article) = &articles[2] {
                right_tmp.article = Some(article.lexeme.clone())
            }
            Some(right_tmp)
        } else if relationship.kind == ast::IdenType::Variable {
            // special case for `X is not? Y`
            let right_tmp = relationship;
            relationship = ast::Identifier {
                kind: ast::IdenType::Literal,
                article: None,
                lexeme: String::from("eq"),
                preposition: None,
            };
            Some(right_tmp)
        } else {
            None
        };

        let clause = ast::Clause::Simple {
            negated,
            left,
            relationship: relationship,
            right,
        };

        return Ok(clause);
    }

    Err(ParseError {
        token: collapsed[i].clone(),
        expected: Vec::from([
            TokenType::LeftParen,
            TokenType::Article,
            TokenType::Literal,
            TokenType::Variable,
        ]),
    })
}

/// Checks if a vec of tokens contains a token of type 'kind' after 'i' and before a terminator.
fn tokens_contain(tokens: &Vec<scanner::Token>, mut i: usize, kind: TokenType) -> bool {
    while i < tokens.len() {
        if tokens[i].is_terminator() {
            return false;
        }

        if tokens[i].kind == kind {
            return true;
        }

        i += 1
    }

    false
}

/// Returns a copy of the next terminator along, with its location in 'tokens'.
fn next_terminator(tokens: &Vec<scanner::Token>, mut i: usize) -> (&scanner::Token, usize) {
    loop {
        if tokens[i].is_terminator() {
            break;
        }
        if i < tokens.len() {
            i += 1
        } else {
            break;
        }
    }

    (&tokens[i], i)
}

/// Finds the index of the next occurrence of 'kind' in 'tokens'.
fn find_next(tokens: &Vec<scanner::Token>, mut i: usize, kind: TokenType) -> usize {
    while i < tokens.len() {
        if tokens[i].kind == kind {
            break;
        }
        i += 1
    }

    i
}

/// Parses a sequence of tokens into a statement, starting from 'i'.
/// Returns the created statement along with the index it stopped at.
fn parse_stmt(tokens: &Vec<scanner::Token>, i: usize) -> Result<(ast::Stmt, usize), ParseError> {
    let (_, stmt_end) = next_terminator(&tokens, i);

    let mut binary = type_between(
        &tokens,
        i,
        tokens.len(),
        TokenType::Prepostion,
        TokenType::If,
    );
    let (collapsed, articles) = collapse_articles(&tokens, i);
    let mut left_index = 0;
    let rel_index = 2;
    let right_index = 4;
    let mut kind = ast::StmtType::Fact;

    let (next_term, _) = next_terminator(&collapsed, 0);
    if next_term.kind == TokenType::QuestionMark {
        match collapsed[0].kind {
            TokenType::Verb => {
                binary = collapsed.len() == 6;
                left_index = 1;
                kind = ast::StmtType::Query;
            }
            TokenType::Literal | TokenType::Pronoun => {
                binary = true;
                kind = ast::StmtType::Query;
            }
            _ => {}
        }
    }

    let mut left = ast::Identifier::try_from(&collapsed[left_index])?;
    if let Some(tmp) = &articles[0] {
        left.article = Some(tmp.lexeme.clone());
    }

    let mut relationship = ast::Identifier::try_from(&collapsed[rel_index])?;
    if let Some(tmp) = &articles[1] {
        relationship.article = Some(tmp.lexeme.clone());
    }

    let mut stmt = ast::Stmt {
        kind,
        left,
        relationship,
        right: None,
        condition: None,
    };

    if binary {
        let preposition = collapsed[3].lexeme.clone();
        stmt.relationship.preposition = Some(preposition);
        let mut right = ast::Identifier::try_from(&collapsed[right_index])?;
        if let Some(article) = &articles[2] {
            right.article = Some(article.lexeme.clone())
        }
        stmt.right = Some(right)
    }

    // Rule
    if tokens_contain(&tokens, i, TokenType::If) {
        stmt.kind = ast::StmtType::Rule;
        let clause_start = find_next(&tokens, i, TokenType::If) + 1;
        stmt.condition = Some(parse_clause(tokens, clause_start, stmt_end)?);
    }

    Ok((stmt, stmt_end))
}

/// Parses a sequence of tokens into an abstract syntax tree.
pub fn parse(tokens: Vec<scanner::Token>) -> Result<Vec<ast::Stmt>, ParseError> {
    let mut trees: Vec<ast::Stmt> = Vec::new();

    let mut i = 0;
    while i < tokens.len() {
        use TokenType::*;
        match tokens[i].kind {
            EOF => break,
            Article | Literal | Variable | Pronoun | Verb => {
                let (tree, end) = parse_stmt(&tokens, i)?;
                //dbg!(&tree);
                trees.push(tree);
                i = end + 1
            }
            _ => {
                return Err(ParseError {
                    token: tokens[i].clone(),
                    expected: Vec::from([Article, Literal, Variable, Pronoun, Verb]),
                })
            }
        }
    }

    Ok(trees)
}

#[cfg(test)]
mod tests;
