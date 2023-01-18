use super::*;
use std::collections::HashSet;

fn test_tokens_equal(left: &str, right: HashSet<Token>) {
    assert_eq!(HashSet::from_iter(scan(String::from(left))), right)
}

#[test]
fn fact_unary() {
    test_tokens_equal("A hamster is a mammal.",
        HashSet::from([
            Token { kind: Article, lexeme: String::from("A"), start: 0, length: 1 },
            Token { kind: Literal, lexeme: String::from("hamster"), start: 2, length: 7 },
            Token { kind: Verb, lexeme: String::from("is"), start: 10, length: 2 },
            Token { kind: Article, lexeme: String::from("a"), start: 13, length: 1 },
            Token { kind: Literal, lexeme: String::from("mammal"), start: 15, length: 6 },
            Token { kind: FullStop, lexeme: String::from("."), start: 21, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 22, length: 0 }
        ])
    )
}

#[test]
fn fact_binary() {
    test_tokens_equal("John is the brother of Jack.",
        HashSet::from([
            Token { kind: Literal, lexeme: String::from("John"), start: 0, length: 4 },
            Token { kind: Verb, lexeme: String::from("is"), start: 5, length: 2 },
            Token { kind: Article, lexeme: String::from("the"), start: 8, length: 3 },
            Token { kind: Literal, lexeme: String::from("brother"), start: 12, length: 7 },
            Token { kind: Prepostion, lexeme: String::from("of"), start: 20, length: 2 },
            Token { kind: Literal, lexeme: String::from("Jack"), start: 23, length: 4 },
            Token { kind: FullStop, lexeme: String::from("."), start: 27, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 28, length: 0 }
        ])
    )
}

#[test]
fn rule_unary() {
    test_tokens_equal("X is a mammal if X is an animal and X is warm-blooded.",
        HashSet::from([
            Token { kind: Variable, lexeme: String::from("X"), start: 0, length: 1 },
            Token { kind: Verb, lexeme: String::from("is"), start: 2, length: 2 },
            Token { kind: Article, lexeme: String::from("a"), start: 5, length: 1 },
            Token { kind: Literal, lexeme: String::from("mammal"), start: 7, length: 6 },
            Token { kind: If, lexeme: String::from("if"), start: 14, length: 2 },
            Token { kind: Variable, lexeme: String::from("X"), start: 17, length: 1 },
            Token { kind: Verb, lexeme: String::from("is"), start: 19, length: 2 },
            Token { kind: Article, lexeme: String::from("an"), start: 22, length: 2 },
            Token { kind: Literal, lexeme: String::from("animal"), start: 25, length: 6 },
            Token { kind: Operator, lexeme: String::from("and"), start: 32, length: 3 },
            Token { kind: Variable, lexeme: String::from("X"), start: 36, length: 1 },
            Token { kind: Verb, lexeme: String::from("is"), start: 38, length: 2 },
            Token { kind: Literal, lexeme: String::from("warm-blooded"), start: 41, length: 12 },
            Token { kind: FullStop, lexeme: String::from("."), start: 53, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 54, length: 0 }
        ])
    )
}

#[test]
fn rule_binary() {
    test_tokens_equal("X is the brother of Y if X is the sibling of Y and X is male.",
        HashSet::from([
            Token { kind: Variable, lexeme: String::from("X"), start: 0, length: 1 },
            Token { kind: Verb, lexeme: String::from("is"), start: 2, length: 2 },
            Token { kind: Article, lexeme: String::from("the"), start: 5, length: 3 },
            Token { kind: Literal, lexeme: String::from("brother"), start: 9, length: 7 },
            Token { kind: Prepostion, lexeme: String::from("of"), start: 17, length: 2 },
            Token { kind: Variable, lexeme: String::from("Y"), start: 20, length: 1 },
            Token { kind: If, lexeme: String::from("if"), start: 22, length: 2 },
            Token { kind: Variable, lexeme: String::from("X"), start: 25, length: 1 },
            Token { kind: Verb, lexeme: String::from("is"), start: 27, length: 2 },
            Token { kind: Article, lexeme: String::from("the"), start: 30, length: 3 },
            Token { kind: Literal, lexeme: String::from("sibling"), start: 34, length: 7 },
            Token { kind: Prepostion, lexeme: String::from("of"), start: 42, length: 2 },
            Token { kind: Variable, lexeme: String::from("Y"), start: 45, length: 1 },
            Token { kind: Operator, lexeme: String::from("and"), start: 47, length: 3 },
            Token { kind: Variable, lexeme: String::from("X"), start: 51, length: 1 },
            Token { kind: Verb, lexeme: String::from("is"), start: 53, length: 2 },
            Token { kind: Literal, lexeme: String::from("male"), start: 56, length: 4 },
            Token { kind: FullStop, lexeme: String::from("."), start: 60, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 61, length: 0 }
        ])
    )
}

#[test]
fn rule_binary_parentheses() {
    test_tokens_equal("B is thing of C if (B is one and C is one) or (B is two and C is two).",
        HashSet::from([
            Token {
                kind: Variable,
                lexeme: "B".to_string(),
                start: 0,
                length: 1,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 2,
                length: 2,
            },
            Token {
                kind: Literal,
                lexeme: "thing".to_string(),
                start: 5,
                length: 5,
            },
            Token {
                kind: Prepostion,
                lexeme: "of".to_string(),
                start: 11,
                length: 2,
            },
            Token {
                kind: Variable,
                lexeme: "C".to_string(),
                start: 14,
                length: 1,
            },
            Token {
                kind: If,
                lexeme: "if".to_string(),
                start: 16,
                length: 2,
            },
            Token {
                kind: LeftParen,
                lexeme: "(".to_string(),
                start: 19,
                length: 1,
            },
            Token {
                kind: Variable,
                lexeme: "B".to_string(),
                start: 20,
                length: 1,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 22,
                length: 2,
            },
            Token {
                kind: Literal,
                lexeme: "one".to_string(),
                start: 25,
                length: 3,
            },
            Token {
                kind: Operator,
                lexeme: "and".to_string(),
                start: 29,
                length: 3,
            },
            Token {
                kind: Variable,
                lexeme: "C".to_string(),
                start: 33,
                length: 1,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 35,
                length: 2,
            },
            Token {
                kind: Literal,
                lexeme: "one".to_string(),
                start: 38,
                length: 3,
            },
            Token {
                kind: RightParen,
                lexeme: ")".to_string(),
                start: 41,
                length: 1,
            },
            Token {
                kind: Operator,
                lexeme: "or".to_string(),
                start: 43,
                length: 2,
            },
            Token {
                kind: LeftParen,
                lexeme: "(".to_string(),
                start: 46,
                length: 1,
            },
            Token {
                kind: Variable,
                lexeme: "B".to_string(),
                start: 47,
                length: 1,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 49,
                length: 2,
            },
            Token {
                kind: Literal,
                lexeme: "two".to_string(),
                start: 52,
                length: 3,
            },
            Token {
                kind: Operator,
                lexeme: "and".to_string(),
                start: 56,
                length: 3,
            },
            Token {
                kind: Variable,
                lexeme: "C".to_string(),
                start: 60,
                length: 1,
            },
            Token {
                kind: Verb,
                lexeme: "is".to_string(),
                start: 62,
                length: 2,
            },
            Token {
                kind: Literal,
                lexeme: "two".to_string(),
                start: 65,
                length: 3,
            },
            Token {
                kind: RightParen,
                lexeme: ")".to_string(),
                start: 68,
                length: 1,
            },
            Token {
                kind: FullStop,
                lexeme: ".".to_string(),
                start: 69,
                length: 1,
            },
            Token {
                kind: EOF,
                lexeme: "".to_string(),
                start: 70,
                length: 0,
            },
        ])
    )
}

#[test]
fn rule_binary_negation() {
    test_tokens_equal("X is the sibling of Y if Z is the parent of X and Z is the parent of Y and X is not Y.",
        HashSet::from([
            Token::new(Variable, "X", 0, 1),
            Token::new(Verb, "is", 2, 2),
            Token::new(Article, "the", 5, 3),
            Token::new(Literal, "sibling", 9, 7),
            Token::new(Prepostion, "of", 17, 2),
            Token::new(Variable, "Y", 20, 1),
            Token::new(If, "if", 22, 2),
            Token::new(Variable, "Z",  25, 1 ),
            Token::new(Verb, "is", 27, 2),
            Token::new(Article, "the", 30, 3),
            Token::new(Literal, "parent", 34, 6),
            Token::new(Prepostion, "of", 41, 2),
            Token::new(Variable, "X", 44, 1),
            Token::new(Operator, "and", 46, 3),
            Token::new(Variable, "Z", 50, 1),
            Token::new(Verb, "is", 52, 2),
            Token::new(Article, "the", 55, 3),
            Token::new(Literal, "parent", 59, 6),
            Token::new(Prepostion, "of", 66, 2),
            Token::new(Variable, "Y", 69, 1),
            Token::new(Operator, "and", 71, 3),
            Token::new(Variable, "X", 75, 1),
            Token::new(Verb, "is", 77, 2),
            Token::new(Not, "not", 80, 3),
            Token::new(Variable, "Y", 84, 1),
            Token::new(FullStop, ".", 85, 1),
            Token::new(EOF, "", 86, 0)
        ])
    )
}


#[test]
fn query_literal() {
    test_tokens_equal("Is a hamster a mammal?",
        HashSet::from([
            Token { kind: Verb, lexeme: String::from("Is"), start: 0, length: 2 },
            Token { kind: Article, lexeme: String::from("a"), start: 3, length: 1 },
            Token { kind: Literal, lexeme: String::from("hamster"), start: 5, length: 7 },
            Token { kind: Article, lexeme: String::from("a"), start: 13, length: 1 },
            Token { kind: Literal, lexeme: String::from("mammal"), start: 15, length: 6 },
            Token { kind: QuestionMark, lexeme: String::from("?"), start: 21, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 22, length: 0 }
        ])
    )
}

#[test]
fn query_literal_literal() {
    test_tokens_equal("Is John the brother of Jack?",
        HashSet::from([
            Token { kind: Verb, lexeme: String::from("Is"), start: 0, length: 2 },
            Token { kind: Literal, lexeme: String::from("John"), start: 3, length: 4 },
            Token { kind: Article, lexeme: String::from("the"), start: 8, length: 3 },
            Token { kind: Literal, lexeme: String::from("brother"), start: 12, length: 7 },
            Token { kind: Prepostion, lexeme: String::from("of"), start: 20, length: 2 },
            Token { kind: Literal, lexeme: String::from("Jack"), start: 23, length: 4 },
            Token { kind: QuestionMark, lexeme: String::from("?"), start: 27, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 28, length: 0 }
        ])
    )
}

#[test]
fn query_literal_pronoun() {
    test_tokens_equal("John is the brother of who?",
        HashSet::from([
            Token::new(Literal, "John", 0, 4),
            Token::new(Verb, "is", 5, 2),
            Token::new(Article, "the", 8, 3),
            Token::new(Literal, "brother", 12, 7),
            Token::new(Prepostion,"of", 20, 2),
            Token { kind: Pronoun, lexeme: String::from("who"), start: 23, length: 3 },
            Token { kind: QuestionMark, lexeme: String::from("?"), start: 26, length: 1 },
            Token { kind: EOF, lexeme: String::from(""), start: 27, length: 0 }
        ])
    )
}

#[test]
fn query_pronoun_literal() {
    test_tokens_equal("Who is the brother of Jane?",
        HashSet::from([
            Token::new(Pronoun, "Who", 0, 3),
            Token::new(Verb, "is", 4, 2),
            Token::new(Article, "the", 7, 3),
            Token::new(Literal, "brother", 11, 7),
            Token::new(Prepostion, "of", 19, 2),
            Token::new(Literal, "Jane", 22, 4),
            Token::new(QuestionMark, "?", 26, 1),
            Token::new(EOF, "", 27, 0)
        ])
    )
}

#[test]
fn query_pronoun_pronoun() {
    test_tokens_equal("Who is the sister of who?",
        HashSet::from([
            Token::new(Pronoun, "Who", 0, 3),
            Token::new(Verb, "is", 4, 2),
            Token::new(Article, "the", 7, 3),
            Token::new(Literal, "sister", 11, 6),
            Token::new(Prepostion, "of", 18, 2),
            Token::new(Pronoun, "who", 21, 3),
            Token::new(QuestionMark, "?", 24, 1),
            Token::new(EOF, "", 25, 0)
        ])
    )
}
