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

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, Tokens::Identifier)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, Tokens::Keyword(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(self.kind, Tokens::Literal(_))
    }

    pub fn is_operator(&self) -> bool {
        matches!(self.kind, Tokens::Operator(_))
    }

    pub fn is(&self, kind: Tokens) -> bool {
        self.kind == kind
    }

    pub fn cmp(&self, content: &str) -> bool {
        self.content == content
    }
}
