use cl_core::transpile;

fn main() {
    transpile(String::from("
John is the sibling of Jack.
John is male.
X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Z."
    ));
}
