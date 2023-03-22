use crate::parser::ast::{self, IdenType};

/// An identifier defined in Co-log, with its article and preposition, and the name used to refer to it in Prolog.
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    cl_name: String,
    pl_name: String,
    article: Option<String>,
    preposition: Option<String>,
}

impl Identifier {
    pub fn cl_name(&self) -> &str {
        &self.cl_name
    }

    pub fn pl_name(&self) -> &str {
        &self.pl_name
    }
}

/// A set of identifiers defined in a co-log program.
#[derive(Debug, Clone, PartialEq)]
pub struct Identifiers {
    identifiers: Vec<Identifier>,
    highest_literal: u16,
    highest_variable: u16,
}

impl Identifiers {
    fn new() -> Self {
        let identifiers = vec![Identifier {
            cl_name: "eq".to_string(),
            pl_name: "eq".to_string(),
            article: None,
            preposition: None,
        }];
        Identifiers {
            identifiers,
            highest_literal: 0,
            highest_variable: 0,
        }
    }

    /// Given a co-log identifier's name, get its Prolog name.
    fn get_from_cl_name(&self, cl_name: &str) -> Option<&Identifier> {
        for identifier in &self.identifiers {
            if is_lowercase(&identifier.cl_name[0..1])
                && identifier.cl_name == cl_name.to_lowercase()
                || !is_lowercase(&identifier.cl_name[0..1]) && identifier.cl_name == cl_name
            {
                return Some(identifier);
            }
        }

        None
    }

    /// Adds a new identifier to the array.
    /// Returns the name used to refer to the identifier in Prolog.
    fn add(&mut self, identifier: ast::Identifier) -> &str {
        let (pl_name, cl_name) = match identifier.kind {
            IdenType::Variable | IdenType::Pronoun => {
                self.highest_variable += 1;
                (
                    "V".to_string() + &(self.highest_variable + 1).to_string(),
                    identifier.lexeme,
                )
            }
            IdenType::Literal => {
                self.highest_literal += 1;
                (
                    "l".to_string() + &(self.highest_literal + 1).to_string(),
                    identifier.lexeme.to_lowercase(),
                )
            }
        };

        self.identifiers.push(Identifier {
            cl_name,
            pl_name: pl_name,
            article: identifier.article,
            preposition: identifier.preposition,
        });

        &self.identifiers.last().unwrap().pl_name
    }

    /// Gets the Prolog name of an identifier, creating it if it doesn't exist.
    fn get_or_create(&mut self, identifier: ast::Identifier) -> String {
        if identifier.kind == ast::IdenType::Pronoun {
            self.add(identifier).to_string()
        } else if let Some(identifier) = self.get_from_cl_name(&identifier.lexeme) {
            identifier.pl_name.clone()
        } else {
            self.add(identifier).to_string()
        }
    }

    pub fn identifiers(&self) -> &Vec<Identifier> {
        &self.identifiers
    }
}

impl From<Vec<Identifier>> for Identifiers {
    fn from(identifiers: Vec<Identifier>) -> Self {
        let mut highest_literal = 0;
        let mut highest_variable = 0;

        for identifier in &identifiers {
            match &identifier.pl_name.as_str()[0..1] {
                "V" => highest_variable = identifier.pl_name[1..].parse::<u16>().unwrap(),
                "l" => highest_literal = identifier.pl_name[1..].parse::<u16>().unwrap(),
                _ => {} // Unreachable
            }
        }

        Self {
            identifiers,
            highest_literal,
            highest_variable,
        }
    }
}

/// A query that can be used by the communicator module to query Prolog.
#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub relationship: String,
    pub left: String,
    pub right: Option<String>,
}

/// Checks if a string is lowercase.
fn is_lowercase(s: &str) -> bool {
    s.to_lowercase() == s
}

/// Transpiles the clause of a query to Prolog.
fn transpile_clause(clause: ast::Clause, identifiers: &mut Identifiers) -> String {
    let mut output = String::new();

    match clause {
        ast::Clause::Operator {
            op_type,
            left,
            right,
        } => {
            let left = transpile_clause(*left, identifiers);
            let right = transpile_clause(*right, identifiers);

            output = match op_type {
                ast::OperatorType::And => format!("({left}, {right})"),
                ast::OperatorType::Or => format!("({left}; {right})"),
            };

            output
        }
        ast::Clause::Simple {
            negated,
            left,
            relationship,
            right,
        } => {
            if negated {
                output += r"\+"
            }

            output += &format!("{}(", identifiers.get_or_create(relationship));
            output += &identifiers.get_or_create(left);
            if let Some(right) = right {
                output += &format!(", {}", identifiers.get_or_create(right))
            }
            output += ")";

            output
        }
    }
}

/// Transpile a series of abstract syntax trees into a Prolog file.
/// Returns the generated Prolog, the queries, and a map of Co-log identifiers to Prolog names.
pub fn transpile(
    trees: Vec<ast::Stmt>,
    initial_identifiers: Option<Identifiers>,
) -> (String, Vec<Query>, Identifiers) {
    let mut identifiers = match initial_identifiers {
        Some(tmp) => tmp,
        None => Identifiers::new(),
    };

    let mut output = String::from("style_check(-discontiguous).\neq(X, Y) :- X == Y.\n");
    let mut queries = Vec::new();

    for tree in trees {
        match tree.kind {
            ast::StmtType::Query => {
                let relationship = identifiers.get_or_create(tree.relationship).to_string();
                let left = identifiers.get_or_create(tree.left).to_string();
                let right = match tree.right {
                    Some(iden) => Some(identifiers.get_or_create(iden).to_string()),
                    None => None,
                };

                queries.push(Query {
                    relationship,
                    left,
                    right,
                })
            }
            _ => {
                output += &format!("{}(", identifiers.get_or_create(tree.relationship));
                output += &identifiers.get_or_create(tree.left);
                if let Some(right) = tree.right {
                    output += &format!(", {}", identifiers.get_or_create(right))
                }
                output += ")";

                if tree.kind == ast::StmtType::Rule {
                    output += " :- ";
                    output += &transpile_clause(tree.condition.unwrap(), &mut identifiers);
                }

                output += ".\n";
            }
        }
    }

    (output, queries, identifiers)
}

#[cfg(test)]
mod tests;
