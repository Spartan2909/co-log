use std::fmt::Debug;
use super::scanner;

#[derive(Debug, Clone)]
pub enum IdenType { Literal, Variable }

#[derive(Debug, Clone)]
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
        } else {
            return Err(super::ParseError {
                token: value.clone(), expected: Vec::from([scanner::TokenType::Literal, scanner::TokenType::Variable]) 
            })
        };

        return Ok(Identifier { lexeme: value.lexeme, article: None, preposition: None, kind })
    }
}

#[derive(Debug)]
pub enum ClauseType { Simple, Operator }
#[derive(Debug)]
pub enum OperatorType { And, Or }

#[derive(Debug)]
pub struct Clause {
    kind: ClauseType,
    negated: bool,
    op_type: Option<OperatorType>,
    left_clause: Option<Box<Clause>>,
    right_clause: Option<Box<Clause>>,
    left_iden: Option<Identifier>,
    right_iden: Option<Identifier>
}

#[derive(Debug)]
pub enum StmtType { Fact, Rule, Query }

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtType,
    pub left: Identifier,
    pub relationship: Identifier,
    pub right: Option<Identifier>,
    pub condition: Option<Clause>
}

impl Stmt {
    pub fn fact_unary(left: scanner::Token, relationship: scanner::Token) -> Result<Self, super::ParseError> {
        let left = Identifier::try_from(left)?;
        let relationship = Identifier::try_from(relationship)?;
        Ok(Self { kind: StmtType::Fact, left, relationship, right: None, condition: None })
    }

    pub fn fact_binary(left: scanner::Token, relationship: scanner::Token, right: scanner::Token) -> Result<Self, super::ParseError> {
        let left = Identifier::try_from(left)?;
        let relationship = Identifier::try_from(relationship)?;
        let right = Identifier::try_from(right)?;
        Ok(Self { kind: StmtType::Fact, left, relationship, right: Some(right), condition: None })
    }

    pub fn rule_unary(left: scanner::Token, relationship: scanner::Token) -> Result<Self, super::ParseError> {
        let left = Identifier::try_from(left)?;
        let relationship = Identifier::try_from(relationship)?;
        Ok(Self { kind: StmtType::Rule, left, relationship, right: None, condition: None })
    }

    pub fn rule_binary(left: scanner::Token, relationship: scanner::Token, right: scanner::Token) -> Result<Self, super::ParseError> {
        let left = Identifier::try_from(left)?;
        let relationship = Identifier::try_from(relationship)?;
        let right = Identifier::try_from(right)?;
        Ok(Self { kind: StmtType::Rule, left, relationship, right: Some(right), condition: None })
    }
}
