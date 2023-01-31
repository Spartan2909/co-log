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

fn query_prolog(context: Context<ActivatedEngine>, query: crate::transpiler::Query) -> Result<(), PrologError> {
    let open_query = match query.right {
        None => {
            let pred: CallablePredicate<1> = unsafe {
                CallablePredicate::wrap(
                    swipl_fli::PL_predicate(query.relationship.as_ptr() as *const i8, 1, std::ptr::null())
                )
            };

            let left = query.left;
            context.open(pred, [&term!{context: #left}?])
        },
        Some(right) => {
            let pred: CallablePredicate<2> = unsafe {
                CallablePredicate::wrap(
                    swipl_fli::PL_predicate(query.relationship.as_ptr() as *const i8, 2, std::ptr::null())
                )
            };

            let left = query.left;
            context.open(pred, [&term!{context: #left}?, &term!{context: #right}?])
        }
    };

    Ok(())
}
