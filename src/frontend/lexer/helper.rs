use crate::{
    position::Position,
    frontend::{
        lexer::Lexer,
        tokens::{ Token, kind::Tokens }
    }
};

impl Lexer {
    pub fn from_kind(kind: Tokens, position: Position) -> Token {
        let c: Result<&'static str, &'static str> = kind.try_into();

        match c {
            Ok(c)    => Token::new(kind, String::from(c), position),
            Err(err) => panic!("Failed to create token from content: {}", err)
        }
    }

    pub fn from_content(content: String, position: Position) -> Token {
        match Tokens::try_from(&content) {
            Ok(kind) => Token::new(kind, content, position),
            Err(err) => panic!("Failed to create token from content: {}", err)
        }
    }
}