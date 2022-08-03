use std::collections::HashMap;

use crate::{
    utils::error::NesError,
    utils::block::Block,
    models::header_model::Header
};

lazy_static! {
    pub static ref NES_HEADER_FIELDS: HashMap<&'static str, Block> = {
        let mut m = HashMap::new();

        m.insert("magic", Block::new(0, 4));
        m.insert("len_prg_rom", Block::new(4, 1));
        m.insert("len_chr_rom", Block::new(5, 1));
        m.insert("f6", Block::new(6, 1));
        m.insert("f7", Block::new(7, 1));
        m.insert("len_prg_ram", Block::new(8, 1));
        m.insert("f9", Block::new(9, 1));
        m.insert("f10", Block::new(10, 1));
        m.insert("reserved", Block::new(11, 5));

        m
    };
}

pub struct NesHeader {
    fields: HashMap<String, Block>,
    mem: Vec<u8>
}

/// 16 bytes header
impl NesHeader {
    pub const HEADER_SIZE: usize = 16;
    pub const TRAINER_SIZE: usize = 512;
    pub const IF_TRAINER_SIZE: usize = NesHeader::HEADER_SIZE + NesHeader::TRAINER_SIZE;

    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            mem: Vec::<u8>::new()
        }
    }

    pub fn get_field(&self, key: &str) -> Block {
        match self.fields.get(key) {
            Some(block) => block.clone(),
            None => Block::default()
        }
    }

    pub fn is_trainer(&self) -> bool {
        let f6 = self.get_field("f6");

            if let Some(value) = f6.value {
                return value[0] & 0b0000_0100 != 0
            }

        false
    }

    pub fn is_chr(&self) -> bool {
        let chr_rom = self.get_field("len_chr_rom");

        if let Some(value) = chr_rom.value {
            return value[0] > 0;
        }

        false
    }
}

impl Header for NesHeader {
    fn parse(&mut self) -> &mut Self {
        for (_, block) in self.fields.iter_mut() {
            block.value_from(&self.mem);
        }
        self.check_magic();
        
        self
    }

    fn dump(&self) -> &Self {
        for (key, block) in &self.fields {
            println!("{} -- {:?}", key, block);
        }

        self
    }

    fn check_magic(&self) {
        let magic = self.fields.get("magic");
        let correct = vec![0x4e, 0x45, 0x53, 0x1a];

        match magic {
            Some(block) => {
                if block.value != Some(correct) {
                    panic!("{}", NesError::WrongNesFormat)
                }
            },
            None => panic!("{}", NesError::HeaderNotParsed)
        }
    }
    
    fn init_header(
        &mut self,
        mem: &Vec<u8>
    ) -> &mut Self {
        // Init header fields

        for (key, value) in NES_HEADER_FIELDS.iter() {
            let block = Block::new(value.pos, value.size);
    
            self.fields.insert(String::from(*key), block);
        }

        // Init header mem
        self.mem = mem[0..16].to_vec();

        self
    }
}
