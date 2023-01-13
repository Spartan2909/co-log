use self::TokenType::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single character tokens
    FullStop, QuestionMark, LeftParen, RightParen,

    // Reserved words
    Article, Operator, Prepostion, Verb,
    
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
    fn new(kind: TokenType, lexeme: String, start: usize, length: usize) -> Self {
        Token { kind, lexeme, start, length }
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
            '(' => tokens.push(Token::new(LeftParen, String::from("("), i, 1)),
            ')' => tokens.push(Token::new(RightParen, String::from(")"), i, 1)),
            '.' => tokens.push(Token::new(FullStop, String::from("."), i, 1)),
            '?' => tokens.push(Token::new(QuestionMark, String::from("?"), i, 1)),
            c if c.is_alphabetic() => {
                let lexeme = get_word(&source, i);
                let length = lexeme.len();
                let kind: TokenType;
                match lexeme.to_lowercase().as_str() {
                    "a" | "an" | "the" => kind = Article,
                    "and" | "or" => kind = Operator,
                    "of" | "to" => kind = Prepostion,
                    "is" | "are" => kind = Verb,
                    _ => {
                        if lexeme.chars().nth(length-1).unwrap().is_lowercase() {
                            kind = Literal;
                        } else {
                            kind = Variable;
                        }
                    }
                }
                tokens.push(Token::new(kind, lexeme, i, length));

                i += length;
                continue
            },
            _ => todo!("unrecognised character"),
        }

        i += 1;
    }

    tokens.push(Token::new(TokenType::EOF, String::from(""), i, 0));
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn scan_test_fact() {
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
    fn scan_test_rule() {
        assert_eq!(HashSet::from_iter(scan(String::from("X is the brother of Y if X is the sibling of Y and X is male."))),
            HashSet::from([
                Token::new(Variable, String::from("X"), 0, 1)
            ])
        )
    }
}
