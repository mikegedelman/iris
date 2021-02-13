use logos::Logos;

// Logos builds a tokenizer for us for free
#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Tok<'input> {
    #[token(".")]
    Dot,

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("/")]
    ForwardSlash,

    #[token("*")]
    Star,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token("&")]
    Ampersand,

    #[token("|")]
    BitwiseOr,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LCurlyBracket,

    #[token("}")]
    RCurlyBracket,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("^")]
    Caret,

    #[token("%")]
    Modulo,

    #[token("!")]
    Exclamation,

    #[token("~")]
    Tilde,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("==")]
    DoubleEquals,

    #[token(">=")]
    GreaterThanEqual,

    #[token("<=")]
    LessThanEqual,

    #[token("<<")]
    ShiftLeft,

    #[token(">>")]
    ShiftRight,

    #[token("**")]
    Exponent,

    #[token("->")]
    Arrow,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("elif")]
    Elif,

    #[token("fn")]
    Fn,

    #[token("return")]
    Return,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("let")]
    Let,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("match")]
    Match,

    #[token("enum")]
    Enum,

    #[token("struct")]
    Struct,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("not")]
    Not,

    #[token("end")]
    End,

    #[token("then")]
    Then,

    #[regex(r#"'([^'\\]|\\t|\\u|\\n|\\')*'"#)]
    SingleLiteralString(&'input str),

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    DoubleLiteralString(&'input str),

    #[regex("[a-zA-Z_]+")]
    Ident(&'input str),

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i32),

    #[regex(r"[\r\n]+")]
    Crlf,

    #[regex(r"//.*(\r|\n)", logos::skip)]
    Comment,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f]+", logos::skip)]
    Error,
}
