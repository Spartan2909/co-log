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

pub fn query_prolog(query: &str) {
    unsafe {
        let t = PL_new_term_refs(1);
        PL_put_atom_chars(t, "a".as_ptr() as *const i8);
        let pred = PL_predicate("one".as_ptr() as *mut i8, 1, std::ptr::null());
        //PL_put_atom_chars(t, "".as_ptr() as *mut i8);
        let query = PL_open_query(std::ptr::null_mut(), PL_Q_CATCH_EXCEPTION as i32, pred, t);

        let mut solns = Vec::new();
        loop {
            let soln = PL_next_solution(query);
            if soln == FALSE as i32 {
                break;
            } else {
                solns.push(soln);
            }
        }

        dbg!(*query);

        PL_cut_query(query);

        dbg!(solns);
    }
}

pub fn kill_prolog() {
    unsafe { PL_halt(0); }
}
