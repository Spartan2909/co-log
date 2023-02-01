use swipl::prelude::*;
use swipl_fli;

fn remove_prefix(s: &str) -> &str {
    if &s[..4] == r"\\?\" {
        &s[4..]
    } else {
        s
    }
}

pub fn start_prolog(source: &str) -> PrologResult<Context<ActivatedEngine>> {
    let activation = initialize_swipl().unwrap();
    let context: Context<_> = activation.into();

    let consult = pred!(consult/1);

    let location = remove_prefix(source);
    let term = term!{ context: #location }?;

    context.call_once(consult, [&term])?;
    
    Ok(context)
}

pub fn query_prolog(context: Context<ActivatedEngine>, query: crate::transpiler::Query) -> PrologResult<()> {
    let mut terms = Vec::new();
    let term;
    let term_l;
    let term_r;
    let relationship = query.relationship.as_str();

    let open_query = match query.right {
        None => {
            let pred: CallablePredicate<1> = unsafe {
                CallablePredicate::wrap(
                    swipl_fli::PL_predicate(relationship.as_ptr() as *const i8, 1, std::ptr::null())
                )   
            };

            let left = query.left;
            term = term!{context: #left}?;
            terms.push(&term);
            
            context.open(pred, [&term])
        },
        Some(right) => {
            let pred: CallablePredicate<2> = unsafe {
                CallablePredicate::wrap(
                    swipl_fli::PL_predicate(relationship.as_ptr() as *const i8, 2, std::ptr::null())
                )
            };

            let left = query.left;
            term_l = term!{context: #left}?;
            term_r = term!{context: #right}?;
            terms.push(&term_l);
            terms.push(&term_r);

            context.open(pred, [&term_l, &term_r])
        }
    };

    println!("opened query");

    dbg!(&terms);

    let mut soln = true;
    while soln {
        println!("getting solution");
        soln = match open_query.next_solution() {
            Ok(next) => next,
            Err(e) => match e {
                PrologError::Failure => false,
                PrologError::Exception => return Err(e)
            }
        }
    }

    dbg!(&terms);

    open_query.cut();

    Ok(())
}
