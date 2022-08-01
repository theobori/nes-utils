use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8
}

lazy_static! {
    static ref OP_CODES: HashMap<u8, OpCode> = {
        let mut m = HashMap::new();

        todo!();

        m
    };
}
