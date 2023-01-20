use crate::parser::ast;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    cl_name: String,
    pl_name: String,
    article: Option<String>,
    preposition: Option<String>
}

#[derive(Debug, Clone)]
struct Identifiers {
    identifiers: Vec<Identifier>,
    highest_literal: u64,
    highest_variable: u64
}

impl From<Vec<Identifier>> for Identifiers {
    fn from(identifiers: Vec<Identifier>) -> Self {
        Self { identifiers, highest_literal: 0, highest_variable: 0 }
    }
}

impl Identifiers {
    fn new() -> Self {
        Identifiers { identifiers: Vec::new(), highest_literal: 0, highest_variable: 0 }
    }

    fn get_from_cl_name(&self, cl_name: String) -> Option<Identifier> {
        for identifier in &self.identifiers {
            if identifier.cl_name == cl_name {
                return Some(identifier.clone())
            }
        }
        
        None
    }

    fn add(&mut self, statement: ast::Identifier) -> String {
        let pl_name;

        if statement.kind == ast::IdenType::Variable {
            pl_name = "V".to_string() + &(self.highest_variable + 1).to_string();
            self.highest_variable += 1
        } else {
            pl_name = "l".to_string() + &(self.highest_literal + 1).to_string();
            self.highest_literal += 1
        }

        self.identifiers.push(Identifier {
            cl_name: statement.lexeme, pl_name: pl_name.clone(), article: statement.article, preposition: statement.preposition
        });

        pl_name
    }

    fn get_or_create(&mut self, statement: ast::Identifier) -> String {
        let result;
        if let Some(tmp) = self.get_from_cl_name(statement.lexeme.clone()) {
            result = tmp.pl_name
        } else {
            result = self.add(statement)
        }

        result
    }
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

pub fn transpile(trees: Vec<ast::Stmt>) -> (String, String, Vec<Identifier>) {
    let mut facts = String::from("style_check(-discontiguous).\n");
    let mut rules = String::from("eq(X, Y) :- X == Y.\n");
    let mut queries = String::new();
    let mut identifiers = Identifiers::new();

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

    (facts + &rules, queries, identifiers.identifiers)
}

#[cfg(test)]
mod tests;
