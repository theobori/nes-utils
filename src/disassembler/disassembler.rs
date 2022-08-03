use crate::addressing::AddressingMode;
use crate::{
    disassembler::header::{
        Header,
        NesHeader
    },
    error::NesError,
    block::Block,
    opcode::OpCode,
    opcode
};

use std::fmt;
use std::fs::File;
use std::io::{
    Read,
    Write
};
use std::path::Display;

struct Line {
    pub opcode: OpCode,
    pub label: Option<String>,
    pub arg: String
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match &self.label {
            Some(label) => format!("{}:\n", label),
            None => String::from("")
        };

        write!(
            f,
            "{}{} {}\n",
            label,
            self.opcode.mnemonic,
            self.arg
        )
    }
}

pub struct NesDisassembler {
    path: String,
    header: NesHeader,
    mem: Vec<u8>,
    lines: Vec<Line>,
    pc: usize,
    prg_rom: Block,
    chr_rom: Block
}

impl NesDisassembler {
    pub fn new(path: &String) -> Self {
        NesDisassembler {
            path: path.to_string(),
            header: NesHeader::new(),
            mem: Vec::new(),
            lines: Vec::new(),
            pc: 0,
            prg_rom: Block::new(16, 0),
            chr_rom: Block::new(0, 0)
        }
    }

    fn mem_from_data(
        &mut self,
        data: Vec<u8>
    ) {
        self.mem = data;
    }

    pub fn mem_from_file(&mut self) -> &mut Self {
        let f = File::open(&self.path);
        
        match f {
            Ok(mut file) => {
                let mut data = Vec::<u8>::new();

                match file.read_to_end(&mut data) {
                    Ok(_) => self.mem_from_data(data.clone()),
                    Err(_) => panic!("{}", NesError::FileInvalid)
                }
            },
            Err(_) => panic!("{}", NesError::FileNotFound)
        }

        self
    }

    fn parse(&mut self) -> &Self {
        self.header
            .init_header(&self.mem)
            .parse();
        
        // Header size
        self.pc = 16;

        // Get header metadata
        self.prg_rom.size = self.header.get_field("len_prg_rom").size * 0x4000;
        self.chr_rom.size = self.header.get_field("len_prg_rom").size * 0x2000;

        // Check if there is the trainer (512 bytes)
        self.chr_rom.pos = self.prg_rom.size;
        if self.header.is_trainer() {
            self.prg_rom.pos = 528;
            self.pc = 528;
        }

        // Fill the blocks
        self.prg_rom.value_from(&self.mem);
        self.chr_rom.value_from(&self.mem);

        self
    }

    pub fn extract_images(&mut self) -> &Self {
        self.parse();

        self
    }

    pub fn disassemble(&mut self) -> &mut Self {
        // Header parsing + link metadata
        self.parse();
        self.lines = Vec::new();

        // Disassemble
        while self.pc < self.prg_rom.size {
            // Check if the opcode has been implemented
            let byte = self.mem[self.pc];
            let code = match opcode::NES_OP_CODES.get(&byte) {
                Some(value) => value,
                None => panic!(
                    "{} (0x{:02x?})",
                    NesError::NotImplementedOpcode, byte
                )
            };
            
            // Reach argument
            self.pc += 1;

            let range = self.pc..self.pc + (code.len as usize - 1);
            let arg = &mut self.mem[range];

            // Reverse because of the little endian
            arg.reverse();
            let arg = OpCode::arg_to_string(arg);
            let arg = code.mode.fmt_arg(&arg);

            // Fill self.lines
            self.lines.push(
                Line {
                    opcode: *code,
                    label: None,
                    arg 
                }
            );

            // Next line
            self.pc += code.len as usize - 1;
        }

        self
    }

    fn dump_to_file(&mut self, filename: &String) -> &mut Self {
        let mut line_str;

        match File::create(filename) {
            Ok(mut file) => {
                for line in &self.lines {
                    line_str = format!("{}", line);

                    file.write_all(line_str.as_bytes())
                        .expect("Unable to write");
                }
            },
            Err(_) => panic!("{}", NesError::FileInvalid)
        };

        self
    }

    pub fn save(&mut self) -> &mut Self {
        let dir_vec: Vec<&str> = self.path.split("/").collect();
        let name = dir_vec[dir_vec.len() - 1];
        let name_vec: Vec<&str> = name.split(".").collect();
        let path = format!("./{}.asm", name_vec[0]);

        self.dump_to_file(&path)
    }


    pub fn save_as(&mut self, path: &str) -> &mut Self {
        self.dump_to_file(&String::from(path))
    }

}
