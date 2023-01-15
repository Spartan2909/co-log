pub trait Identifier {}

pub struct Literal {
    lexeme: &'static str,
    article: Option<&'static str>
}

impl Identifier for Literal {}

pub struct Variable {
    lexeme: &'static str,
    article: Option<&'static str>
}

impl Identifier for Variable {}

pub struct Clause {}

pub enum StmtType {
    Fact, Rule
}

pub trait Stmt {
    fn type_of(&self) -> StmtType;
    fn is_binary(&self) -> bool;
}

pub struct Fact {
    binary: bool,
    left: Literal,
    relationship: Literal,
    right: Literal
}

impl Stmt for Fact {
    fn type_of(&self) -> StmtType {
        StmtType::Fact
    }

    fn is_binary(&self) -> bool {
        self.binary
    }
}

pub struct Rule {
    binary: bool,
    left: Box<dyn Identifier>,
    relationship: Literal,
    right: Box<dyn Identifier>,
    condition: Clause
}

impl Stmt for Rule {
    fn type_of(&self) -> StmtType {
        StmtType::Rule
    }

    fn is_binary(&self) -> bool {
        self.binary
    }
}
