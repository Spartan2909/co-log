use cl_core;

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
    /*let r = transpile(String::from("
        John is the sibling of Jack.
        John is male.
        X is the brother of Y if X is the sibling of Y and X is male.
        Is John the brother of Jack?
    "));*/
    let pl = cl_core::transpile(String::from("X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.")).unwrap();
    let r = cl_core::communicator::start_prolog("C:/Users/caleb/Documents/GitHub/co-log/test.pl", "call_nth((thing(X, Y), write(X), write(\",\"), writeln(Y)), 100).");
    dbg!(r);
    //println!("{}", r.unwrap().0);
}
