use cl_core;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    let pl = cl_core::transpile(String::from("X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.")).unwrap();
    let mut path = env::current_exe().unwrap();
    path.pop();
    path.push("temp.pl");
    let context = cl_core::communicator::start_prolog(path.to_str().unwrap()).unwrap();
    cl_core::communicator::query_prolog(context, cl_core::transpiler::Query {
        left: "l2".to_string(),
        relationship: "l1".to_string(),
        right: None
    }).unwrap();
}
