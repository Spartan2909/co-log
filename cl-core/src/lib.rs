use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
mod scanner;

pub fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

pub fn cl_to_prolog(source: String) -> String {
    let tokens = scanner::scan(source);
    println!("{:?}", tokens);
    String::from("")
}
