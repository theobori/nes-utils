# nes-utils

## How to build and run ?

1. Install the dependencies
    - `cargo`

## Usage example

```rust
use std::fs::File;
use std::io::Read;

use nes_utils::disassembler::disassembler::NesDisassembler;
use nes_utils::models::nesutil_model::{Util, Save};

fn main() {
    // Getting program bytes
    let path = String::from("games/pacman.nes");
    let mut mem = Vec::<u8>::new();
    let mut f = File::open(&path).unwrap();

    f.read_to_end(&mut mem);

    // Disassembling the NES file and save into a file
    let mut disas = NesDisassembler::new(&path, &mem);
    disas.run();
    disas.save(); // or disas.save_as("other_path.asm");

    // Disassembling the NES file
    // Dumping Header + PRG ROM (withouth CHR ROM)
    // let mut disas = NesDisassembler::new(&path, &mem);
    // disas.run();
    // disas.dump();
}
```

## Documentation

Run `cargo doc --open` to read the documentation in the browser.

## Features status

Name           | Status
-------------  |:-------------:
Dump header into .asm | ✅
Include .chr into .asm | ✅
Disassemble instructions | ⌛
PPU / 2A03 registers indication for asm code | ✅
Dump CHR ROM graphics data into images | ✅
Dump image into CHR ROM | ⌛
NES PRNG | ✅
NES Game Genie decode | ✅
