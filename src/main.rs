mod chunk;
mod debug;
mod vm;

use chunk::{Chunk, Instruction};
use debug::Debug;
use vm::VM;

fn main() {

    let mut chunk: Chunk = Chunk::new();

    let constant_index = chunk.add_constant(12.1);
    chunk.add_instruction(Instruction::Constant(constant_index), 123);

    let constant_index = chunk.add_constant(3.4);
    chunk.add_instruction(Instruction::Constant(constant_index), 123);

    chunk.add_instruction(Instruction::Add, 123);

    let constant_index = chunk.add_constant(5.6);
    chunk.add_instruction(Instruction::Constant(constant_index), 123);

    chunk.add_instruction(Instruction::Divide, 123);

    chunk.add_instruction(Instruction::Negate, 123);

    chunk.add_instruction(Instruction::Return, 123);

    chunk.disassemble("test chunk");

    let mut vm = VM::new(&chunk);
    vm.run();
}
