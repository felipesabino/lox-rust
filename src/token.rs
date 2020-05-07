#[derive(Debug)]
pub enum TokenType {
  // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, Str, Number,

    // Keywords.
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error,
    Eof
  }

  pub struct Token<'lifetime> {
    pub length: usize,
    pub line: usize,
    pub start: usize,
    pub source: &'lifetime String,
    pub r#type: TokenType,
    pub error: &'static str,
  }