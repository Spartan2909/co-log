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
    let argv = ["swipl".as_ptr() as *mut i8, trimmed.as_ptr() as *mut i8].as_mut_ptr();

    unsafe { PL_initialise(2, argv); }

    println!("started prolog");
}

pub fn query_prolog() {

}

pub fn kill_prolog() {
    unsafe { PL_halt(0); }
}
