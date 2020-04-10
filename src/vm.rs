use crate::chunk::{Value, Chunk, Instruction::*};

// const STACK_MAX: isize = 256;

pub enum InterpreterResult {
  Ok,
  CompileError,
  RuntimeError,
}

pub struct VM<'lifetime> {
  chunk: &'lifetime Chunk,
  ip: usize,
  stack: Vec<Value>,
}

impl<'lifetime> VM<'lifetime> {

  fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  fn pop(&mut self) -> Value {
    match self.stack.pop() {
      Some(value) => return value,
      _ => panic!("VM tried to get value from empty stack"),
    }
  }

  pub fn new(chunk: &'lifetime Chunk) -> VM<'lifetime> {
    VM {
        chunk: chunk,
        ip: 0,
        stack: Vec::<Value>::new(),
    }
  }

  pub fn run(&mut self) -> InterpreterResult {

    loop {

      let instruction = &self.chunk.instructions[self.ip];
      self.ip = self.ip + 1;

      #[cfg(feature = "log_level_debug")]
      {
        print!("          ");
        for value in self.stack.iter() {
          print!("[ {:04} ]", value);
        }
        println!("");
        disassemble_instruction(self.chunk, &instruction, 0);
      }

      match instruction {
        Constant(index) => {
          let value = self.chunk.values[*index];
          self.push(value);
        },
        Return => {
          println!("{:04}", self.pop());
          return InterpreterResult::Ok;
        }
      }
    }
  }
}
