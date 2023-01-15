use super::scanner;

mod ast;

pub fn parse(tokens: Vec<scanner::Token>) -> Vec<Box<dyn ast::Stmt>> {
    let trees: Vec<Box<dyn ast::Stmt>> = Vec::new();

    let i = 0;
    while i < tokens.len() {
        match tokens[i].kind {
            _ => {}
        }
    }

    return trees;
}
