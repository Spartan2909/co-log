use crate::parser::ast;

/// An identifier defined in Co-log, with its article and preposition, and the name used to refer to it in Prolog.
#[derive(Debug, Clone, PartialEq)]
struct Identifier {
    cl_name: String,
    pl_name: String,
    article: Option<String>,
    preposition: Option<String>,
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
    fn get_from_cl_name(&self, cl_name: String) -> Option<Identifier> {
        for identifier in &self.identifiers {
            if is_lowercase(&identifier.cl_name[0..1])
                && identifier.cl_name == cl_name.to_lowercase()
                || !is_lowercase(&identifier.cl_name[0..1]) && identifier.cl_name == cl_name
            {
                return Some(identifier.clone());
            }
        }

        None
    }

    fn add(&mut self, statement: ast::Identifier) -> String {
        let pl_name;
        let cl_name;

        if statement.kind == ast::IdenType::Variable {
            pl_name = "V".to_string() + &(self.highest_variable + 1).to_string();
            cl_name = statement.lexeme;
            self.highest_variable += 1
        } else if statement.kind == ast::IdenType::Pronoun {
            pl_name = "V".to_string() + &(self.highest_variable + 1).to_string();
            cl_name = statement.lexeme.to_lowercase();
            self.highest_variable += 1
        } else {
            pl_name = "l".to_string() + &(self.highest_literal + 1).to_string();
            cl_name = statement.lexeme.to_lowercase();
            self.highest_literal += 1
        }

        self.identifiers.push(Identifier {
            cl_name,
            pl_name: pl_name.clone(),
            article: statement.article,
            preposition: statement.preposition,
        });

        pl_name
    }

    fn get_or_create(&mut self, identifier: ast::Identifier) -> String {
        let result;

        if identifier.kind == ast::IdenType::Pronoun {
            return self.add(identifier);
        }

        if let Some(tmp) = self.get_from_cl_name(identifier.lexeme.clone()) {
            result = tmp.pl_name
        } else {
            result = self.add(identifier)
        }

        result
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

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub relationship: String,
    pub left: String,
    pub right: Option<String>,
}

fn is_lowercase(s: &str) -> bool {
    s.to_lowercase() == s
}

fn transpile_clause(clause: ast::Clause, mut identifiers: Identifiers) -> (String, Identifiers) {
    let mut output = String::new();

    match clause {
        ast::Clause::Operator {
            op_type,
            left,
            right,
        } => {
            let (left, identifiers) = transpile_clause(*left, identifiers);
            let (right, identifiers) = transpile_clause(*right, identifiers);

            output = match op_type {
                ast::OperatorType::And => format!("({left}, {right})"),
                ast::OperatorType::Or => format!("({left}; {right})"),
            };

            (output, identifiers)
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

            (output, identifiers)
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
                let relationship = identifiers.get_or_create(tree.relationship);
                let left = identifiers.get_or_create(tree.left);
                let right = match tree.right {
                    Some(iden) => Some(identifiers.get_or_create(iden)),
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
                    let clause;
                    (clause, identifiers) = transpile_clause(tree.condition.unwrap(), identifiers);
                    output += &clause;
                }

                output += ".\n";
            }
        }
    }

    (output, queries, identifiers)
}

#[cfg(test)]
mod tests;
