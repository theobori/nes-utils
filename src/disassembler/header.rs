use std::io::Bytes;

struct Block<T> {
    pub pos: i32,
    pub size: i32,
    pub value: Option<T>
}

impl<T> Block<T> {
    pub fn new(pos: i32, size: i32) -> Self {
        Self {
            pos,
            size,
            value: None,
        }
    }

    pub fn value_from(&self, mem: Vec<u8>) {

    }
}

trait Header {
    fn parse(&self);
    fn dump(&self);
}
/// 16 bytes header
struct NesHeader {
    magic: Block<Bytes<u8>>,
    len_prg_rom: Block<u8>,
    len_chr_rom: Block<u8>,
    f6 : Block<u8>,
    f7: Block<u8>,
    len_prg_ram: Block<u8>,
    f9: Block<u8>,
    f10: Block<u8>,
    reserved: Block<i64>
}

impl NesHeader {
    pub fn new() -> Self {
        Self {
            magic: Block::new(0, 4),
            len_prg_rom: Block::new(0, 0),
            len_chr_rom: Block::new(0, 0),
            f6: Block::new(0, 0),
            f7: Block::new(0, 0),
            len_prg_ram: Block::new(0, 0),
            f9: Block::new(0, 0),
            f10: Block::new(0, 0),
            reserved: Block::new(0, 0)
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
}