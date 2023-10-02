use crate::data::token::Token;
use crate::data::object::Object;
use crate::data::object::ParseError;
use crate::lexer::tokenize;

pub fn parse(program: &str) -> Result<Vec<Object>, ParseError> {
    let mut tokens: Vec<Token> = tokenize(program).into_iter().rev().collect::<Vec<_>>();
    let parsed_result: Vec<Object> = parse_list(&mut tokens).unwrap();

    Ok(parsed_result)
}

fn parse_list(tokens_vec: &mut Vec<Token>) -> Result<Vec<Object>, ParseError> {
//    let objects_result: Vec<Object> = vec![];
//    while !tokens_vec.is_empty() {
//        let mut t: Vec<Token> = vec![];
//        for _times in vec![1..8] {
//            let onetime_token: Token = tokens_vec.pop().unwrap();
//            t.push(onetime_token);
//        }
//        match t {
//            _ => {
//                return Err(ParseError { err: format!("Did not match any command or number")});
//            }
//        }
//    }
//
//    Ok(objects_result)
}

#[cfg(test)]
mod test {
    use crate::data::object::Object;
    use crate::parser::parse;

    #[test]
    fn test_parse() {
        const PROGRAM: &str = "!!!???!!";
        let parsed_result: Object = parse(PROGRAM).unwrap();
        assert_eq!(parsed_result, Object::List(vec![]),);
    }
}
