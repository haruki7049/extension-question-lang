#[derive(Debug, PartialEq)]
pub enum Token {
    ExtensionMark,
    QuestionMark,
}

#[derive(Debug, PartialEq)]
pub struct SeparatedToken {
    pub first_token: Token,
    pub second_token: Token,
    pub third_token: Token,
    pub fourth_token: Token,
    pub fifth_token: Token,
    pub sixth_token: Token,
    pub seventh_token: Token,
    pub eighth_token: Token,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::ExtensionMark => write!(f, "!"),
            Token::QuestionMark => write!(f, "?"),
        }
    }
}

impl SeparatedToken {
    fn new(first: Token, Second: Token, Third: Token,) -> SeparatedToken {
    }
}
