// mod chunk;
// mod debug;
// mod vm;
mod compiler;
mod scanner;
mod token;

// use chunk::{Chunk, Instruction};
// use debug::Debug;
// use vm::VM;
use compiler::Compiler;

use std::env;
use std::io::{self, Write};
use std::fs;

fn main() {

    // let mut chunk: Chunk = Chunk::new();

    // let constant_index = chunk.add_constant(12.1);
    // chunk.add_instruction(Instruction::Constant(constant_index), 123);

    // let constant_index = chunk.add_constant(3.4);
    // chunk.add_instruction(Instruction::Constant(constant_index), 123);

    // chunk.add_instruction(Instruction::Add, 123);

    // let constant_index = chunk.add_constant(5.6);
    // chunk.add_instruction(Instruction::Constant(constant_index), 123);

    // chunk.add_instruction(Instruction::Divide, 123);

    // chunk.add_instruction(Instruction::Negate, 123);

    // chunk.add_instruction(Instruction::Return, 123);

    // chunk.disassemble("test chunk");

    // let mut vm = VM::new(&chunk);
    // vm.run();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        match args.get(1) {
            Some(path) => run_file(path),
            _ => panic!("Usage: cargo run [path]"),
        }
    } else {
        panic!("Usage: cargo run [path]");
    }

}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut compiler = Compiler::new();
                compiler.compile(input);
            },
            Err(_error) => println!("Error reading input"),
        }
    }
}

fn run_file(path: &String) {
    match fs::read_to_string(path) {
        Ok(input) => {
            let mut compiler = Compiler::new();
            compiler.compile(input);
        },
        _ => println!("Could not open file."),
    }
}