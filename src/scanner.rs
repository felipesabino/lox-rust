use crate::token::{Token,TokenType};
use std::str::{Chars};

pub struct Scanner<'lifetime> {
  source: &'lifetime String,
  start: usize,
  current: usize,
  line: usize,
}

impl<'lifetime> Scanner<'lifetime> {

  pub fn new(source: &'lifetime String) -> Scanner {
    Scanner {
      source: source,
      start: 0,
      current: 0,
      line: 0,
    }
  }

  fn is_digit(value: char) -> bool {
    return match value {
      '1'..='9' => true,
      _ => return false,
    };
  }

  fn is_alpha(value: char) -> bool {
    return match value {
      'a'..='z' => true,
      'A'..='Z' => true,
      _ => false
    };
  }

  fn is_at_end(&self) -> bool {
    return self.current >= self.source.len();
  }

  fn advance(&mut self) -> char {
    self.current = self.current + 1;
    return self.source.chars().nth(self.current - 1)
      .expect("[scanner] trying to advance to out of bounds character"); //TODO: improve error handling
  }

  fn peek(&self) -> char {
    if self.is_at_end() { return '\0'; }
    return self.source.chars().nth(self.current)
      .expect("[scanner] trying to peek out of bounds character"); //TODO: improve error handling
  }

  fn peek_next(&self) -> char {
    if self.is_at_end() { return '\0'; }
    return self.source.chars().nth(self.current + 1)
      .expect("[scanner] trying to peek next out of bounds character"); //TODO: improve error handling
  }

  fn _match(&mut self, token: char) -> bool {
    if self.is_at_end() { return false; }
    let next = self.peek_next();
    if next == token {
      self.current = self.current + 1;
      return true;
    } else {
      return false;
    }
  }

  fn make_token(&self, r#type: TokenType) -> Token {
    Token {
      length: self.current - self.start,
      line: self.line,
      start: self.start,
      source: self.source,
      r#type: r#type,
    }
  }

  fn error_token(&self) -> Token {
    // Token {
    //   length: message.len(),
    //   line: self.line,
    //   start: 0,
    //   source: message,
    //   r#type: TokenType::Error,
    // }
    // TODO: how to return reference to a message?
    Token {
      length: 0,
      line: self.line,
      start: 0,
      source: self.source,
      r#type: TokenType::Error,
    }
  }

  fn skip_whitespace(&mut self) {
    loop {
      let c = self.peek();
      match c {
        ' ' | '\r' | '\t' => {
          self.advance();
        },
        '\n' => {
          self.line = self.line + 1;
          self.advance();
        },
        '/' => {
          if self.peek_next() == '/' {
            // A comment goes until the end of the line.
            loop {
              if self.peek() != '\n' && !self.is_at_end() {
                self.advance();
              } else {
                return;
              }
            }
          }
        }
        _ => return,
      }
    }
  }

  fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
    // TODO: all of this looks horrible, surely there is a better way to compare chars and mplement this trie
    let mut offset = 0;
    let end = rest.len();
    loop {
      let mut chars = self.source.chars();
      let mut rest = rest.chars();
      let curr_char = chars.nth(self.start + start + offset).unwrap();
      let curr_checked = rest.nth(offset).unwrap();
      if curr_char != curr_checked {
        return TokenType::Identifier
      }
      offset = offset + 1;
      if offset >= end - start {
        return token_type;
      }
    }
  }

  fn identifier_type(&mut self) -> TokenType {

    let c = self.source.chars().nth(self.start)
      .expect("[scanner] trying to peek identifier out of bounds character"); //TODO: improve error handling

    return match c {
      'a' => self.check_keyword(1, "nd", TokenType::And),
      'c' => self.check_keyword(1, "lass", TokenType::Class),
      'e' => self.check_keyword(1, "lse", TokenType::Else),
      'i' => self.check_keyword(1, "f", TokenType::If),
      'n' => self.check_keyword(1, "il", TokenType::Nil),
      'o' => self.check_keyword(1, "r", TokenType::Or),
      'p' => self.check_keyword(1, "rint", TokenType::Print),
      'r' => self.check_keyword(1, "eturn", TokenType::Return),
      's' => self.check_keyword(1, "uper", TokenType::Super),
      'v' => self.check_keyword(1, "ar", TokenType::Var),
      'w' => self.check_keyword(1, "hile", TokenType::While),
      'f' => {
        match self.peek_next() {
          'a' => self.check_keyword(2, "lse", TokenType::False),
          'o' => self.check_keyword(2, "r", TokenType::For),
          'u' => self.check_keyword(2, "n", TokenType::Fun),
          _ => TokenType::Identifier
        }
      },
      't' => {
        match self.peek_next() {
          'h' => self.check_keyword(2, "is", TokenType::This),
          'r' => self.check_keyword(2, "ue", TokenType::True),
          _ => TokenType::Identifier
        }
      },
      _ => TokenType::Identifier,
    }

  }

  fn identifier(&mut self) -> Token {
    loop {
      if Self::is_alpha(self.peek()) || Self::is_digit(self.peek()) {
        self.advance();
      } else {
        break;
      }
    }

    let identifier_type = self.identifier_type();
    return self.make_token(identifier_type);
  }

  fn number(&mut self) -> Token {

    loop {
      if Self::is_digit(self.peek()) {
        self.advance();
      } else {
        break;
      }
    }

    if self.peek() == '.' && Self::is_digit(self.peek_next()) {
      // consume the "."
      self.advance();

      loop {
        if Self::is_digit(self.peek()) {
          self.advance();
        } else {
          break;
        }
      }
    }

    return self.make_token(TokenType::Number);

  }

  fn string(&mut self) -> Token {
    loop {
      if self.peek() != '"' && !self.is_at_end() {
        if self.peek() == '\n' {
          self.line = self.line + 1;
        }
        self.advance();
      } else {
        break;
      }
    }

    if self.is_at_end() {
      return self.error_token();
    }

    //closing quote
    self.advance();
    return self.make_token(TokenType::Str);
  }

  pub fn scan_token(&mut self) -> Token {

    self.skip_whitespace();

    self.start = self.current;

    if self.is_at_end() { return self.make_token(TokenType::Eof); }

    let c = self.advance();

    if Self::is_digit(c) { return self.number(); }
    if Self::is_alpha(c) { return self.identifier(); }

    match c {
      '(' => return self.make_token(TokenType::LeftParen),
      ')' => return self.make_token(TokenType::RightParen),
      '{' => return self.make_token(TokenType::LeftBrace),
      '}' => return self.make_token(TokenType::RightBrace),
      ';' => return self.make_token(TokenType::Semicolon),
      ',' => return self.make_token(TokenType::Comma),
      '.' => return self.make_token(TokenType::Dot),
      '-' => return self.make_token(TokenType::Minus),
      '+' => return self.make_token(TokenType::Plus),
      '/' => return self.make_token(TokenType::Slash),
      '*' => return self.make_token(TokenType::Star),
      '!' => {
        let token_type = match self._match('=') {
          true => TokenType::BangEqual,
          _ => TokenType::Bang,
        };
        return self.make_token(token_type);
      },
      '=' => {
        let token_type = match self._match('=') {
          true => TokenType::EqualEqual,
          _ => TokenType::Equal,
        };
        return self.make_token(token_type);
      },
      '<' => {
        let token_type = match self._match('=') {
          true => TokenType::LessEqual,
          _ => TokenType::Less,
        };
        return self.make_token(token_type);
      },
      '>' => {
        let token_type = match self._match('=') {
          true => TokenType::GreaterEqual,
          _ => TokenType::Greater,
        };
        return self.make_token(token_type);
      },
      '"' => return self.string(),
      _ => return self.error_token(),
    }
  }
}