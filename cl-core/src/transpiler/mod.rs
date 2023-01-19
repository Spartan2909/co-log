use crate::parser::ast::Stmt;

#[derive(Clone)]
struct Identifier {
    cl_name: String,
    pl_name: String,
    article: Option<String>,
    preposition: Option<String>
}

struct IdenVec {
    identifiers: Vec<Identifier>,
    highest_literal: u64,
    highest_variable: u64
}

impl IdenVec {
    fn new() -> Self {
        IdenVec { identifiers: Vec::new(), highest_literal: 0, highest_variable: 0 }
    }

    fn get_from_cl_name(&self, cl_name: String) -> Option<Identifier> {
        for identifier in &self.identifiers {
            if identifier.cl_name == cl_name {
                return Some(identifier.clone())
            }
        }
        
        None
    }

    fn add(&mut self, cl_name: String, article: Option<String>, preposition: Option<String>, variable: bool) -> String {
        let pl_name;

        if variable {
            pl_name = "V".to_string() + &(self.highest_variable + 1).to_string();
        } else {
            pl_name = "i".to_string() + &(self.highest_literal + 1).to_string();
        }

        self.identifiers.push(Identifier { cl_name, pl_name: pl_name.clone(), article, preposition });

        pl_name
    }
}

pub fn transpile(trees: Vec<Stmt>) -> String {
    let facts = String::from("style_check(-discontiguous).\n");
    let rules = String::from("eq(X, Y) :- X == Y.\n");
    let queries = String::new();
    let mut identifiers = IdenVec::new();

    for tree in trees {
        let result = String::new();

        let pl_name;
        if let Some(tmp) = identifiers.get_from_cl_name(tree.relationship.lexeme.clone()) {
            pl_name = tmp.pl_name;
        } else {
            pl_name = identifiers.add(
                tree.relationship.lexeme, tree.relationship.article, tree.relationship.preposition, false
            )
        }
    }

    todo!()
}
