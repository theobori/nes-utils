mod disassembler;
mod chr;
mod utils;
mod models;

use chr::chr::NesChr;
// use disassembler::disassembler::NesDisassembler;
use models::nesutil_model::{
    Util,
    Save
};
use utils::error::NesError;

#[macro_use]
extern crate lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;


use std::env;

#[derive(StructOpt, Debug)]
#[structopt(name = "nes-utils")]
struct Opt {
    /// Dump CHR ROM graphics data into PNGs
    #[structopt(short, long)]
    dump_tilesets: bool,

    /// Disassemble a NES file
    #[structopt(short, long)]
    disassemble: bool,

    /// Convert a PNG to CHR ROM
    #[structopt(short, long)]
    to_chr: bool,

    /// Output filename base
    #[structopt(short, long)]
    output: bool,

    /// Output file
    #[structopt(parse(from_os_str))]
    input: PathBuf
}

fn main() {
    let opt = Opt::from_args();
}
