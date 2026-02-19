

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Keywords {
    Fn,
    Var,
    Const,
    Return,
    Import,
    From,
    Export,
    Class,
    Type,
    Interface,
    Enum,
    Module,
    For,
    While,
    Do,
    Break,
    Skip,
    If,
    Else,
    Elif,
    Match,
    Switch,
    Extends,
    Implements,
    Case,
    Default,
    Of,
    Try,
    Catch,
    Pub,
    Private,
    Protected,
    Static,
    Override,
    Setter,
    Getter,
    Async,
    Await,
    Dissipate,
    Inline,
    Impl,
    Extern
}

impl TryFrom<&String> for Keywords {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "fn"         => Ok(Keywords::Fn),
            "var"        => Ok(Keywords::Var),
            "const"      => Ok(Keywords::Const),
            "return"     => Ok(Keywords::Return),
            "import"     => Ok(Keywords::Import),
            "from"       => Ok(Keywords::From),
            "export"     => Ok(Keywords::Export),
            "class"      => Ok(Keywords::Class),
            "type"       => Ok(Keywords::Type),
            "interface"  => Ok(Keywords::Interface),
            "enum"       => Ok(Keywords::Enum),
            "module"     => Ok(Keywords::Module),
            "for"        => Ok(Keywords::For),
            "while"      => Ok(Keywords::While),
            "do"         => Ok(Keywords::Do),
            "break"      => Ok(Keywords::Break),
            "skip"       => Ok(Keywords::Skip),
            "if"         => Ok(Keywords::If),
            "else"       => Ok(Keywords::Else),
            "elif"       => Ok(Keywords::Elif),
            "match"      => Ok(Keywords::Match),
            "switch"     => Ok(Keywords::Switch),
            "extends"    => Ok(Keywords::Extends),
            "implements" => Ok(Keywords::Implements),
            "case"       => Ok(Keywords::Case),
            "default"    => Ok(Keywords::Default),
            "of"         => Ok(Keywords::Of),
            "try"        => Ok(Keywords::Try),
            "catch"      => Ok(Keywords::Catch),
            "pub"        => Ok(Keywords::Pub),
            "private"    => Ok(Keywords::Private),
            "protected"  => Ok(Keywords::Protected),
            "static"     => Ok(Keywords::Static),
            "override"   => Ok(Keywords::Override),
            "setter"     => Ok(Keywords::Setter),
            "getter"     => Ok(Keywords::Getter),
            "async"      => Ok(Keywords::Async),
            "await"      => Ok(Keywords::Await),
            "dissipate"  => Ok(Keywords::Dissipate),
            "inline"     => Ok(Keywords::Inline),
            "impl"       => Ok(Keywords::Impl),
            "extern"     => Ok(Keywords::Extern),
            _            => Err("Unknown keyword")
        }
    }
}

impl Into<&str> for Keywords {
    fn into(self) -> &'static str {
        match self {
            Keywords::Fn         => "fn",
            Keywords::Var        => "var",
            Keywords::Const      => "const",
            Keywords::Return     => "return",
            Keywords::Import     => "import",
            Keywords::From       => "from",
            Keywords::Export     => "export",
            Keywords::Class      => "class",
            Keywords::Type       => "type",
            Keywords::Interface  => "interface",
            Keywords::Enum       => "enum",
            Keywords::Module     => "module",
            Keywords::For        => "for",
            Keywords::While      => "while",
            Keywords::Do         => "do",
            Keywords::Break      => "break",
            Keywords::Skip       => "skip",
            Keywords::If         => "if",
            Keywords::Else       => "else",
            Keywords::Elif       => "elif",
            Keywords::Match      => "match",
            Keywords::Switch     => "switch",
            Keywords::Extends    => "extends",
            Keywords::Implements => "implements",
            Keywords::Case       => "case",
            Keywords::Default    => "default",
            Keywords::Of         => "of",
            Keywords::Try        => "try",
            Keywords::Catch      => "catch",
            Keywords::Pub        => "pub",
            Keywords::Private    => "private",
            Keywords::Protected  => "protected",
            Keywords::Static     => "static",
            Keywords::Override   => "override",
            Keywords::Setter     => "setter",
            Keywords::Getter     => "getter",
            Keywords::Async      => "async",
            Keywords::Await      => "await",
            Keywords::Dissipate  => "dissipate",
            Keywords::Inline     => "inline",
            Keywords::Impl       => "impl",
            Keywords::Extern     => "extern"
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Operators {
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percen,  // %
    Less,    // <
    Greater, // >
    Dot,     // .
    Pipe,    // |
    Bang,    // !
    Amper,   // &
    Equal,   // =
    Comma,   // ,

    // ========================
    // KEYWORD OPERATORS
    // ========================

    In,          // in
    And,         // and
    ExplicitAnd, // and!
    Or,          // or
    ExplicitOr,  // or!
    InstanceOf,  // instanceof
    Not,         // not
    As,          // as
    New,         // new
    Drop,        // drop

    // ========================
    // COMPOUNDED OPERATORS
    // ========================

    Increment,   // ++
    Decrement,   // --
    AddAssign,   // +=
    MinusAssign, // -=
    MultAssign,  // *=
    DivAssign,   // /=
    RemAssign,   // %=
    LessEqual,   // <=
    GreatEqual,  // >=
    Equals,      // ==
    NotEquals,   // !=
    ModAccessor, // ::
    Slice,       // ..
    Spread,      // ...

    Lparen,  // ( 
    Rparen,  // )
    Lsquare, // [
    Rsquare, // ]
}

impl TryFrom<&String> for Operators {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "+"          => Ok(Operators::Plus),
            "-"          => Ok(Operators::Minus),
            "*"          => Ok(Operators::Star),
            "/"          => Ok(Operators::Slash),
            "%"          => Ok(Operators::Percen),
            "<"          => Ok(Operators::Less),
            ">"          => Ok(Operators::Greater),
            "."          => Ok(Operators::Dot),
            "|"          => Ok(Operators::Pipe),
            "!"          => Ok(Operators::Bang),
            "&"          => Ok(Operators::Amper),
            "="          => Ok(Operators::Equal),
            ","          => Ok(Operators::Comma),
            "in"         => Ok(Operators::In),
            "and"        => Ok(Operators::And),
            "and!"       => Ok(Operators::ExplicitAnd),
            "or"         => Ok(Operators::Or),
            "or!"        => Ok(Operators::ExplicitOr),
            "instanceof" => Ok(Operators::InstanceOf),
            "not"        => Ok(Operators::Not),
            "as"         => Ok(Operators::As),
            "new"        => Ok(Operators::New),
            "drop"       => Ok(Operators::Drop),
            "++"         => Ok(Operators::Increment),
            "--"         => Ok(Operators::Decrement),
            "+="         => Ok(Operators::AddAssign),
            "-="         => Ok(Operators::MinusAssign),
            "*="         => Ok(Operators::MultAssign),
            "/="         => Ok(Operators::DivAssign),
            "%="         => Ok(Operators::RemAssign),
            "<="         => Ok(Operators::LessEqual),
            ">="         => Ok(Operators::GreatEqual),
            "=="         => Ok(Operators::Equals),
            "!="         => Ok(Operators::NotEquals),
            "::"         => Ok(Operators::ModAccessor),
            ".."         => Ok(Operators::Slice),
            "..."        => Ok(Operators::Spread),
            "("          => Ok(Operators::Lparen),
            ")"          => Ok(Operators::Rparen),
            "["          => Ok(Operators::Lsquare),
            "]"          => Ok(Operators::Rsquare),
            _            => Err("Unknown operator")
        }
    }
}

impl Into<&str> for Operators {
    fn into(self) -> &'static str {
        match self {
            Operators::Plus        => "+",
            Operators::Minus       => "-",
            Operators::Star        => "*",
            Operators::Slash       => "/",
            Operators::Percen      => "%",
            Operators::Less        => "<",
            Operators::Greater     => ">",
            Operators::Dot         => ".",
            Operators::Pipe        => "|",
            Operators::Bang        => "!",
            Operators::Amper       => "&",
            Operators::Equal       => "=",
            Operators::Comma       => ",",
            Operators::In          => "in",
            Operators::And         => "and",
            Operators::ExplicitAnd => "and!",
            Operators::Or          => "or",
            Operators::ExplicitOr  => "or!",
            Operators::InstanceOf  => "instanceof",
            Operators::Not         => "not",
            Operators::As          => "as",
            Operators::New         => "new",
            Operators::Drop        => "drop",
            Operators::Increment   => "++",
            Operators::Decrement   => "--",
            Operators::AddAssign   => "+=",
            Operators::MinusAssign => "-=",
            Operators::MultAssign  => "*=",
            Operators::DivAssign   => "/=",
            Operators::RemAssign   => "%=",
            Operators::LessEqual   => "<=",
            Operators::GreatEqual  => ">=",
            Operators::Equals      => "==",
            Operators::NotEquals   => "!=",
            Operators::ModAccessor => "::",
            Operators::Slice       => "..",
            Operators::Spread      => "...",
            Operators::Lparen      => "(",
            Operators::Rparen      => ")",
            Operators::Lsquare     => "[",
            Operators::Rsquare     => "]",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Literals {
    String,
    Char,
    Number,
    Float
}

impl TryFrom<&String> for Literals {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "STRING" => Ok(Literals::String),
            "CHAR"   => Ok(Literals::Char),
            "NUMBER" => Ok(Literals::Number),
            "FLOAT"  => Ok(Literals::Float),
            _        => Err("Unknown literal")
        }
    }
}

impl Into<&str> for Literals {
    fn into(self) -> &'static str {
        match self {
            Literals::String => "STRING",
            Literals::Char   => "CHAR",
            Literals::Number => "NUMBER",
            Literals::Float  => "FLOAT",
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Tokens {
    Keyword(Keywords),
    Operator(Operators),
    Literal(Literals),

    Identifier,
    Arrow,
    Semicolon,
    Colon,
    Underscore,
    Question,
    Lbrace,
    Rbrace,

    Comment,

    Eof,
    Unknown
}

impl TryInto<&str> for Tokens {
    type Error = &'static str;

    fn try_into(self) -> Result<&'static str, Self::Error> {
        match self {
            Tokens::Keyword(k)    => Ok(k.into()),
            Tokens::Operator(op)  => Ok(op.into()),
            Tokens::Literal(lit)  => Ok(lit.into()),
            Tokens::Semicolon     => Ok(";"),
            Tokens::Colon         => Ok(":"),
            Tokens::Underscore    => Ok("_"),
            Tokens::Question      => Ok("?"),
            Tokens::Arrow         => Ok("->"),
            Tokens::Lbrace        => Ok("{{"),
            Tokens::Rbrace        => Ok("}}"),
            _                     => Err("The token is unknown or has no textual representation")
        }
    }
}

impl TryFrom<&String> for Tokens {
    type Error = &'static str;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Keywords::try_from(s)
            .map(Tokens::Keyword)
            .or_else(|_| Operators::try_from(s).map(Tokens::Operator))
            .or_else(|_| Literals::try_from(s).map(Tokens::Literal))
            .or_else(|_| {
                match s.as_str() {
                    "?"  => Ok(Tokens::Question),
                    ";"  => Ok(Tokens::Semicolon),
                    "_"  => Ok(Tokens::Underscore),
                    ":"  => Ok(Tokens::Colon),
                    "->" => Ok(Tokens::Arrow),
                    "{{" => Ok(Tokens::Lbrace),
                    "}}" => Ok(Tokens::Rbrace),
                    _    => Err("The token is unknown or has no textual representation"),
                }
            })
    }
} 
