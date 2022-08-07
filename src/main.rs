mod disassembler;
mod chr;
mod utils;
mod models;
mod prng;
mod game_genie;

#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
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

    /// Output filename
    #[structopt(short, long)]
    output: Option<String>,

    /// PRNG seed
    #[structopt(long)]
    seed: Option<u16>,

    /// PRNG iteration
    #[structopt(long)]
    it: Option<u16>,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>
}

mod cli_args {
    use structopt::StructOpt;

    use crate::{
        models::nesutil_model::NesUtil,
        Opt,
        utils::util::read_file,
        chr::chr::NesChr,
        disassembler::disassembler::NesDisassembler,
        prng::prng::NesPrng
    };

    type Object = Box<dyn NesUtil>;

    pub struct CliArgs {
        opt: Opt,
        pub objs: Vec<Object>
    }

    impl CliArgs {
        pub fn new() -> Self {
            Self {
                opt: Opt::from_args(),
                objs: Vec::new()
            }
        }

        pub fn fill_dump(&mut self) {
            let input = match &self.opt.input {
                Some(value) => value,
                None => return
            };

            let input = input.to_str().unwrap();
            let path = String::from(input);
            let mem = read_file(&path);

            if self.opt.extract_chr {
                self.objs.push(Box::new(NesChr::new(&path, &mem)));
            }
            if self.opt.disassemble {
                self.objs.push(Box::new(NesDisassembler::new(&path, &mem)));
            }
        }

        pub fn fill_prng(&mut self) {
            let seed = match self.opt.seed {
                Some(value) => value,
                None => return
            };
            let mut prng = NesPrng::new(seed, None);
            
            if let Some(it) = self.opt.it {
                prng.set_it(it);
            }
        
            self.objs.push(Box::new(prng));
        }

        pub fn output(&self) -> &Option<String> {
            &self.opt.output
        }
    }
}

fn main() {
    let mut args = cli_args::CliArgs::new();

    args.fill_dump();
    args.fill_prng();

    let output = args.output().clone();

    for obj in args.objs.iter_mut() {
        obj.run();
        
        match output {
            Some(ref path) => obj.save_as(path),
            None => obj.save()
        };
    }
}
