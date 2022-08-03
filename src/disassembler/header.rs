use std::collections::HashMap;

use crate::{
    error::NesError,
    block::Block
};

pub struct NesHeader {
    fields: HashMap<String, Block>,
    mem: Vec<u8>
}
pub trait Header {
    fn parse(&mut self) -> &mut Self;
    fn dump(&self) -> &Self;
    fn init_header(&mut self, mem: &Vec<u8>) -> &mut Self;
    fn check_magic(&self);
}

/// 16 bytes header
impl NesHeader {
    pub fn new() -> Self {
        NesHeader {
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
        let f6 = self.fields.get("f6");

        if let Some(block) = f6 {
            if let Some(value) = &block.value {
                let byte = value[0];

                return byte & 0b0000_0100 != 0
            }
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
        self.fields.insert(String::from("magic"), Block::new(0, 4));
        self.fields.insert(String::from("len_prg_rom"), Block::new(4, 1));
        self.fields.insert(String::from("len_chr_rom"), Block::new(5, 1));
        self.fields.insert(String::from("f6"), Block::new(6, 1));
        self.fields.insert(String::from("f7"), Block::new(7, 1));
        self.fields.insert(String::from("len_prg_ram"), Block::new(8, 1));
        self.fields.insert(String::from("f9"), Block::new(9, 1));
        self.fields.insert(String::from("f10"), Block::new(10, 1));
        self.fields.insert(String::from("reserved"), Block::new(11, 5));

        // Init header mem
        self.mem = mem[0..16].to_vec();

        self
    }
}
