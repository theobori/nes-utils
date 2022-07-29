mod error;
mod disassembler;

use std::env;

use error::NesError;
use disassembler::header::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();

    if args_count < 2 || args_count > 3 {
        panic!("{}", NesError::Arguments);
    }
}
