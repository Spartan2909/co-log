use std::ptr::{null, null_mut};
use swipl_fli::*;

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        println!("running destructor");
        unsafe { PL_halt(0); }
    }
}

const _CLEANUP: Cleanup = Cleanup;

fn str_to_ptr(s: &str) -> *const i8 {
    return s.as_ptr() as *const i8
}

fn str_to_mut_ptr(s: &str) -> *mut i8 {
    return s.as_ptr() as *mut i8
}

unsafe fn consult_file(filename: &str) {
    let p = PL_predicate(str_to_ptr("consult"), 1, null());
    let a = PL_new_term_ref();
    PL_put_atom_chars(a, str_to_ptr(filename));
    PL_call_predicate(null_mut(), PL_Q_NORMAL.try_into().unwrap(), p, a);
}

pub fn start_prolog() {
    unsafe {
        let argv = [str_to_mut_ptr("consult_file"), str_to_mut_ptr("-q")].as_mut_ptr();
        PL_initialise(2, null_mut());
        //consult_file("filename");
        //PL_predicate("name".as_ptr() as *const i8, 4, "module".as_ptr() as *const i8);
    }

    println!("test");
}

pub fn query_prolog() {

}
