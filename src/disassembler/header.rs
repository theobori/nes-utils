use std::collections::HashMap;

struct Block {
    pub pos: usize,
    pub size: usize,
    pub value: Option<Vec<u8>>
}

impl Block {
    pub fn new(pos: usize, size: usize) -> Self {
        Block {
            pos,
            size,
            value: None
        }
    }

    pub fn value_from(
        &mut self,
        mem: Vec<u8>
    ) {
        if mem.len() < self.pos + self.size {
            return
        }
        self.value = Some(mem[self.pos..self.size].to_vec())
    }
}

trait Header {
    fn parse(&self);
    fn dump(&self);
    fn init(&self, mem: Vec<u8>);
}
/// 16 bytes header
pub struct NesHeader {
    fields: HashMap<String, Block>,
    mem: Vec<u8>
}

impl NesHeader {
    pub fn new() -> Self {
        NesHeader {
            fields: HashMap::new(),
            mem: Vec::<u8>::new()
        }
    }
}

impl Header for NesHeader {
    fn parse(&self) {
        todo!()
    }

    fn dump(&self) {
        todo!()
    }

    fn init(&self, mem: Vec<u8>) {
        todo!()
    }
}
