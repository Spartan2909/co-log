use cl_core;
use std::env::current_exe;

fn main() {
    let pl = cl_core::transpile(String::from("X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.")).unwrap();
    let mut path = current_exe().unwrap();
    path.pop();
    path.push("temp.pl");
    let context = cl_core::communicator::start_prolog(path.to_str().unwrap());
}
