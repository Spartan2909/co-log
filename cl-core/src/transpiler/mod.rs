use crate::parser::ast;

#[derive(Debug, Clone, PartialEq)]
struct Identifier {
    cl_name: String,
    pl_name: String,
    article: Option<String>,
    preposition: Option<String>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifiers {
    identifiers: Vec<Identifier>,
    highest_literal: u16,
    highest_variable: u16
}

impl From<Vec<Identifier>> for Identifiers {
    fn from(identifiers: Vec<Identifier>) -> Self {
        let mut highest_literal = 0;
        let mut highest_variable = 0;

        for identifier in &identifiers {
            match &identifier.pl_name.as_str()[0..1] {
                "V" => highest_variable = identifier.pl_name[1..].parse::<u16>().unwrap(),
                "l" => highest_literal = identifier.pl_name[1..].parse::<u16>().unwrap(),
                &_ => {} // Unreachable
            }
        }

        Self { identifiers, highest_literal, highest_variable }
    }
}

impl Identifiers {
    fn new() -> Self {
        Identifiers { identifiers: Vec::new(), highest_literal: 0, highest_variable: 0 }
    }

    fn get_from_cl_name(&self, cl_name: String) -> Option<Identifier> {
        for identifier in &self.identifiers {
            if is_lowercase(&identifier.cl_name[0..1]) && identifier.cl_name == cl_name.to_lowercase()
            || !is_lowercase(&identifier.cl_name[0..1]) && identifier.cl_name == cl_name {
                return Some(identifier.clone())
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
            cl_name, pl_name: pl_name.clone(), article: statement.article, preposition: statement.preposition
        });

        pl_name
    }

    fn get_or_create(&mut self, identifier: ast::Identifier) -> String {
        let result;

        if identifier.kind == ast::IdenType::Pronoun {
            return self.add(identifier)
        }

        if let Some(tmp) = self.get_from_cl_name(identifier.lexeme.clone()) {
            result = tmp.pl_name
        } else {
            result = self.add(identifier)
        }

        result
    }
}

fn is_lowercase(s: &str) -> bool {
    s.to_lowercase() == s
}

fn transpile_clause(clause: ast::Clause, mut identifiers: Identifiers) -> (String, Identifiers) {
    let mut output = String::new();
    if clause.negated {
        output += r"\+"
    }

    if clause.kind == ast::ClauseType::Operator {
        let left;
        let right;
        (left, identifiers) = transpile_clause(*clause.left_clause.unwrap(), identifiers);
        (right, identifiers) = transpile_clause(*clause.right_clause.unwrap(), identifiers);

        output = match clause.op_type.unwrap() {
            ast::OperatorType::And => format!("({left}, {right})"),
            ast::OperatorType::Or => format!("({left}; {right})")
        };

        (output, identifiers)
    } else {
        output += &format!("{}(", identifiers.get_or_create(clause.relationship.unwrap()));
        output += &identifiers.get_or_create(clause.left_iden.unwrap());
        if let Some(right) = clause.right_iden {
            output += &format!(", {}", identifiers.get_or_create(right))
        }
        output += ")";

        (output, identifiers)
    }
}

pub fn transpile(trees: Vec<ast::Stmt>, initial_identifiers: Option<Identifiers>) -> (String, String, Identifiers) {
    let mut identifiers = match initial_identifiers {
        Some(tmp) => tmp,
        None => Identifiers::new()
    };

    let mut facts = String::from("style_check(-discontiguous).\n");
    let mut rules = String::from("eq(X, Y) :- X == Y.\n\r");
    let mut queries = String::new();

    for tree in trees {
        let mut result = String::new();

        result += &format!("{}(", identifiers.get_or_create(tree.relationship));
        result += &identifiers.get_or_create(tree.left);
        if let Some(right) = tree.right {
            result += &format!(", {}", identifiers.get_or_create(right))
        }
        result += ")";

        if tree.kind == ast::StmtType::Rule {
            result += " :- ";
            let clause;
            (clause, identifiers) = transpile_clause(tree.condition.unwrap(), identifiers);
            result += &clause;
        }

        result += ".\n";

        match tree.kind {
            ast::StmtType::Fact => facts += &result,
            ast::StmtType::Rule => rules += &result,
            ast::StmtType::Query => queries += &result
        }
    }

    (facts + &rules, queries, identifiers)
}

#[cfg(test)]
mod tests;
