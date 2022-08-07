use crate::{
    disassembler::{
        header::NesHeader,
        line::Line
    },
    utils::{
        error::NesError,
        block::Block,
        opcode::get_nes_opcode,
        util::{
            path_to_name,
            join_bytes,
            create_and_write_file,
        }
    },
    models::{
        header_model::Header,
        nesutil_model::{
            Util,
            Save, NesUtil
        }
    }
};

pub type EquConst = (u16, String);

pub struct NesDisassembler {
    path: String,
    header: NesHeader,
    mem: Vec<u8>,
    const_lines: Vec<EquConst>,
    prg_lines: Vec<Line>,
    pc: usize,
    prg_rom: Block,
    chr_rom: Block
}

impl NesDisassembler {
    pub fn new(path: &String, mem: &Vec<u8>) -> Self {

        Self {
            path: String::from(path),
            header: NesHeader::new(&mem),
            mem: mem.to_vec(),
            const_lines: Vec::new(),
            prg_lines: Vec::new(),
            pc: 0,
            prg_rom: Block::new(NesHeader::HEADER_SIZE, 0),
            chr_rom: Block::new(0, 0)
        }
    }

    fn parse(&mut self) -> &mut Self {
        self.header.parse();
        
        // Header size
        self.pc = NesHeader::HEADER_SIZE;

        // Get header metadata
        self.prg_rom.size = self.header.field("len_prg_rom").value.unwrap()[0] as usize * NesHeader::PRG_ROM_UNIT_SIZE;
        self.chr_rom.size = self.header.field("len_chr_rom").value.unwrap()[0] as usize * NesHeader::CHR_ROM_UNIT_SIZE;

        // Check if there is the trainer (512 bytes)
        if self.header.is_trainer() {
            self.prg_rom.pos += NesHeader::TRAINER_SIZE;
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
        self.prg_lines = Vec::new();

        // Disassemble
        while self.pc < self.prg_rom.size {
            // Check if the opcode has been implemented
            let byte = self.mem[self.pc];
            let code = match get_nes_opcode(&byte) {
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
                opcode: code,
                label: None,
                fmt_arg: String::from(""),
                comment: None
            };
            if let Some(equ) = line.fmt() {
                if self.const_lines.contains(&equ) == false {
                    self.const_lines.push(equ);
                }
            }

            // Fill self.prg_lines
            self.prg_lines.push(line);

            // Next line
            self.pc += code.len as usize;
        }

        self
    }

    fn add_comments(&mut self) -> &mut Self {
        let mut comment;
        let n = self.prg_lines
            .iter()
            .max_by(|x, y| x.cmp(y));
        
        let n = match n {
            Some(line) => line.len(),
            None => 0
        };

        let mut spaces;

        for line in self.prg_lines.iter_mut() {
            spaces = " ".repeat(n - line.len());
            comment = join_bytes(&line.bytes, " ");
            comment = format!("{} ; {}", spaces, comment);
            line.comment = Some(comment);
        }

        self
    }

    fn dump_chr(&mut self, path: &str) -> bool {
        match &self.chr_rom.value {
            Some(data) => {
                create_and_write_file(path, &data);
                true
            },
            None => false
        }
    }
}

impl NesUtil for NesDisassembler { }

impl Util for NesDisassembler {
    fn run(&mut self) {
        self
            .parse()
            .disassemble()
            .add_comments();
    }
}

impl Save for NesDisassembler {
    fn save_as(&mut self, path: &str) {
        let mut line_str = String::from("");
        let name = path_to_name(path);
        
        
        line_str.push_str("; Mapped registers\n\n");
        for (value, name) in &self.const_lines {
            line_str.push_str(&format!("{} equ ${:02x?}\n", name, value));
        }
        
        // Dumping header
        line_str.push_str("\n; Header\n\n");
        line_str.push_str(&format!("{}\n", self.header));

        // Dumping PRG
        line_str.push_str("\n; PRG ROM\n\n");
        
        for line in &self.prg_lines {
            line_str.push_str(&format!("{}", line));
        }
        
        // Dumping CHR
        let chr_path = format!("{}.chr", name);

        if self.dump_chr(&chr_path) {
            line_str.push_str(&format!("\n\n; CHR ROM\n.incbin {}.chr\n", name));
        }

        // Writing bytes to the file
        create_and_write_file(
            &path.to_string(),
            line_str.as_bytes()
        );
    }
    
    fn save(&mut self) {
        let name = path_to_name(&self.path);
        let path = format!("./{}.asm", name);
        
        self.save_as(&path)
    }
}
