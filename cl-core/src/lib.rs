use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod scanner;
mod parser;
pub mod transpiler;
pub mod communicator;

pub fn remove_path_prefix(s: &str) -> &str {
    if &s[..4] == r"\\?\" {
        &s[4..]
    } else {
        s
    }
}

pub fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

pub fn transpile(source: String) -> Result<(String, Vec<transpiler::Query>, transpiler::Identifiers), parser::ParseError> {
    let tokens = scanner::scan(source);
    //dbg!(&tokens);
    let trees = parser::parse(tokens)?;
    //dbg!(&trees);
    Ok(transpiler::transpile(trees, None))
}
