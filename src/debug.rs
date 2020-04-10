use crate::chunk::{Chunk, Instruction, Instruction::*};

pub trait Debug {
  fn disassemble(&self, name: &str);
}

impl Debug for Chunk {
  fn disassemble(&self, name: &str) {
    println!("== {} ==", name);

    for i in 0..self.instructions.len() {
      match self.instructions.get(i) {
        Some(instruction) => disassemble_instruction(self, instruction, i),
        _ => println!("Error debugging instruction on at {}", i),
      }
    }
  }
}

fn disassemble_instruction(chunk: &Chunk, instruction: &Instruction, offset: usize) {

  print!("{:04} ", offset);

  if offset > 0 && offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
    print!("   | ");
  } else {
    print!("{:04} ", chunk.lines[offset]);
  }

  match instruction {
    Constant(index) => disassemble_constant_instruction(chunk, *index),
    Return => disassemble_simple_instruction("RETURN"),
  }
}

fn disassemble_simple_instruction(name: &str) {
  println!("{:<16}", name);
}

fn disassemble_constant_instruction(chunk: &Chunk, index: usize) {
  let value = chunk.values[index];
  println!("{:<16} {:04} '{:.prec$}'", "CONSTANT", index, value, prec=2);
}