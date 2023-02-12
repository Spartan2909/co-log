use super::{
    ast::{IdenType::*, OperatorType::*, StmtType::*, *},
    *,
};
use crate::scanner::scan;
use std::collections::HashSet;

fn test_trees_equal(left: &str, right: HashSet<ast::Stmt>) {
    let left_tmp = parse(scan(String::from(left))).unwrap();
    assert_eq!(HashSet::from_iter(left_tmp), right)
}

#[test]
fn fact_unary() {
    test_trees_equal(
        "A hamster is a mammal.",
        HashSet::from([Stmt {
            kind: Fact,
            left: Identifier {
                kind: Literal,
                lexeme: "hamster".to_string(),
                article: Some("A".to_string()),
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "mammal".to_string(),
                article: Some("a".to_string()),
                preposition: None,
            },
            right: None,
            condition: None,
        }]),
    )
}

#[test]
fn fact_binary() {
    test_trees_equal(
        "John is the brother of Jack.",
        HashSet::from([Stmt {
            kind: Fact,
            left: Identifier {
                kind: Literal,
                lexeme: "John".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "brother".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Literal,
                lexeme: "Jack".to_string(),
                article: None,
                preposition: None,
            }),
            condition: None,
        }]),
    )
}

#[test]
fn rule_unary() {
    test_trees_equal(
        "X is a mammal if X is an animal and X is warm-blooded.",
        HashSet::from([Stmt {
            kind: Rule,
            left: Identifier {
                kind: Variable,
                lexeme: "X".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "mammal".to_string(),
                article: Some("a".to_string()),
                preposition: None,
            },
            right: None,
            condition: Some(Clause::Operator {
                op_type: And,
                left: Box::new(Clause::Simple {
                    negated: false,
                    left: Identifier {
                        kind: Variable,
                        lexeme: "X".to_string(),
                        article: None,
                        preposition: None,
                    },
                    relationship: Identifier {
                        kind: Literal,
                        lexeme: "animal".to_string(),
                        article: Some("an".to_string()),
                        preposition: None,
                    },
                    right: None,
                }),
                right: Box::new(Clause::Simple {
                    negated: false,
                    left: Identifier {
                        kind: Variable,
                        lexeme: "X".to_string(),
                        article: None,
                        preposition: None,
                    },
                    relationship: Identifier {
                        kind: Literal,
                        lexeme: "warm-blooded".to_string(),
                        article: None,
                        preposition: None,
                    },
                    right: None,
                }),
            }),
        }]),
    )
}

#[test]
fn rule_binary() {
    test_trees_equal(
        "X is the brother of Y if X is the sibling of Y and X is male.",
        HashSet::from([Stmt {
            kind: Rule,
            left: Identifier {
                kind: Variable,
                lexeme: "X".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "brother".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Variable,
                lexeme: "Y".to_string(),
                article: None,
                preposition: None,
            }),
            condition: Some(Clause::Operator {
                op_type: And,
                left: Box::new(Clause::Simple {
                    negated: false,
                    left: Identifier {
                        kind: Variable,
                        lexeme: "X".to_string(),
                        article: None,
                        preposition: None,
                    },
                    relationship: Identifier {
                        kind: Literal,
                        lexeme: "sibling".to_string(),
                        article: Some("the".to_string()),
                        preposition: Some("of".to_string()),
                    },
                    right: Some(Identifier {
                        kind: Variable,
                        lexeme: "Y".to_string(),
                        article: None,
                        preposition: None,
                    }),
                }),
                right: Box::new(Clause::Simple {
                    negated: false,
                    left: Identifier {
                        kind: Variable,
                        lexeme: "X".to_string(),
                        article: None,
                        preposition: None,
                    },
                    relationship: Identifier {
                        kind: Literal,
                        lexeme: "male".to_string(),
                        article: None,
                        preposition: None,
                    },
                    right: None,
                }),
            }),
        }]),
    )
}

#[test]
fn rule_binary_parentheses() {
    test_trees_equal(
        "B is thing of C if (B is one and C is one) or (B is two and C is two).",
        HashSet::from([Stmt {
            kind: Rule,
            left: Identifier {
                kind: Variable,
                lexeme: "B".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "thing".to_string(),
                article: None,
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Variable,
                lexeme: "C".to_string(),
                article: None,
                preposition: None,
            }),
            condition: Some(Clause::Operator {
                op_type: Or,
                left: Box::new(Clause::Operator {
                    op_type: And,
                    left: Box::new(Clause::Simple {
                        negated: false,
                        left: Identifier {
                            kind: Variable,
                            lexeme: "B".to_string(),
                            article: None,
                            preposition: None,
                        },
                        relationship: Identifier {
                            kind: Literal,
                            lexeme: "one".to_string(),
                            article: None,
                            preposition: None,
                        },
                        right: None,
                    }),
                    right: Box::new(Clause::Simple {
                        negated: false,
                        left: Identifier {
                            kind: Variable,
                            lexeme: "C".to_string(),
                            article: None,
                            preposition: None,
                        },
                        relationship: Identifier {
                            kind: Literal,
                            lexeme: "one".to_string(),
                            article: None,
                            preposition: None,
                        },
                        right: None,
                    }),
                }),
                right: Box::new(Clause::Operator {
                    op_type: And,
                    left: Box::new(Clause::Simple {
                        negated: false,
                        left: Identifier {
                            kind: Variable,
                            lexeme: "B".to_string(),
                            article: None,
                            preposition: None,
                        },
                        relationship: Identifier {
                            kind: Literal,
                            lexeme: "two".to_string(),
                            article: None,
                            preposition: None,
                        },
                        right: None,
                    }),
                    right: Box::new(Clause::Simple {
                        negated: false,
                        left: Identifier {
                            kind: Variable,
                            lexeme: "C".to_string(),
                            article: None,
                            preposition: None,
                        },
                        relationship: Identifier {
                            kind: Literal,
                            lexeme: "two".to_string(),
                            article: None,
                            preposition: None,
                        },
                        right: None,
                    }),
                }),
            }),
        }]),
    )
}

#[test]
fn rule_binary_negation() {
    test_trees_equal(
        "X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.",
        HashSet::from([Stmt {
            kind: Rule,
            left: Identifier {
                kind: Variable,
                lexeme: "X".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "sibling".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Variable,
                lexeme: "Y".to_string(),
                article: None,
                preposition: None,
            }),
            condition: Some(Clause::Operator {
                op_type: And,
                left: Box::new(Clause::Simple {
                    negated: false,
                    left: Identifier {
                        kind: Variable,
                        lexeme: "Z".to_string(),
                        article: None,
                        preposition: None,
                    },
                    relationship: Identifier {
                        kind: Literal,
                        lexeme: "parent".to_string(),
                        article: Some("the".to_string()),
                        preposition: Some("of".to_string()),
                    },
                    right: Some(Identifier {
                        kind: Variable,
                        lexeme: "X".to_string(),
                        article: None,
                        preposition: None,
                    }),
                }),
                right: Box::new(Clause::Operator {
                    op_type: And,
                    left: Box::new(Clause::Simple {
                        negated: false,
                        left: Identifier {
                            kind: Variable,
                            lexeme: "Z".to_string(),
                            article: None,
                            preposition: None,
                        },
                        relationship: Identifier {
                            kind: Literal,
                            lexeme: "parent".to_string(),
                            article: Some("the".to_string()),
                            preposition: Some("of".to_string()),
                        },
                        right: Some(Identifier {
                            kind: Variable,
                            lexeme: "Y".to_string(),
                            article: None,
                            preposition: None,
                        }),
                    }),
                    right: Box::new(Clause::Simple {
                        negated: true,
                        left: Identifier {
                            kind: Variable,
                            lexeme: "X".to_string(),
                            article: None,
                            preposition: None,
                        },
                        relationship: Identifier {
                            kind: Literal,
                            lexeme: "eq".to_string(),
                            article: None,
                            preposition: None,
                        },
                        right: Some(Identifier {
                            kind: Variable,
                            lexeme: "Y".to_string(),
                            article: None,
                            preposition: None,
                        }),
                    }),
                }),
            }),
        }]),
    )
}

#[test]
fn query_literal() {
    test_trees_equal(
        "Is a hamster a mammal?",
        HashSet::from([Stmt {
            kind: Query,
            left: Identifier {
                kind: Literal,
                lexeme: "hamster".to_string(),
                article: Some("a".to_string()),
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "mammal".to_string(),
                article: Some("a".to_string()),
                preposition: None,
            },
            right: None,
            condition: None,
        }]),
    )
}

#[test]
fn query_literal_literal() {
    test_trees_equal(
        "Is John the brother of Jack?",
        HashSet::from([Stmt {
            kind: Query,
            left: Identifier {
                kind: Literal,
                lexeme: "John".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "brother".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Literal,
                lexeme: "Jack".to_string(),
                article: None,
                preposition: None,
            }),
            condition: None,
        }]),
    )
}

#[test]
fn query_literal_pronoun() {
    test_trees_equal(
        "John is the brother of who?",
        HashSet::from([Stmt {
            kind: Query,
            left: Identifier {
                kind: Literal,
                lexeme: "John".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "brother".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Pronoun,
                lexeme: "who".to_string(),
                article: None,
                preposition: None,
            }),
            condition: None,
        }]),
    )
}

#[test]
fn query_pronoun_literal() {
    test_trees_equal(
        "Who is the brother of Jane?",
        HashSet::from([Stmt {
            kind: Query,
            left: Identifier {
                kind: Pronoun,
                lexeme: "Who".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "brother".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Literal,
                lexeme: "Jane".to_string(),
                article: None,
                preposition: None,
            }),
            condition: None,
        }]),
    )
}

#[test]
fn query_pronoun_pronoun() {
    test_trees_equal(
        "Who is the sister of who?",
        HashSet::from([Stmt {
            kind: Query,
            left: Identifier {
                kind: Pronoun,
                lexeme: "Who".to_string(),
                article: None,
                preposition: None,
            },
            relationship: Identifier {
                kind: Literal,
                lexeme: "sister".to_string(),
                article: Some("the".to_string()),
                preposition: Some("of".to_string()),
            },
            right: Some(Identifier {
                kind: Pronoun,
                lexeme: "who".to_string(),
                article: None,
                preposition: None,
            }),
            condition: None,
        }]),
    )
}
