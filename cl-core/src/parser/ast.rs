use std::fmt::Debug;
use super::scanner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdenType { Literal, Variable }

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
        } else if value.kind == scanner::TokenType::Variable || value.kind == scanner::TokenType::Pronoun {
            IdenType::Variable
        } else {
            return Err(super::ParseError {
                token: value.clone(), expected: Vec::from([scanner::TokenType::Literal, scanner::TokenType::Variable]) 
            })
        };

        return Ok(Identifier { lexeme: value.lexeme, article: None, preposition: None, kind })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ClauseType { Simple, Operator }
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OperatorType { And, Or }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Clause {
    pub kind: ClauseType,
    pub negated: bool,
    pub op_type: Option<OperatorType>,
    pub left_clause: Option<Box<Clause>>,
    pub right_clause: Option<Box<Clause>>,
    pub left_iden: Option<Identifier>,
    pub relationship: Option<Identifier>,
    pub right_iden: Option<Identifier>
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
