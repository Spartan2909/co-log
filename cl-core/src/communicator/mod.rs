use swipl::prelude::*;
//use swipl_fli;

pub fn start_prolog(source: &str) -> PrologResult<Context<ActivatedEngine>> {
    let mut activation_attempts = 0_u8;
    let activation = loop {
        match initialize_swipl() {
            None => {
                if activation_attempts < 10 {
                    activation_attempts += 1;
                } else {
                    panic!("failed to initialise Prolog");
                }
            }
            Some(activation) => break activation,
        }
    };

    let context: Context<_> = activation.into();

    let consult = pred!(consult/1);

    let location = crate::remove_path_prefix(source);
    let term = term! { context: #location }?;

    context.call_once(consult, [&term])?;

    Ok(context)
}

pub fn query_prolog(
    context: &Context<ActivatedEngine>,
    query: crate::transpiler::Query,
) -> PrologResult<bool> {
    let mut terms = Vec::new();
    let term;
    let term_l;
    let term_r;
    let relationship = query.relationship.as_str();

    let module = Module::new("user");

    let open_query = match query.right {
        None => {
            let pred: CallablePredicate<1> =
                CallablePredicate::new(Predicate::new(Functor::new(relationship, 1), module))
                    .unwrap();

            let left = query.left;
            //term = term!{context: #left}?;
            term = context.term_from_string(&left)?;
            terms.push(&term);

            context.open(pred, [&term])
        }
        Some(right) => {
            let pred: CallablePredicate<2> =
                CallablePredicate::new(Predicate::new(Functor::new(relationship, 2), module))
                    .unwrap();

            let left = query.left;
            //term_l = term!{context: #left}?;
            term_l = context.term_from_string(&left)?;
            //term_r = term!{context: #right}?;
            term_r = context.term_from_string(&right)?;
            terms.push(&term_l);
            terms.push(&term_r);

            context.open(pred, [&term_l, &term_r])
        }
    };

    println!("opened query");

    dbg!(&terms);

    let mut next = true;
    let mut soln = false;
    while next {
        println!("getting solution");
        next = match dbg!(open_query.next_solution()) {
            Ok(not_last_soln) => {
                soln = true;
                not_last_soln
            }
            Err(e) => match e {
                PrologError::Failure => false,
                PrologError::Exception => return Err(e),
            },
        };
        //soln = soln || next;
        dbg!(
            terms[0].get::<String>(),
            terms[0].get::<Vec<u8>>(),
            terms[0].is_var(),
            terms[0].is_atom(),
            terms[0].is_string()
        );
    }

    dbg!(&terms[0].get::<String>(), &terms[0].is_var(), soln);

    open_query.cut();

    Ok(soln)
}
