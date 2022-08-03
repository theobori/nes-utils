mod error;
mod disassembler;
mod opcode;
mod addressing;
mod block;

#[macro_use]
extern crate lazy_static;

use std::env;

use disassembler::disassembler::NesDisassembler;
use error::NesError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();

    if args_count < 2 {
        panic!("{}", NesError::CLIArguments);
    }

    NesDisassembler::new(&args[1])
        .mem_from_file()
        .disassemble()
        .save_as("mario.asm");
}
