use crate::{
    disassembler::header::NesHeader,
    utils::{
        error::NesError,
        block::Block,
        opcode::OpCode,
        opcode,
        util::{
            path_to_name,
            write_file,
            read_file,
            join_any,
            unwrap_str
        }
    },
    models::{
        header_model::Header,
        nesutil_model::{
            Util,
            Save
        }
    }
};

use std::fmt;
use std::cmp::Ordering;

struct Line {
    pub bytes: Vec<u8>,
    pub opcode: OpCode,
    pub label: Option<String>,
    pub fmt_arg: String,
    pub comment: Option<String>
}

impl Line {
    pub fn len(&self) -> usize {
        let mut ret: usize = 0;

        if let Some(comment) = &self.comment {
            ret += comment.len();
        }
        if let Some(label) = &self.label {
            ret += label.len();
        }

        ret += self.opcode.mnemonic.len() + self.fmt_arg.len();

        ret
    }

    pub fn set_fmt_arg(&mut self) -> &String {
        let mut arg = self.bytes[1..].to_vec();

        // Reverse because of the little endian
        arg.reverse();

        let mut arg = OpCode::arg_to_string(&arg);
        arg = self.opcode.mode.fmt_arg(&arg);

        self.fmt_arg = arg;

        &self.fmt_arg
    }
}

impl Eq for Line { }

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp: i32 = self.len() as i32 - other.len() as i32;
        let ret: Ordering;

        if cmp < 0 {
            ret = Ordering::Less;
        } else if cmp > 0 {
            ret = Ordering::Greater
        } else {
            ret = Ordering::Equal
        }

        ret
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = unwrap_str(&self.label, "", ":\n");
        let comment = unwrap_str(&self.comment, "", "");

        write!(f, "{}{} {}{}\n",
            label,
            self.opcode.mnemonic,
            self.fmt_arg,
            comment
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
        let mut ret = Self {
            path: String::from(path),
            header: NesHeader::new(),
            mem: read_file(path),
            lines: Vec::new(),
            pc: 0,
            prg_rom: Block::new(NesHeader::HEADER_SIZE, 0),
            chr_rom: Block::new(0, 0)
        };

        ret.header.init_header(&ret.mem);

        ret
    }

    fn parse(&mut self) -> &mut Self {
        self.header.parse();
        
        // Header size
        self.pc = NesHeader::HEADER_SIZE;

        // Get header metadata
        self.prg_rom.size = self.header.get_field("len_prg_rom").size * 0x4000;
        self.chr_rom.size = self.header.get_field("len_prg_rom").size * 0x2000;

        // Check if there is the trainer (512 bytes)
        if self.header.is_trainer() {
            self.prg_rom.pos = NesHeader::IF_TRAINER_SIZE;
            self.pc = self.prg_rom.pos;
        }
        self.chr_rom.pos = self.prg_rom.size + self.prg_rom.pos;

        // Fill the blocks
        self.prg_rom.value_from(&self.mem);
        self.chr_rom.value_from(&self.mem);

        self
    }

    fn disassemble(&mut self) -> &mut Self {
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

            let range = self.pc..self.pc + (code.len as usize);
            let code_bytes = &mut self.mem[range];

            let mut line = Line {
                bytes: code_bytes.to_vec(),
                opcode: *code,
                label: None,
                fmt_arg: String::from(""),
                comment: None
            };
            line.set_fmt_arg();

            // Fill self.lines
            self.lines.push(line);

            // Next line
            self.pc += code.len as usize;
        }

        self
    }

    fn add_comments(&mut self) -> &mut Self {
        let mut comment;
        let n = self.lines
            .iter()
            .max_by(|x, y| x.cmp(y));
        
        let n = match n {
            Some(line) => line.len(),
            None => 0
        };

        let mut spaces;

        for line in self.lines.iter_mut() {
            spaces = " ".repeat(n - line.len());
            comment = join_any(&line.bytes, " ");
            comment = format!("{} ; {}", spaces, comment);
            line.comment = Some(comment);
        }

        self
    }
}

impl Util for NesDisassembler {
    fn run(&mut self) -> &mut Self {
        self
        .parse()
        .disassemble()
        .add_comments();

        self
    }
}

impl Save for NesDisassembler {
    fn save_as(&mut self, filename: &str) -> &mut Self {
        let mut line_str = String::from("");

        for line in &self.lines {
            line_str.push_str(&format!("{}", line));
        }

        write_file(
            &filename.to_string(),
            line_str.as_bytes()
        );

        self
    }

    fn save(&mut self) -> &mut Self {
        let name = path_to_name(&self.path);
        let path = format!("./{}.asm", name);

        self.save_as(&path)
    }
}
