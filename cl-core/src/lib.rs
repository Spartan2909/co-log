use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(path: &str) -> io::Result<String> {
    // Create a path to the desired file
    let path = Path::new(path);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path)?;

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_test() {
        match read_file("../hello") {
            Ok(_) | Err(_) => todo!(),
        }
    }
}
