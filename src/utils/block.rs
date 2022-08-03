#[derive(Debug, Clone)]
pub struct Block {
    pub pos: usize,
    pub size: usize,
    pub value: Option<Vec<u8>>
}

impl Block {
    pub fn new(pos: usize, size: usize) -> Self {
        Self {
            pos,
            size,
            value: None
        }
    }

    pub fn default() -> Self {
        Block {
            pos: 0,
            size: 0,
            value: None
        }
    }

    pub fn value_from(
        &mut self,
        mem: &Vec<u8>
    ) {
        if mem.len() < self.pos + self.size {
            return;
        }

        let dest = self.pos + self.size;
        self.value = Some(mem[self.pos..dest].to_vec())
    }
}
