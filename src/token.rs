
#[derive(Debug, Clone, Copy)]
pub enum TokenType {

    // single char (syntax delim)
    LeftParen, RigtParen,
    LeftBrace, RightBrace,
    Comma, Dot, Semicolon,

    // algebra-ish
    Minus, Plus, Slash, Star,

    // one or two char (basically comparison)
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // literrals
    Identifier, String, Number,

    // keywords
    True, False, And, Or, Nil,
    Fun, Let, Cond, Cons,

    Eof
}

#[derive(Debug)]
pub enum BasicType {
    Number(f32),
    String(String),
    Nill,
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: BasicType,
    line: u32,
}

impl Token {
    pub fn new(t: TokenType, lex: String, lit: BasicType, line: u32) -> Self {
        return Token {token_type: t, lexeme: lex, literal: lit, line: line}
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
