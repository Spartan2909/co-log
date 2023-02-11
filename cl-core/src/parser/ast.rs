use std::fmt::Debug;
use super::scanner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdenType { Literal, Variable, Pronoun }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub kind: IdenType,
    pub lexeme: String,
    pub article: Option<String>,
    pub preposition: Option<String>
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
                token: value.clone(), expected: Vec::from([scanner::TokenType::Literal, scanner::TokenType::Variable]) 
            })
        };

        return Ok(Identifier { lexeme: value.lexeme, article: None, preposition: None, kind })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OperatorType { And, Or }

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Clause {
    Simple {
        negated: bool,
        left: Identifier,
        relationship: Identifier,
        right: Option<Identifier>
    },
    Operator {
        op_type: OperatorType,
        left: Box<Clause>,
        right: Box<Clause>
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StmtType { Fact, Rule, Query }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub kind: StmtType,
    pub left: Identifier,
    pub relationship: Identifier,
    pub right: Option<Identifier>,
    pub condition: Option<Clause>
}
