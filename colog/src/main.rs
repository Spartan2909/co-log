use cl_core;
fn main() {
    let pl = cl_core::transpile(String::from("X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.")).unwrap();
    cl_core::communicator::start_prolog("worskspaces/co-log/target/debug/temp.pl");
}
