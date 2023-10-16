/// contains some data which is Token, SeparatedToken and Object.
mod data;

/// this module convert &str to SeparatedToken.
/// &str -> Token -> SeparatedToken
pub mod lexer;

/// this module convert SeparatedToken to Object which is able to process in Rust-lang.
/// SeparatedToken -> Object -> process
pub mod evaluator;
