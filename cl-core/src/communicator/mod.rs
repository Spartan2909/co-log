use std::env::current_exe;
use swipl_fli::*;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("running destructor");
        unsafe { PL_halt(0); }
    }
}

const _CLEANUP: Cleanup = Cleanup;

fn remove_prefix(s: &str) -> &str {
    if &s[..4] == r"\\?\" {
        &s[4..]
    } else {
        s
    }
}

pub fn start_prolog() {
    let mut path = current_exe().unwrap();
    path.pop();
    path.push("temp.pl");
    let trimmed = remove_prefix(path.to_str().unwrap());
    unsafe {
        let argv = ["swipl".as_ptr() as *mut i8, trimmed.as_ptr() as *mut i8].as_mut_ptr();
        PL_initialise(2, argv);
        //PL_predicate("name".as_ptr() as *const i8, 4, "module".as_ptr() as *const i8);
    }

    println!("test");
}

pub fn query_prolog() {

}
