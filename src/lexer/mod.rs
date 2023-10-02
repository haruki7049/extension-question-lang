use crate::data::token::SeparatedToken;
use crate::data::token::Token;

pub fn tokenize(program: &str) -> Vec<SeparatedToken> {
    let vec_char: Vec<char> = make_vector_char(program);
    let mut tokens: Vec<Token> = vec![];

    for c in vec_char {
        match c {
            '!' => tokens.push(Token::ExtensionMark),
            '?' => tokens.push(Token::QuestionMark),
            _ => {}
        }
    }

    separate_token(&mut tokens)
}

fn make_vector_char(str: &str) -> Vec<char> {
    str.chars().collect()
}

/// TODO: separate_token function, this function partition tokens into
fn separate_token(tokens: &mut Vec<Token>) -> Vec<SeparatedToken> {
    let mut result: Vec<SeparatedToken> = Vec::new();

    let first: Token = tokens.pop().unwrap();
    let second: Token = tokens.pop().unwrap();
    let third: Token = tokens.pop().unwrap();
    let fourth: Token = tokens.pop().unwrap();
    let fifth: Token = tokens.pop().unwrap();
    let sixth: Token = tokens.pop().unwrap();
    let seventh: Token = tokens.pop().unwrap();
    let eighth: Token = tokens.pop().unwrap();

    result.push(SeparatedToken {
        first_token: first,
        second_token: second,
        third_token: third,
        fourth_token: fourth,
        fifth_token: fifth,
        sixth_token: sixth,
        seventh_token: seventh,
        eighth_token: eighth,
    });

    result
}

#[cfg(test)]
mod test {
    use crate::data::token::Token;
    use crate::lexer::tokenize;
    use crate::data::token::SeparatedToken;

    #[test]
    fn test_tokenize() {
        const PROGRAM: &str = "!!!!!!!!";
        let tokens: Vec<SeparatedToken> = tokenize(PROGRAM);
        assert_eq!(
            tokens,
            vec![
                SeparatedToken {
                    first_token: Token::ExtensionMark,
                    second_token: Token::ExtensionMark,
                    third_token: Token::ExtensionMark,
                    fourth_token: Token::ExtensionMark,
                    fifth_token: Token::ExtensionMark,
                    sixth_token: Token::ExtensionMark,
                    seventh_token: Token::ExtensionMark,
                    eighth_token: Token::ExtensionMark,
                }
            ]
        );
    }
}
