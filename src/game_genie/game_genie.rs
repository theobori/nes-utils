use crate::{
    models::nesutil_model::{
        NesUtil,
        Util,
        Save
    }
};

pub struct NesGameGenie {
    code: Vec<u8>,
    value: u8,
    address: u16
}

impl NesGameGenie {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            code,
            value: 0,
            address:  0
        }
    }

    pub fn decode_6(&self) {
        todo!()
    }

    pub fn decode_8(&self) {
        todo!()
    }

    pub fn decode(&self) {
        match self.code.len() {
            6 => self.decode_6(),
            8 => self.decode_8(),
            _ => println!("Invalid code lenght")
        }
    }
}

impl NesUtil for NesGameGenie { }

impl Util for NesGameGenie {
    fn run(&mut self) {
        self.decode();
    }
}

impl Save for NesGameGenie {
    fn save(&mut self) {
        println!("{} {}", self.address, self.value);
    }

    fn save_as(&mut self, _path: &str) {
        println!("{} {}", self.address, self.value);
    }
}
