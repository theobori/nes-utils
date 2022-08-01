mod error;
mod disassembler;
pub mod opcode;

#[macro_use]
extern crate lazy_static;

use std::env;

use disassembler::disassembler::NesDisassembler;
use error::NesError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();

    if args_count < 2 || args_count > 3 {
        panic!("{}", NesError::Arguments);
    }

    let mut nes_disas = NesDisassembler::new()
        .mem_from_file(&args[1])
        .disassemble()
        .print();
}
