use std::fmt::Debug;
use super::scanner;

#[derive(Debug, Clone)]
pub enum IdenType { Literal, Variable, Relationship }

#[derive(Debug, Clone)]
pub struct Identifier {
    var: bool,
    lexeme: String,
    article: Option<String>,
    preposition: Option<String>,
    kind: IdenType
}

impl TryFrom<(scanner::Token, Option<scanner::Token>, Option<scanner::Token>)> for Identifier {
    type Error = super::ParseError;

    /// # Arguments
    /// * `value` - A tuple of the base token, an option of the relevant article, and an option of the relevant preposition
    fn try_from(value: (scanner::Token, Option<scanner::Token>, Option<scanner::Token>)) -> Result<Self, super::ParseError> {
        if ![scanner::TokenType::Literal, scanner::TokenType::Variable].contains(&value.0.kind) {
            return Err(super::ParseError { token_index: 0 })
        }

        let article = match value.1 {
            Some(t) => Some(t.lexeme),
            None => None
        };

        let preposition = match value.2 {
            Some(t) => Some(t.lexeme),
            None => None
        };

        let kind = match value.0.kind {
            scanner::TokenType::Variable => IdenType::Variable,
            v if v == scanner::TokenType::Article => IdenType::Relationship,
            _ => IdenType::Literal
        };

        return Ok(Identifier { var: value.0.kind == scanner::TokenType::Variable, lexeme: value.0.lexeme.clone(), article, preposition, kind })
    }
}

#[derive(Debug)]
pub struct Clause {}

pub enum StmtType { Fact, Rule }

pub trait Stmt: Debug {
    fn unary() -> Self where Self: Sized;
    fn type_of(&self) -> StmtType;
    fn is_binary(&self) -> bool;
    fn get_left(&self) -> Identifier;
    fn get_rel(&self) -> Identifier;
    fn get_right(&self) -> Option<Identifier>;
}

#[derive(Debug)]
pub struct Fact {
    binary: bool,
    left: Identifier,
    relationship: Identifier,
    right: Option<Identifier>
}

impl Stmt for Fact {
    fn unary() -> Self where Self: Sized {
        Fact { binary: false }
    }
    
    fn type_of(&self) -> StmtType {
        StmtType::Fact
    }

    fn is_binary(&self) -> bool {
        self.binary
    }

    fn get_left(&self) -> Identifier {
        self.left
    }

    fn get_rel(&self) -> Identifier {
        self.relationship
    }

    fn get_right(&self) -> Option<Identifier> {
        self.right
    }
}

#[derive(Debug)]
pub struct Rule {
    binary: bool,
    left: Identifier,
    relationship: Identifier,
    right: Option<Identifier>,
    condition: Clause
}

impl Stmt for Rule {
    fn type_of(&self) -> StmtType {
        StmtType::Rule
    }

    fn is_binary(&self) -> bool {
        self.binary
    }

    fn get_left(&self) -> Identifier {
        self.left
    }

    fn get_rel(&self) -> Identifier {
        self.relationship
    }

    fn get_right(&self) -> Option<Identifier> {
        self.right
    }
}
