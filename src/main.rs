mod disassembler;
mod chr;
mod utils;
mod models;

#[macro_use]
extern crate lazy_static;

use std::env;

use disassembler::disassembler::NesDisassembler;
use models::nesutil_model::{
    Util,
    Save
};
use utils::error::NesError;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_count = args.len();

    if args_count < 2 {
        panic!("{}", NesError::CliArguments);
    }

    NesDisassembler::new(&args[1])
        .run()
        .save_as("mario.asm");
}
