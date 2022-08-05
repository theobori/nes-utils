mod disassembler;
mod chr;
mod utils;
mod models;

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use chr::chr::NesChr;
use disassembler::disassembler::NesDisassembler;
use models::nesutil_model::NesUtil;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "nes-utils")]
struct Opt {
    /// Dump CHR ROM graphics data into PNGs
    #[structopt(short, long)]
    extract_chr: bool,

    /// Disassemble a NES file
    #[structopt(short, long)]
    disassemble: bool,

    /// Convert a PNG to CHR ROM
    // #[structopt(short, long)]
    // to_chr: bool,

    /// Output filename base
    #[structopt(short, long)]
    output: Option<String>,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf
}

fn main() {
    let opt = Opt::from_args();
    let path = String::from(opt.input.to_str().unwrap());
    let mut objs = Vec::<Box<dyn NesUtil>>::new();

    if opt.extract_chr {
        objs.push(Box::new(NesChr::new(&path)));
    }
    if opt.disassemble {
        objs.push(Box::new(NesDisassembler::new(&path)));
    }

    for obj in objs.iter_mut() {
        obj.run();
        match opt.output {
            Some(ref path) => obj.save_as(path),
            None => obj.save()
        };
    }
}
