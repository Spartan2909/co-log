use super::scanner;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdenType {
    Literal,
    Variable,
    Pronoun,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub(crate) kind: IdenType,
    pub(crate) lexeme: String,
    pub(crate) article: Option<String>,
    pub(crate) preposition: Option<String>,
}

impl TryFrom<scanner::Token> for Identifier {
    type Error = super::ParseError;

    fn try_from(value: scanner::Token) -> Result<Self, super::ParseError> {
        let kind = if value.kind == scanner::TokenType::Literal {
            IdenType::Literal
        } else if value.kind == scanner::TokenType::Variable {
            IdenType::Variable
        } else if value.kind == scanner::TokenType::Pronoun {
            IdenType::Pronoun
        } else {
            return Err(super::ParseError {
                token: value.clone(),
                expected: Vec::from([scanner::TokenType::Literal, scanner::TokenType::Variable]),
            });
        };

        return Ok(Identifier {
            lexeme: value.lexeme,
            article: None,
            preposition: None,
            kind,
        });
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OperatorType {
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Clause {
    Simple {
        negated: bool,
        left: Identifier,
        relationship: Identifier,
        right: Option<Identifier>,
    },
    Operator {
        op_type: OperatorType,
        left: Box<Clause>,
        right: Box<Clause>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StmtType {
    Fact,
    Rule,
    Query,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub(crate) kind: StmtType,
    pub(crate) left: Identifier,
    pub(crate) relationship: Identifier,
    pub(crate) right: Option<Identifier>,
    pub(crate) condition: Option<Clause>,
}
