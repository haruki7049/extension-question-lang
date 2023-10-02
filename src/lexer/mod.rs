use crate::data::token::Token;

pub fn tokenize(program: &str) -> Vec<Token> {
    let vec_char: Vec<char> = make_vector_char(program);
    let mut tokens: Vec<Token> = vec![];

    for c in vec_char {
        match c {
            '!' => tokens.push(Token::ExtensionMark),
            '?' => tokens.push(Token::QuestionMark),
            _ => {}
        }
    }

    tokens
}

fn make_vector_char(str: &str) -> Vec<char> {
    str.chars().collect()
}

#[cfg(test)]
mod test {
    use crate::data::token::Token;
    use crate::lexer::tokenize;

    #[test]
    fn test_tokenize() {
        const PROGRAM: &str = "!!!!!!!!";
        let tokens: Vec<Token> = tokenize(PROGRAM);
        assert_eq!(
            tokens,
            vec![
                Token::ExtensionMark,
                Token::ExtensionMark,
                Token::ExtensionMark,
                Token::ExtensionMark,
                Token::ExtensionMark,
                Token::ExtensionMark,
                Token::ExtensionMark,
                Token::ExtensionMark,
            ]
        );
    }
}
