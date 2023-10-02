/// Object, used in parse function's return value.
#[derive(Debug, PartialEq)]
pub enum Object {
    List(Vec<Object>),
    Integer(Integer),
    Command(Command),
}

#[derive(Debug, PartialEq)]
pub enum Integer {
}

/// Command, used in parse function's return value.
#[derive(Debug, PartialEq)]
pub enum Command {
    Define,
    Operator(Operator),
}

/// Operator, only support four arithmetic operations.
#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

/// ParseError, used in parse function's Result value.
#[derive(Debug)]
pub struct ParseError {
    pub err: String,
}
