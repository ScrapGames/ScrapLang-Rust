pub mod position;
pub mod frontend;

#[cfg(test)]
mod token_kinds_tests {
    use crate::frontend::tokens::kind::*;

    #[test]
    fn string_to_token_kind() {
        let token_piece = &String::from("fn");
        let kind: Keywords = Keywords::try_from(token_piece).unwrap();
        assert_eq!(kind, Keywords::Fn);
    }

    #[test]
    fn token_kind_to_string() {
        let kind = Keywords::Fn;
        let token_piece: &str = kind.try_into().unwrap();
        assert_eq!(token_piece, "fn");
    }

    #[test]
    fn string_to_token_kind_delegating() {
        let token_piece = &String::from("fn");
        let kind: Tokens = Tokens::try_from(token_piece).unwrap();
        assert_eq!(kind, Tokens::Keyword(Keywords::Fn));
    }

    #[test]
    fn token_kind_to_string_delegating() {
        let kind = Tokens::Keyword(Keywords::Fn);
        let token_piece: &str = kind.try_into().unwrap();
        assert_eq!(token_piece, "fn");
    }
}

#[cfg(test)]
mod tokens_tests {
    use crate::{
        position::Position,
        frontend::tokens::{ Token, kind::* },
    };

    #[test]
    fn token_is_keyword() {
        let content      = String::from("f");
        let kind: Tokens = Tokens::try_from(&content).unwrap();
        let position     = Position::new(0, 0, 0);
        let token        = Token::new(kind, content, position);
        assert!(token.is_keyword());
    }
}

fn main() {}
