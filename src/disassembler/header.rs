use core::fmt;
use std::collections::HashMap;

use crate::{
    utils::error::NesError,
    utils::{
        block::Block,
        util::vec_bytes_to_string
    },
    models::header_model::Header
};

const NES_HEADER_FIELDS_ORDER: [(&str, usize, usize); 9] = {
    [
        ("magic", 0, 4),
        ("len_prg_rom", 4, 1),
        ("len_chr_rom", 5, 1),
        ("f6", 6, 1),
        ("f7", 7, 1),
        ("len_prg_ram", 8, 1),
        ("f9", 9, 1),
        ("f10", 10, 1),
        ("reserved", 11, 5)
    ]
};

lazy_static! {
    pub static ref NES_HEADER_FIELDS: HashMap<&'static str, Block> = {
        let mut m = HashMap::new();

        for (name, pos, size) in NES_HEADER_FIELDS_ORDER {
            m.insert(name, Block::new(pos, size));
        }

        m
    };
}

pub struct NesHeader {
    fields: HashMap<String, Block>,
    mem: Vec<u8>
}

impl NesHeader {
    pub const HEADER_SIZE: usize = 16;
    pub const TRAINER_SIZE: usize = 512;
    pub const PRG_ROM_UNIT_SIZE: usize = 0x4000;
    pub const CHR_ROM_UNIT_SIZE: usize = 0x2000;
    pub const CHR_ROM_BANK_SIZE: usize = 0x1000;

    pub fn new(mem: &Vec<u8>) -> Self {
        let mut fields = HashMap::new();

        for (key, value) in NES_HEADER_FIELDS.iter() {
            let block = Block::new(value.pos, value.size);
    
            fields.insert(String::from(*key), block);
        }

        Self {
            fields,
            mem: mem[0..16].to_vec()
        }
    }

    pub fn field(&self, key: &str) -> Block {
        match self.fields.get(key) {
            Some(block) => block.clone(),
            None => Block::default()
        }
    }

    pub fn is_trainer(&self) -> bool {
        let f6 = self.field("f6");

            if let Some(value) = f6.value {
                return value[0] & 0b0000_0100 != 0
            }

        false
    }

    pub fn is_chr(&self) -> bool {
        let chr_rom = self.field("len_chr_rom");

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
}

impl fmt::Display for NesHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = Vec::<String>::new();

        for (name, _, _) in NES_HEADER_FIELDS_ORDER {
            let bytes = match self.fields.get(name) {
                Some(field) => match &field.value {
                    Some(v) => v.clone(),
                    None => vec![0x00]
                },
                None => continue
            };

            let bytes_str = vec_bytes_to_string(&bytes);
            let line = format!("hex {}", bytes_str);

            msg.push(line);            
        }

        write!(f, "{}", msg.join("\n"))
    }
}
