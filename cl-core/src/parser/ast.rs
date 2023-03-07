use super::scanner;

/// The type of term that an identifier represents.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdenType {
    Literal,
    Variable,
    Pronoun,
}

/// A literal, variable, or pronoun.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub(crate) kind: IdenType,
    pub(crate) lexeme: String,
    pub(crate) article: Option<String>,
    pub(crate) preposition: Option<String>,
}

impl TryFrom<&scanner::Token> for Identifier {
    type Error = super::ParseError;

    /// Convert a Token to an Identifier, and return a ParseError if the conversion fails.
    fn try_from(value: &scanner::Token) -> Result<Self, super::ParseError> {
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
            lexeme: value.lexeme.clone(),
            article: None,
            preposition: None,
            kind,
        });
    }
}

/// The function of an operator.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OperatorType {
    And,
    Or,
}

/// A clause in a rule. Note that clauses of the form `'(' clause ')'` has no special representation, as it simply changes the order of the parsing.
#[derive(Debug, PartialEq, Eq, Hash)]
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
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StmtType {
    Fact,
    Rule,
    Query,
}

/// A statement, terminated with a full stop or a question mark.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Stmt {
    pub(crate) kind: StmtType,
    pub(crate) left: Identifier,
    pub(crate) relationship: Identifier,
    pub(crate) right: Option<Identifier>,
    pub(crate) condition: Option<Clause>,
}
