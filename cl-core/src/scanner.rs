use self::TokenType::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single character tokens
    FullStop, QuestionMark, LeftParen, RightParen,

    // Reserved words
    Article, Operator, Prepostion, Verb, If, Pronoun,
    
    // Identifiers
    Literal, Variable,

    EOF
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    start: usize,
    length: usize,
}

impl Token {
    fn new(kind: TokenType, lexeme: &str, start: usize, length: usize) -> Self {
        Token { kind, lexeme: String::from(lexeme), start, length }
    }
}

fn valid_iden(c: char) -> bool {
    c.is_alphabetic() || c == '-' || c == '_'
}

fn get_word(source: &str, start: usize) -> String {
    let mut end = start;
    while end < source.len() && valid_iden(source.chars().nth(end).unwrap()) {
        end += 1;
    }

    String::from(&source[start..end])
}

pub fn scan(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut i: usize = 0;
    while i < source.len() {
        let c = source.chars().nth(i).unwrap();
        match c {
            c if c.is_whitespace() => {i += 1; continue},
            '(' => tokens.push(Token::new(LeftParen, "(", i, 1)),
            ')' => tokens.push(Token::new(RightParen, ")", i, 1)),
            '.' => tokens.push(Token::new(FullStop, ".", i, 1)),
            '?' => tokens.push(Token::new(QuestionMark, "?", i, 1)),
            c if c.is_alphabetic() => {
                let lexeme = get_word(&source, i);
                let length = lexeme.len();
                let kind = match lexeme.to_lowercase().as_str() {
                    "a" | "an" | "the" => Article,
                    "and" | "or" => Operator,
                    "of" | "to" => Prepostion,
                    "is" | "are" => Verb,
                    "if" => If,
                    "who" | "what" => Pronoun,
                    _ => {
                        if lexeme.chars().nth(length-1).unwrap().is_lowercase() {
                            Literal
                        } else {
                            Variable
                        }
                    }
                };
                tokens.push(Token::new(kind, &lexeme, i, length));

                i += length;
                continue
            },
            _ => todo!("unrecognised character"),
        }

        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, "", i, 0));
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn fact_unary() {
        assert_eq!(HashSet::from_iter(scan(String::from("A hamster is a mammal."))),
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
        assert_eq!(HashSet::from_iter(scan(String::from("John is the brother of Jack."))),
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
        assert_eq!(HashSet::from_iter(scan(String::from("X is a mammal if X is an animal and X is warm-blooded."))),
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
        assert_eq!(HashSet::from_iter(scan(String::from("X is the brother of Y if X is the sibling of Y and X is male."))),
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
    fn query_literal() {
        assert_eq!(HashSet::from_iter(scan(String::from("Is a hamster a mammal?"))),
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
        assert_eq!(HashSet::from_iter(scan(String::from("Is John the brother of Jack?"))),
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
        assert_eq!(HashSet::from_iter(scan(String::from("John is the brother of who?"))),
            HashSet::from([
                Token { kind: Literal, lexeme: String::from("John"), start: 0, length: 4 },
                Token { kind: Verb, lexeme: String::from("is"), start: 5, length: 2 },
                Token { kind: Article, lexeme: String::from("the"), start: 8, length: 3 },
                Token { kind: Literal, lexeme: String::from("brother"), start: 12, length: 7 },
                Token { kind: Prepostion, lexeme: String::from("of"), start: 20, length: 2 },
                Token { kind: Pronoun, lexeme: String::from("who"), start: 23, length: 3 },
                Token { kind: QuestionMark, lexeme: String::from("?"), start: 26, length: 1 },
                Token { kind: EOF, lexeme: String::from(""), start: 27, length: 0 }
            ])
        )
    }

    #[test]
    fn query_pronoun_literal() {
        assert_eq!(HashSet::from_iter(scan(String::from("Who is the brother of Jane?"))),
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
        assert_eq!(HashSet::from_iter(scan(String::from("Who is the sister of who?"))),
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
}
