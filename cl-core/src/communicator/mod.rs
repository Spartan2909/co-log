use std::env::current_exe;
use std::path::PathBuf;
use swipl_fli::*;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("running destructor");
        unsafe { PL_halt(0); }
    }
}

const _CLEANUP: Cleanup = Cleanup;

fn path_to_mut_ptr(p: PathBuf) -> *mut i8 {
    p.to_str().unwrap().as_ptr() as *mut i8
}

pub fn start_prolog() {
    let mut path = current_exe().unwrap();
    path.pop();
    path.push("temp.pl");
    unsafe {
        let argv = ["swipl".as_ptr() as *mut i8, path_to_mut_ptr(path)].as_mut_ptr();
        PL_initialise(2, argv);
        //PL_predicate("name".as_ptr() as *const i8, 4, "module".as_ptr() as *const i8);
    }

    println!("test");
}

pub fn query_prolog() {

}
