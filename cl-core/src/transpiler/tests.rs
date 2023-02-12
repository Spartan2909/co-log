use super::{Identifier, Identifiers, Query};
use crate::{parser, scanner};

fn transpile(source: &str) -> (String, Vec<Query>, Identifiers) {
    super::transpile(
        parser::parse(scanner::scan(source.to_string())).unwrap(),
        None,
    )
}

#[test]
fn fact_unary() {
    assert_eq!(
        transpile("A hamster is a mammal."),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\nl1(l2).\n".to_string(),
            vec![],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "mammal".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("a".to_string(),),
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "hamster".to_string(),
                        pl_name: "l2".to_string(),
                        article: Some("A".to_string(),),
                        preposition: None,
                    },
                ],
                highest_literal: 2,
                highest_variable: 0,
            },
        )
    )
}

#[test]
fn fact_binary() {
    assert_eq!(
        transpile("John is the brother of Jack."),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\nl1(l2, l3).\n".to_string(),
            vec![],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "brother".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("the".to_string(),),
                        preposition: Some("of".to_string(),),
                    },
                    Identifier {
                        cl_name: "john".to_string(),
                        pl_name: "l2".to_string(),
                        article: None,
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "jack".to_string(),
                        pl_name: "l3".to_string(),
                        article: None,
                        preposition: None,
                    },
                ],
                highest_literal: 3,
                highest_variable: 0,
            },
        )
    )
}

#[test]
fn rule_unary() {
    assert_eq!(
        transpile("X is a mammal if X is an animal and X is warm-blooded."),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\nl1(V1) :- (l2(V1), l3(V1)).\n"
                .to_string(),
            vec![],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "mammal".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("a".to_string(),),
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "X".to_string(),
                        pl_name: "V1".to_string(),
                        article: None,
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "animal".to_string(),
                        pl_name: "l2".to_string(),
                        article: Some("an".to_string(),),
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "warm-blooded".to_string(),
                        pl_name: "l3".to_string(),
                        article: None,
                        preposition: None,
                    },
                ],
                highest_literal: 3,
                highest_variable: 1,
            },
        )
    )
}

#[test]
fn rule_binary() {
    assert_eq!(transpile("X is the brother of Y if X is the sibling of Y and X is male."), (
        "style_check(-discontiguous).\neq(X, Y) :- X == Y.\nl1(V1, V2) :- (l2(V1, V2), l3(V1)).\n".to_string(),
        vec![],
        Identifiers {
            identifiers: vec![
                Identifier {
                    cl_name: "eq".to_string(),
                    pl_name: "eq".to_string(),
                    article: None,
                    preposition: None
                },
                Identifier {
                    cl_name: "brother".to_string(),
                    pl_name: "l1".to_string(),
                    article: Some(
                        "the".to_string(),
                    ),
                    preposition: Some(
                        "of".to_string(),
                    ),
                },
                Identifier {
                    cl_name: "X".to_string(),
                    pl_name: "V1".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "Y".to_string(),
                    pl_name: "V2".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "sibling".to_string(),
                    pl_name: "l2".to_string(),
                    article: Some(
                        "the".to_string(),
                    ),
                    preposition: Some(
                        "of".to_string(),
                    ),
                },
                Identifier {
                    cl_name: "male".to_string(),
                    pl_name: "l3".to_string(),
                    article: None,
                    preposition: None,
                },
            ],
            highest_literal: 3,
            highest_variable: 2,
        },
    ))
}

#[test]
fn rule_binary_parentheses() {
    assert_eq!(transpile("B is thing of C if (B is one and C is one) or (B is two and C is two)."), (
        "style_check(-discontiguous).\neq(X, Y) :- X == Y.\nl1(V1, V2) :- ((l2(V1), l2(V2)); (l3(V1), l3(V2))).\n".to_string(),
        vec![],
        Identifiers {
            identifiers: vec![
                Identifier {
                    cl_name: "eq".to_string(),
                    pl_name: "eq".to_string(),
                    article: None,
                    preposition: None
                },
                Identifier {
                    cl_name: "thing".to_string(),
                    pl_name: "l1".to_string(),
                    article: None,
                    preposition: Some(
                        "of".to_string(),
                    ),
                },
                Identifier {
                    cl_name: "B".to_string(),
                    pl_name: "V1".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "C".to_string(),
                    pl_name: "V2".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "one".to_string(),
                    pl_name: "l2".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "two".to_string(),
                    pl_name: "l3".to_string(),
                    article: None,
                    preposition: None,
                },
            ],
            highest_literal: 3,
            highest_variable: 2,
        },
    ))
}

#[test]
fn rule_binary_negation() {
    assert_eq!(transpile("X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y."), (
        "style_check(-discontiguous).\neq(X, Y) :- X == Y.\nl1(V1, V2) :- (l2(V3, V1), (l2(V3, V2), \\+eq(V1, V2))).\n".to_string(),
        vec![],
        Identifiers {
            identifiers: vec![
                Identifier {
                    cl_name: "eq".to_string(),
                    pl_name: "eq".to_string(),
                    article: None,
                    preposition: None
                },
                Identifier {
                    cl_name: "sibling".to_string(),
                    pl_name: "l1".to_string(),
                    article: Some(
                        "the".to_string(),
                    ),
                    preposition: Some(
                        "of".to_string(),
                    ),
                },
                Identifier {
                    cl_name: "X".to_string(),
                    pl_name: "V1".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "Y".to_string(),
                    pl_name: "V2".to_string(),
                    article: None,
                    preposition: None,
                },
                Identifier {
                    cl_name: "parent".to_string(),
                    pl_name: "l2".to_string(),
                    article: Some(
                        "the".to_string(),
                    ),
                    preposition: Some(
                        "of".to_string(),
                    ),
                },
                Identifier {
                    cl_name: "Z".to_string(),
                    pl_name: "V3".to_string(),
                    article: None,
                    preposition: None,
                },
            ],
            highest_literal: 2,
            highest_variable: 3,
        },
    ))
}

#[test]
fn query_literal() {
    assert_eq!(
        transpile("Is a hamster a mammal?"),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\n".to_string(),
            vec![Query {
                relationship: "l1".to_string(),
                left: "l2".to_string(),
                right: None
            }],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "mammal".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("a".to_string(),),
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "hamster".to_string(),
                        pl_name: "l2".to_string(),
                        article: Some("a".to_string(),),
                        preposition: None,
                    },
                ],
                highest_literal: 2,
                highest_variable: 0,
            },
        )
    )
}

#[test]
fn query_literal_literal() {
    assert_eq!(
        transpile("Is John the brother of Jack?"),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\n".to_string(),
            vec![Query {
                left: "l2".to_string(),
                relationship: "l1".to_string(),
                right: Some("l3".to_string())
            }],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "brother".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("the".to_string(),),
                        preposition: Some("of".to_string(),),
                    },
                    Identifier {
                        cl_name: "john".to_string(),
                        pl_name: "l2".to_string(),
                        article: None,
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "jack".to_string(),
                        pl_name: "l3".to_string(),
                        article: None,
                        preposition: None,
                    },
                ],
                highest_literal: 3,
                highest_variable: 0,
            },
        )
    )
}

#[test]
fn query_literal_pronoun() {
    assert_eq!(
        transpile("John is the brother of who?"),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\n".to_string(),
            vec![Query {
                left: "l2".to_string(),
                relationship: "l1".to_string(),
                right: Some("V1".to_string())
            }],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "brother".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("the".to_string(),),
                        preposition: Some("of".to_string(),),
                    },
                    Identifier {
                        cl_name: "john".to_string(),
                        pl_name: "l2".to_string(),
                        article: None,
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "who".to_string(),
                        pl_name: "V1".to_string(),
                        article: None,
                        preposition: None,
                    },
                ],
                highest_literal: 2,
                highest_variable: 1,
            },
        )
    )
}

#[test]
fn query_pronoun_literal() {
    assert_eq!(
        transpile("Who is the brother of Jane?"),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\n".to_string(),
            vec![Query {
                left: "V1".to_string(),
                relationship: "l1".to_string(),
                right: Some("l2".to_string())
            }],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "brother".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("the".to_string(),),
                        preposition: Some("of".to_string(),),
                    },
                    Identifier {
                        cl_name: "who".to_string(),
                        pl_name: "V1".to_string(),
                        article: None,
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "jane".to_string(),
                        pl_name: "l2".to_string(),
                        article: None,
                        preposition: None,
                    },
                ],
                highest_literal: 2,
                highest_variable: 1,
            },
        )
    )
}

#[test]
fn query_pronoun_pronoun() {
    assert_eq!(
        transpile("Who is the sister of who?"),
        (
            "style_check(-discontiguous).\neq(X, Y) :- X == Y.\n".to_string(),
            vec![Query {
                left: "V1".to_string(),
                relationship: "l1".to_string(),
                right: Some("V2".to_string())
            }],
            Identifiers {
                identifiers: vec![
                    Identifier {
                        cl_name: "eq".to_string(),
                        pl_name: "eq".to_string(),
                        article: None,
                        preposition: None
                    },
                    Identifier {
                        cl_name: "sister".to_string(),
                        pl_name: "l1".to_string(),
                        article: Some("the".to_string(),),
                        preposition: Some("of".to_string(),),
                    },
                    Identifier {
                        cl_name: "who".to_string(),
                        pl_name: "V1".to_string(),
                        article: None,
                        preposition: None,
                    },
                    Identifier {
                        cl_name: "who".to_string(),
                        pl_name: "V2".to_string(),
                        article: None,
                        preposition: None,
                    },
                ],
                highest_literal: 1,
                highest_variable: 2,
            },
        )
    )
}
