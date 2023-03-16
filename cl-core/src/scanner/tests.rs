use std::collections::HashSet;

use super::*;

fn test_tokens_equal(left: &str, right: HashSet<Token>) {
    assert_eq!(HashSet::from_iter(scan(String::from(left))), right)
}

#[test]
fn fact_unary() {
    test_tokens_equal(
        "A hamster is a mammal.",
        HashSet::from([
            Token {
                kind: Article,
                lexeme: String::from("A"),
                start: 0,
            },
            Token {
                kind: Literal,
                lexeme: String::from("hamster"),
                start: 2,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 10,
            },
            Token {
                kind: Article,
                lexeme: String::from("a"),
                start: 13,
            },
            Token {
                kind: Literal,
                lexeme: String::from("mammal"),
                start: 15,
            },
            Token {
                kind: FullStop,
                lexeme: String::from("."),
                start: 21,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 22,
            },
        ]),
    )
}

#[test]
fn fact_binary() {
    test_tokens_equal(
        "John is the brother of Jack.",
        HashSet::from([
            Token {
                kind: Literal,
                lexeme: String::from("John"),
                start: 0,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 5,
            },
            Token {
                kind: Article,
                lexeme: String::from("the"),
                start: 8,
            },
            Token {
                kind: Literal,
                lexeme: String::from("brother"),
                start: 12,
            },
            Token {
                kind: Prepostion,
                lexeme: String::from("of"),
                start: 20,
            },
            Token {
                kind: Literal,
                lexeme: String::from("Jack"),
                start: 23,
            },
            Token {
                kind: FullStop,
                lexeme: String::from("."),
                start: 27,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 28,
            },
        ]),
    )
}

#[test]
fn rule_unary() {
    test_tokens_equal(
        "X is a mammal if X is an animal and X is warm-blooded.",
        HashSet::from([
            Token {
                kind: Variable,
                lexeme: String::from("X"),
                start: 0,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 2,
            },
            Token {
                kind: Article,
                lexeme: String::from("a"),
                start: 5,
            },
            Token {
                kind: Literal,
                lexeme: String::from("mammal"),
                start: 7,
            },
            Token {
                kind: If,
                lexeme: String::from("if"),
                start: 14,
            },
            Token {
                kind: Variable,
                lexeme: String::from("X"),
                start: 17,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 19,
            },
            Token {
                kind: Article,
                lexeme: String::from("an"),
                start: 22,
            },
            Token {
                kind: Literal,
                lexeme: String::from("animal"),
                start: 25,
            },
            Token {
                kind: Operator,
                lexeme: String::from("and"),
                start: 32,
            },
            Token {
                kind: Variable,
                lexeme: String::from("X"),
                start: 36,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 38,
            },
            Token {
                kind: Literal,
                lexeme: String::from("warm-blooded"),
                start: 41,
            },
            Token {
                kind: FullStop,
                lexeme: String::from("."),
                start: 53,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 54,
            },
        ]),
    )
}

#[test]
fn rule_binary() {
    test_tokens_equal(
        "X is the brother of Y if X is the sibling of Y and X is male.",
        HashSet::from([
            Token {
                kind: Variable,
                lexeme: String::from("X"),
                start: 0,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 2,
            },
            Token {
                kind: Article,
                lexeme: String::from("the"),
                start: 5,
            },
            Token {
                kind: Literal,
                lexeme: String::from("brother"),
                start: 9,
            },
            Token {
                kind: Prepostion,
                lexeme: String::from("of"),
                start: 17,
            },
            Token {
                kind: Variable,
                lexeme: String::from("Y"),
                start: 20,
            },
            Token {
                kind: If,
                lexeme: String::from("if"),
                start: 22,
            },
            Token {
                kind: Variable,
                lexeme: String::from("X"),
                start: 25,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 27,
            },
            Token {
                kind: Article,
                lexeme: String::from("the"),
                start: 30,
            },
            Token {
                kind: Literal,
                lexeme: String::from("sibling"),
                start: 34,
            },
            Token {
                kind: Prepostion,
                lexeme: String::from("of"),
                start: 42,
            },
            Token {
                kind: Variable,
                lexeme: String::from("Y"),
                start: 45,
            },
            Token {
                kind: Operator,
                lexeme: String::from("and"),
                start: 47,
            },
            Token {
                kind: Variable,
                lexeme: String::from("X"),
                start: 51,
            },
            Token {
                kind: Verb,
                lexeme: String::from("is"),
                start: 53,
            },
            Token {
                kind: Literal,
                lexeme: String::from("male"),
                start: 56,
            },
            Token {
                kind: FullStop,
                lexeme: String::from("."),
                start: 60,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 61,
            },
        ]),
    )
}

#[test]
fn rule_binary_parentheses() {
    test_tokens_equal(
        "B is thing of C if (B is one and C is one) or (B is two and C is two).",
        HashSet::from([
            Token {
                kind: Variable,
                lexeme: "B".to_string(),
                start: 0,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 2,
            },
            Token {
                kind: Literal,
                lexeme: "thing".to_string(),
                start: 5,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 11,
            },
            Token {
                kind: Variable,
                lexeme: "C".to_string(),
                start: 14,
            },
            Token {
                kind: If,
                lexeme: "if".to_string(),
                start: 16,
            },
            Token {
                kind: LeftParen,
                lexeme: "(".to_string(),
                start: 19,
            },
            Token {
                kind: Variable,
                lexeme: "B".to_string(),
                start: 20,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 22,
            },
            Token {
                kind: Literal,
                lexeme: "one".to_string(),
                start: 25,
            },
            Token {
                kind: Operator,
                lexeme: "and".to_string(),
                start: 29,
            },
            Token {
                kind: Variable,
                lexeme: "C".to_string(),
                start: 33,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 35,
            },
            Token {
                kind: Literal,
                lexeme: "one".to_string(),
                start: 38,
            },
            Token {
                kind: RightParen,
                lexeme: ")".to_string(),
                start: 41,
            },
            Token {
                kind: Operator,
                lexeme: "or".to_string(),
                start: 43,
            },
            Token {
                kind: LeftParen,
                lexeme: "(".to_string(),
                start: 46,
            },
            Token {
                kind: Variable,
                lexeme: "B".to_string(),
                start: 47,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 49,
            },
            Token {
                kind: Literal,
                lexeme: "two".to_string(),
                start: 52,
            },
            Token {
                kind: Operator,
                lexeme: "and".to_string(),
                start: 56,
            },
            Token {
                kind: Variable,
                lexeme: "C".to_string(),
                start: 60,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 62,
            },
            Token {
                kind: Literal,
                lexeme: "two".to_string(),
                start: 65,
            },
            Token {
                kind: RightParen,
                lexeme: ")".to_string(),
                start: 68,
            },
            Token {
                kind: FullStop,
                lexeme: ".".to_string(),
                start: 69,
            },
            Token {
                kind: EOF,
                lexeme: "".to_string(),
                start: 70,
            },
        ]),
    )
}

#[test]
fn rule_binary_negation() {
    test_tokens_equal(
        "X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.",
        HashSet::from([
            Token::new(Variable, "X", 0),
            Token::new(Verb, "is", 2),
            Token::new(Article, "the", 5),
            Token::new(Literal, "sibling", 9),
            Token::new(Prepostion, "of", 17),
            Token::new(Variable, "Y", 20),
            Token::new(If, "if", 22),
            Token::new(Variable, "Z", 25),
            Token::new(Verb, "is", 27),
            Token::new(Article, "the", 30),
            Token::new(Literal, "parent", 34),
            Token::new(Prepostion, "of", 41),
            Token::new(Variable, "X", 44),
            Token::new(Operator, "and", 46),
            Token::new(Variable, "Z", 50),
            Token::new(Verb, "is", 52),
            Token::new(Article, "the", 55),
            Token::new(Literal, "parent", 59),
            Token::new(Prepostion, "of", 66),
            Token::new(Variable, "Y", 69),
            Token::new(Operator, "and", 71),
            Token::new(Variable, "X", 75),
            Token::new(Verb, "is", 77),
            Token::new(Not, "not", 80),
            Token::new(Variable, "Y", 84),
            Token::new(FullStop, ".", 85),
            Token::new(EOF, "", 86),
        ]),
    )
}

#[test]
fn query_literal() {
    test_tokens_equal(
        "Is a hamster a mammal?",
        HashSet::from([
            Token {
                kind: Verb,
                lexeme: String::from("Is"),
                start: 0,
            },
            Token {
                kind: Article,
                lexeme: String::from("a"),
                start: 3,
            },
            Token {
                kind: Literal,
                lexeme: String::from("hamster"),
                start: 5,
            },
            Token {
                kind: Article,
                lexeme: String::from("a"),
                start: 13,
            },
            Token {
                kind: Literal,
                lexeme: String::from("mammal"),
                start: 15,
            },
            Token {
                kind: QuestionMark,
                lexeme: String::from("?"),
                start: 21,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 22,
            },
        ]),
    )
}

#[test]
fn query_literal_literal() {
    test_tokens_equal(
        "Is John the brother of Jack?",
        HashSet::from([
            Token {
                kind: Verb,
                lexeme: String::from("Is"),
                start: 0,
            },
            Token {
                kind: Literal,
                lexeme: String::from("John"),
                start: 3,
            },
            Token {
                kind: Article,
                lexeme: String::from("the"),
                start: 8,
            },
            Token {
                kind: Literal,
                lexeme: String::from("brother"),
                start: 12,
            },
            Token {
                kind: Prepostion,
                lexeme: String::from("of"),
                start: 20,
            },
            Token {
                kind: Literal,
                lexeme: String::from("Jack"),
                start: 23,
            },
            Token {
                kind: QuestionMark,
                lexeme: String::from("?"),
                start: 27,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 28,
            },
        ]),
    )
}

#[test]
fn query_literal_pronoun() {
    test_tokens_equal(
        "John is the brother of who?",
        HashSet::from([
            Token::new(Literal, "John", 0),
            Token::new(Verb, "is", 5),
            Token::new(Article, "the", 8),
            Token::new(Literal, "brother", 12),
            Token::new(Prepostion, "of", 20),
            Token {
                kind: Pronoun,
                lexeme: String::from("who"),
                start: 23,
            },
            Token {
                kind: QuestionMark,
                lexeme: String::from("?"),
                start: 26,
            },
            Token {
                kind: EOF,
                lexeme: String::from(""),
                start: 27,
            },
        ]),
    )
}

#[test]
fn query_pronoun_literal() {
    test_tokens_equal(
        "Who is the brother of Jane?",
        HashSet::from([
            Token::new(Pronoun, "Who", 0),
            Token::new(Verb, "is", 4),
            Token::new(Article, "the", 7),
            Token::new(Literal, "brother", 11),
            Token::new(Prepostion, "of", 19),
            Token::new(Literal, "Jane", 22),
            Token::new(QuestionMark, "?", 26),
            Token::new(EOF, "", 27),
        ]),
    )
}

#[test]
fn query_pronoun_pronoun() {
    test_tokens_equal(
        "Who is the sister of who?",
        HashSet::from([
            Token::new(Pronoun, "Who", 0),
            Token::new(Verb, "is", 4),
            Token::new(Article, "the", 7),
            Token::new(Literal, "sister", 11),
            Token::new(Prepostion, "of", 18),
            Token::new(Pronoun, "who", 21),
            Token::new(QuestionMark, "?", 24),
            Token::new(EOF, "", 25),
        ]),
    )
}

#[test]
fn program_1() {
    test_tokens_equal("A hamster is an animal. A hamster is warm-blooded. X is a mammal if X is an animal and X is warm-blooded.", HashSet::from([
        Token {
            kind: Article,
            lexeme: "A".to_string(),
            start: 0,
        },
        Token {
            kind: Literal,
            lexeme: "hamster".to_string(),
            start: 2,
        },
        Token {
            kind: Verb,
            lexeme: "is".to_string(),
            start: 10,
        },
        Token {
            kind: Article,
            lexeme: "an".to_string(),
            start: 13,
        },
        Token {
            kind: Literal,
            lexeme: "animal".to_string(),
            start: 16,
        },
        Token {
            kind: FullStop,
            lexeme: ".".to_string(),
            start: 22,
        },
        Token {
            kind: Article,
            lexeme: "A".to_string(),
            start: 24,
        },
        Token {
            kind: Literal,
            lexeme: "hamster".to_string(),
            start: 26,
        },
        Token {
            kind: Verb,
            lexeme: "is".to_string(),
            start: 34,
        },
        Token {
            kind: Literal,
            lexeme: "warm-blooded".to_string(),
            start: 37,
        },
        Token {
            kind: FullStop,
            lexeme: ".".to_string(),
            start: 49,
        },
        Token {
            kind: Variable,
            lexeme: "X".to_string(),
            start: 51,
        },
        Token {
            kind: Verb,
            lexeme: "is".to_string(),
            start: 53,
        },
        Token {
            kind: Article,
            lexeme: "a".to_string(),
            start: 56,
        },
        Token {
            kind: Literal,
            lexeme: "mammal".to_string(),
            start: 58,
        },
        Token {
            kind: If,
            lexeme: "if".to_string(),
            start: 65,
        },
        Token {
            kind: Variable,
            lexeme: "X".to_string(),
            start: 68,
        },
        Token {
            kind: Verb,
            lexeme: "is".to_string(),
            start: 70,
        },
        Token {
            kind: Article,
            lexeme: "an".to_string(),
            start: 73,
        },
        Token {
            kind: Literal,
            lexeme: "animal".to_string(),
            start: 76,
        },
        Token {
            kind: Operator,
            lexeme: "and".to_string(),
            start: 83,
        },
        Token {
            kind: Variable,
            lexeme: "X".to_string(),
            start: 87,
        },
        Token {
            kind: Verb,
            lexeme: "is".to_string(),
            start: 89,
        },
        Token {
            kind: Literal,
            lexeme: "warm-blooded".to_string(),
            start: 92,
        },
        Token {
            kind: FullStop,
            lexeme: ".".to_string(),
            start: 104,
        },
        Token {
            kind: EOF,
            lexeme: "".to_string(),
            start: 105,
        },
    ]))
}

#[test]
fn program_2() {
    test_tokens_equal(
        "John is the parent of Jack. John is the parent of Jane. X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.",
        HashSet::from([
            Token {
                kind: Literal,
                lexeme: "John".to_string(),
                start: 0,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 5,
            },
            Token {
                kind: Article,
                lexeme: "the".to_string(),
                start: 8,
            },
            Token {
                kind: Literal,
                lexeme: "parent".to_string(),
                start: 12,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 19,
            },
            Token {
                kind: Literal,
                lexeme: "Jack".to_string(),
                start: 22,
            },
            Token {
                kind: FullStop,
                lexeme: ".".to_string(),
                start: 26,
            },
            Token {
                kind: Literal,
                lexeme: "John".to_string(),
                start: 28,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 33,
            },
            Token {
                kind: Article,
                lexeme: "the".to_string(),
                start: 36,
            },
            Token {
                kind: Literal,
                lexeme: "parent".to_string(),
                start: 40,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 47,
            },
            Token {
                kind: Literal,
                lexeme: "Jane".to_string(),
                start: 50,
            },
            Token {
                kind: FullStop,
                lexeme: ".".to_string(),
                start: 54,
            },
            Token {
                kind: Variable,
                lexeme: "X".to_string(),
                start: 56,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 58,
            },
            Token {
                kind: Article,
                lexeme: "the".to_string(),
                start: 61,
            },
            Token {
                kind: Literal,
                lexeme: "sibling".to_string(),
                start: 65,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 73,
            },
            Token {
                kind: Variable,
                lexeme: "Y".to_string(),
                start: 76,
            },
            Token {
                kind: If,
                lexeme: "if".to_string(),
                start: 78,
            },
            Token {
                kind: Variable,
                lexeme: "Z".to_string(),
                start: 81,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 83,
            },
            Token {
                kind: Article,
                lexeme: "the".to_string(),
                start: 86,
            },
            Token {
                kind: Literal,
                lexeme: "parent".to_string(),
                start: 90,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 97,
            },
            Token {
                kind: Variable,
                lexeme: "X".to_string(),
                start: 100,
            },
            Token {
                kind: Operator,
                lexeme: "and".to_string(),
                start: 102,
            },
            Token {
                kind: Variable,
                lexeme: "Z".to_string(),
                start: 106,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 108,
            },
            Token {
                kind: Article,
                lexeme: "the".to_string(),
                start: 111,
            },
            Token {
                kind: Literal,
                lexeme: "parent".to_string(),
                start: 115,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 122,
            },
            Token {
                kind: Variable,
                lexeme: "Y".to_string(),
                start: 125,
            },
            Token {
                kind: Operator,
                lexeme: "and".to_string(),
                start: 127,
            },
            Token {
                kind: Variable,
                lexeme: "X".to_string(),
                start: 131,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 133,
            },
            Token {
                kind: Not,
                lexeme: "not".to_string(),
                start: 136,
            },
            Token {
                kind: Variable,
                lexeme: "Y".to_string(),
                start: 140,
            },
            Token {
                kind: FullStop,
                lexeme: ".".to_string(),
                start: 141,
            },
            Token {
                kind: EOF,
                lexeme: "".to_string(),
                start: 142,
            },
        ]),
    );
}
