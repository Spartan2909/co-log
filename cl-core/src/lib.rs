use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod communicator;
mod parser;
mod scanner;
mod transpiler;

pub use communicator::{query_prolog, start_prolog};

/// Remove the '\\?\' prefix that Windows sometimes adds to paths.
pub fn remove_path_prefix(s: &str) -> &str {
    if &s[..4] == r"\\?\" {
        &s[4..]
    } else {
        s
    }
}

/// Read a given file to a string.
pub fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

/// Transpile a given source string to Prolog.
/// Returns the generated code, any queries included in the source string, and a table to translate identifiers from Prolog to co-log.
pub fn transpile(
    source: String,
) -> Result<(String, Vec<transpiler::Query>, transpiler::Identifiers), parser::ParseError> {
    let tokens = scanner::scan(source);
    //dbg!(&tokens);
    let trees = parser::parse(tokens)?;
    //dbg!(&trees);
    Ok(transpiler::transpile(trees, None))
}

/// Transpiles a given source string to Prolog, returning a single query.
pub fn transpile_query(source: String) -> Result<transpiler::Query, parser::ParseError> {
    let (_, queries, _) = transpile(source)?;

    Ok(queries[0].clone())
}
