pub type Value = f64;
pub type ConstantIndex = usize;

pub enum Instruction {
  Constant(ConstantIndex),
  Return,
}

pub struct Chunk {
  pub instructions: Vec<Instruction>,
  pub values: Vec<Value>,
  pub lines: Vec<isize>,
}

impl Chunk {

  pub fn new() -> Chunk {
    Chunk {
      instructions: Vec::<Instruction>::new(),
      values: Vec::<Value>::new(),
      lines: Vec::<isize>::new(),
    }
  }

  pub fn add_constant(&mut self, value: Value) -> ConstantIndex {
    self.values.push(value);
    return self.values.len() - 1;
  }

  pub fn add_instruction(&mut self, instruction: Instruction, line: isize) {
    self.instructions.push(instruction);
    self.lines.push(line);
  }
}