use crate::scanner::Scanner;
use crate::token::TokenType;

pub struct Compiler { }

impl Compiler {
  pub fn new() -> Compiler {
    Compiler { }
  }

  pub fn compile(&mut self, source: String) {
    let mut scanner = Scanner::new(&source);
    let mut line: usize = usize::max_value(); //it could be any value > 0, really
    loop {
      let token = scanner.scan_token();
      if token.line != line {
        print!("{:04} ", token.line);
        line = token.line;
      } else {
        print!("   | ");
      }

      let lexeme = match token.r#type {
        TokenType::Error => token.error,
        _ =>  &token.source[token.start..token.length + token.start],
      };
      println!("{:?} {}", token.r#type, lexeme);

      match token.r#type {
        TokenType::Eof => break,
        _ => {},
      }
    }
  }
}