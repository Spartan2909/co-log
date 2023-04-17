use crate::scanner::{Token, TokenType};

/// The type of term that an identifier represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdenType {
    Literal,
    Variable,
    Pronoun,
}

/// A literal, variable, or pronoun.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub(super) kind: IdenType,
    pub(super) lexeme: String,
    pub(super) article: Option<String>,
    pub(super) preposition: Option<String>,
}

impl Identifier {
    pub fn kind(&self) -> IdenType {
        self.kind
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn article(&self) -> &Option<String> {
        &self.article
    }

    pub fn preposition(&self) -> &Option<String> {
        &self.article
    }
}

impl TryFrom<&Token> for Identifier {
    type Error = super::ParseError;

    /// Convert a Token to an Identifier, and return a ParseError if the conversion fails.
    fn try_from(value: &Token) -> Result<Self, super::ParseError> {
        let kind = if value.kind() == TokenType::Literal {
            IdenType::Literal
        } else if value.kind() == TokenType::Variable {
            IdenType::Variable
        } else if value.kind() == TokenType::Pronoun {
            IdenType::Pronoun
        } else {
            return Err(super::ParseError {
                token: value.clone(),
                expected: Vec::from([TokenType::Literal, TokenType::Variable]),
            });
        };

        return Ok(Identifier {
            lexeme: value.lexeme().to_string(),
            article: None,
            preposition: None,
            kind,
        });
    }
}

/// The function of an operator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperatorType {
    And,
    Or,
}

/// A clause in a rule. Note that clauses of the form `'(' clause ')'` have no special representation, as the parentheses simply change the order of the parsing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Clause {
    /// A clause of the form `article? identifier verb ‘not’? article? literal (preposition article? identifier)?.`
    Simple {
        negated: bool,
        left: Identifier,
        relationship: Identifier,
        right: Option<Identifier>,
    },

    /// A clause of the form `clause operator clause`.
    Operator {
        op_type: OperatorType,
        left: Box<Clause>,
        right: Box<Clause>,
    },
}

/// The type of a statement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StmtType {
    Fact,
    Rule,
    Query,
}

/// A statement, terminated with a full stop or a question mark.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub(super) kind: StmtType,
    pub(super) left: Identifier,
    pub(super) relationship: Identifier,
    pub(super) right: Option<Identifier>,
    pub(super) condition: Option<Clause>,
}

impl Stmt {
    pub fn kind(&self) -> StmtType {
        self.kind
    }

    pub fn left(&self) -> &Identifier {
        &self.left
    }

    pub fn relationship(&self) -> &Identifier {
        &self.relationship
    }

    pub fn right(&self) -> &Option<Identifier> {
        &self.right
    }

    pub fn condition(&self) -> &Option<Clause> {
        &self.condition
    }
}
