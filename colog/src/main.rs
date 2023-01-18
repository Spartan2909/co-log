use cl_core::transpile;

fn main() {
    /*transpile(String::from("
John is the sibling of Jack.
John is male.
    "));*/
    /*transpile(String::from("
John is the parent of Jack.
John is the parent of Jane.
X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y."
    ));*/
    transpile(String::from("Is a hamster a mammal?"));
}
