use crate::data::token::SeparatedToken;
use crate::data::token::Token;

/// tokenize function, convert Token &str
/// TODO: now this function is convert SeparatedToken so fix this
pub fn tokenize(program: &str) -> Vec<SeparatedToken> {
    let mut result: Vec<SeparatedToken> = Vec::new();
    let vec_char: Vec<char> = make_vector_char(program);

    push_into_separate_token(vec_char, &mut result);

    result
}

/// a helper function push_into_separate_token, convert Vec<SeparatedToken> Vec<char>
fn push_into_separate_token(vec_char: Vec<char>, result: &mut Vec<SeparatedToken>) {
    let mut tokens: Vec<Token> = vec![];

    for c in vec_char {
        match c {
            '!' => tokens.push(Token::ExtensionMark),
            '?' => tokens.push(Token::QuestionMark),
            _ => {}
        }
        result.push(separate_token(&mut tokens));
    }
}

/// a helper function make_vector_char, convert Vec<char>. all charactor is ignored by this function but '!' and '?' is not.
fn make_vector_char(str: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    let vec_char: Vec<char> = str.chars().collect();
    for c in vec_char.iter() {
        match c {
            '!' => result.push('!'),
            '?' => result.push('?'),
            _ => {}
        }
    }
    result
}

/// separate_token function, this function partition tokens into SeparatedToken.
fn separate_token(tokens: &mut Vec<Token>) -> SeparatedToken {
    let first: Token = tokens.pop().unwrap();
    let second: Token = tokens.pop().unwrap();
    let third: Token = tokens.pop().unwrap();
    let fourth: Token = tokens.pop().unwrap();
    let fifth: Token = tokens.pop().unwrap();
    let sixth: Token = tokens.pop().unwrap();
    let seventh: Token = tokens.pop().unwrap();
    let eighth: Token = tokens.pop().unwrap();

    SeparatedToken {
        first_token: first,
        second_token: second,
        third_token: third,
        fourth_token: fourth,
        fifth_token: fifth,
        sixth_token: sixth,
        seventh_token: seventh,
        eighth_token: eighth,
    }
}

#[cfg(test)]
mod test {
    use crate::data::token::SeparatedToken;
    use crate::data::token::Token;
    use crate::lexer::make_vector_char;
    use crate::lexer::tokenize;

    /// test whether tokenize function is correctly convert Token one line, or not.
    #[test]
    fn test_tokenize() {
        const PROGRAM: &str = "!!!!!!!!";
        let tokens: Vec<SeparatedToken> = tokenize(PROGRAM);
        assert_eq!(
            tokens,
            vec![SeparatedToken {
                first_token: Token::ExtensionMark,
                second_token: Token::ExtensionMark,
                third_token: Token::ExtensionMark,
                fourth_token: Token::ExtensionMark,
                fifth_token: Token::ExtensionMark,
                sixth_token: Token::ExtensionMark,
                seventh_token: Token::ExtensionMark,
                eighth_token: Token::ExtensionMark,
            }]
        );
    }

    /// test whether tokenize function is correctly convert Token two lines, or not.
    #[test]
    fn test_twoline_programs() {
        const PROGRAM: &str = "
!!!!!!!!
????????
        ";
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
                },
                SeparatedToken {
                    first_token: Token::QuestionMark,
                    second_token: Token::QuestionMark,
                    third_token: Token::QuestionMark,
                    fourth_token: Token::QuestionMark,
                    fifth_token: Token::QuestionMark,
                    sixth_token: Token::QuestionMark,
                    seventh_token: Token::QuestionMark,
                    eighth_token: Token::QuestionMark,
                }
            ]
        );
    }

    /// test whether newline and whitespace is ignored by make_vector_char function, or not.
    #[test]
    fn test_ignore_newline_and_whitespace() {
        const PROGRAM: &str = "
        ????????
        !!!!!!!!
        ";
        let vec_char: Vec<char> = make_vector_char(PROGRAM);
        assert_eq!(
            vec_char,
            vec!['?', '?', '?', '?', '?', '?', '?', '?', '!', '!', '!', '!', '!', '!', '!', '!',]
        );
    }
}
