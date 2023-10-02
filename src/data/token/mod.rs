#[derive(Debug, PartialEq)]
pub enum Token {
    ExtensionMark,
    QuestionMark,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::ExtensionMark => write!(f, "!"),
            Token::QuestionMark => write!(f, "?"),
        }
    }
}
