use crate::chunk::{Chunk, Instruction};

pub trait Debug {
  fn disassemble(&self, name: String);
}

impl Debug for Chunk {
  fn disassemble(&self, name: String) {
    println!("== {} ==", name);

    for i in 0..self.instructions.len() {
      match self.instructions.get(i) {
        Some(instruction) => disassemble_instruction(self, instruction, i),
        _ => println!("Error debugging instruction on at {}", i),
      }
    }
  }
}

fn disassemble_instruction(chunk: &Chunk, instruction: &Instruction, index: usize) {
  match instruction {
    Return =>
  }
}