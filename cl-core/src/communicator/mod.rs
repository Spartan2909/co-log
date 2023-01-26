use swipl::prelude::*;

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
