pub mod kind;

use crate::{
    position::Position,
    frontend::tokens::kind::Tokens,
};

pub struct Token {
    pub kind: Tokens,
    pub content: String,
    pub position: Position
}

impl Token {
    pub fn new(kind: Tokens, content: String, position: Position) -> Self {
        Self { kind, content, position }
    }

    #[inline]
    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, Tokens::Identifier)
    }

    #[inline]
    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, Tokens::Keyword(_))
    }

    #[inline]
    pub fn is_literal(&self) -> bool {
        matches!(self.kind, Tokens::Literal(_))
    }

    #[inline]
    pub fn is_operator(&self) -> bool {
        matches!(self.kind, Tokens::Operator(_))
    }

    #[inline]
    pub fn is(&self, kind: Tokens) -> bool {
        self.kind == kind
    }

    #[inline]
    pub fn cmp(&self, content: &str) -> bool {
        self.content == content
    }
}
