use crate::disassembler::header::NesHeader;
use crate::error::NesError;

use std::fs::File;
use std::io::Read;

pub struct NesDisassembler {
    pub header: NesHeader,
    memory: Vec<u8>
}

impl NesDisassembler {
    pub fn new() -> Self {
        NesDisassembler {
            header: NesHeader::new(),
            memory: Vec::new()
        }
    }

    fn mem_from_data(
        &mut self,
        data: Vec<u8>
    ) {
        self.memory = data;
    }

    pub fn mem_from_file(
        &mut self,
        path: &String
    ) -> &Self {
        let f = File::open(path);
        
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

    fn extract_images(&self) {
        todo!()
    }

    fn print_header(&self) {
        todo!()
    }

    pub fn disassemble(&self) -> &Self{
        self
    }
    
    pub fn print(&self) -> &Self {
        self
    }

}
