use crate::{
    models::{
        nesutil_model::{
            Save,
            Util
        },
        header_model::Header
    },
    disassembler::header::NesHeader,
    utils::{
        block::Block,
        util::read_file, error::NesError
    }
};

pub struct NesChr {
    path: String,
    header: NesHeader,
    chr_rom: Block,
    mem: Vec<u8>
}

impl NesChr {
    pub fn new(path: &String) -> Self {
        let mut ret = Self {
            path: String::from(path),
            header: NesHeader::new(),
            chr_rom: Block::new(0, 0),
            mem: read_file(path)
        };

        ret.header.init_header(&ret.mem);

        ret
    }

    pub fn new_from_mem(mem: &Vec<u8>) -> Self {
        let mut ret = Self {
            path: String::from("default.asm"),
            header: NesHeader::new(),
            chr_rom: Block::new(0, 0),
            mem: mem.to_vec()
        };

        ret.header.init_header(&ret.mem);

        ret
    }

    fn parse(&mut self) -> &Self {
        self.header.parse();
        
        // Get header metadata
        let prg_rom_size = self.header.get_field("len_prg_rom").size * 0x4000;
        self.chr_rom.size = self.header.get_field("len_chr_rom").size * 0x2000;

        // Check if there is the trainer (512 bytes)
        if self.header.is_trainer() {
            self.chr_rom.pos = NesHeader::IF_TRAINER_SIZE;
        } else {
            self.chr_rom.pos = NesHeader::HEADER_SIZE;
        }
        self.chr_rom.pos += prg_rom_size;

        // Fill the blocks
        self.chr_rom.value_from(&self.mem);

        self
    }

    fn chr_to_img(&self) -> &Self {
        
        self
    }

    pub fn img_to_chr(bytes: &[u8]) {

    }
}

impl Save for NesChr {
    fn save(&mut self) -> &mut Self {
        todo!()
    }

    fn save_as(&mut self, path: &str) -> &mut Self {
        todo!()
    }
}

impl Util for NesChr {
    fn run(&mut self) -> &mut Self {
        self.parse();

        if self.header.is_chr() == false {
            panic!("{}", NesError::MissingChr)
        }

        self.chr_to_img();

        self
    }
}
