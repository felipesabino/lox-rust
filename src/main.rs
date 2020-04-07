mod chunk;
mod debug;

use chunk::{Chunk, Instruction};
use debug::Debug;

fn main() {

    let mut chunk: Chunk = Chunk::new();

    let constant_index = chunk.add_constant(12.1);
    chunk.add_instruction(Instruction::Constant(constant_index), 123);

    chunk.add_instruction(Instruction::Return, 124);

    chunk.disassemble("test chunk".to_string());
}
