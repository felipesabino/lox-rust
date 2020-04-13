use crate::token::{Token,TokenType};

pub struct Scanner {
  pub source: String,
  start: usize,
  current: usize,
  line: usize,
}

impl Scanner {

  pub fn new(source: String) -> Scanner {
    Scanner {
      source: source,
      start: 0,
      current: 0,
      line: 0,
    }
  }

  pub fn scan_token(&mut self) -> Token {
    Token {
      start: self.start,
      length: self.current - self.start,
      line: self.line,
      r#type: TokenType::Eof,
    }
  }
}