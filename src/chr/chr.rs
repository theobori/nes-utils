use crate::{
    models::{
        nesutil_model::{
            Save,
            Util, NesUtil
        },
        header_model::Header
    },
    disassembler::header::NesHeader,
    utils::{
        block::Block,
        util::{
            path_to_name
        },
        error::NesError
    }
};

use super::image::NesImage;

/// Interacting with the CHR ROM 
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nes_utils::chr::chr::NesChr;
/// use nes_utils::models::nesutil_model::{Util, Save};
/// 
/// use std::fs::File;
/// use std::io::Read;
/// 
/// let path = String::from("games/game.nes");
/// let mut mem = Vec::<u8>::new();
/// let mut f = File::open(&path).unwrap();
///
/// f.read_to_end(&mut mem);
///
/// let mut chr = NesChr::new(&path, &mem);
/// chr.run();
/// chr.save();
/// ```
pub struct NesChr {
    path: String,
    header: NesHeader,
    chr_rom: Block,
    mem: Vec<u8>,
    images: Vec<NesImage>
}

impl NesChr {
    pub fn new(path: &String, mem: &Vec<u8>) -> Self {
        Self {
            path: String::from(path),
            header: NesHeader::new(&mem),
            chr_rom: Block::new(0, 0),
            mem: mem.to_vec(),
            images: Vec::new()
        }
    }

    fn parse(&mut self) -> &Self {
        self.header.parse();
        
        // Get header metadata
        let prg_rom_size = self.header.field("len_prg_rom").value.unwrap()[0] as usize * NesHeader::PRG_ROM_UNIT_SIZE;
        self.chr_rom.size = self.header.field("len_chr_rom").value.unwrap()[0] as usize * NesHeader::CHR_ROM_UNIT_SIZE;

        // Check if there is the trainer (512 bytes)
        self.chr_rom.pos = NesHeader::HEADER_SIZE;
        if self.header.is_trainer() {
            self.chr_rom.pos += NesHeader::TRAINER_SIZE;
        }

        self.chr_rom.pos += prg_rom_size;

        // Fill the blocks
        self.chr_rom.value_from(&self.mem);

        self
    }

    fn chr_to_img(&mut self) {

        let chr_mem = self.chr_rom.value.as_ref().unwrap();

        for n in 0..2 {

            let idx = n * NesHeader::CHR_ROM_BANK_SIZE;
            let bank = &chr_mem[idx..idx + NesHeader::CHR_ROM_BANK_SIZE];
            let path = format!(
                "{}{}.png",
                path_to_name(&self.path),
                n
            );
                        
            let mut image = NesImage::new(&path);
            image.fill_with_bank(bank);
    
            self.images.push(image);
        }
    }
}

impl NesUtil for NesChr { }

impl Save for NesChr {
    /// Save pixels to the path as argument.
    /// 
    /// Dump the two CHR banks into two PNG files
    fn save_as(&mut self, path: &str) {
        let mut n = 0;

        for image in &mut self.images {
            let path = format!(
                "{}{}.png",
                path_to_name(&path),
                n
            );
            image.save_as(&path);
            n += 1;
        }
    }

    /// Same as `save_as` but with the path stored in the struct.
    fn save(&mut self) {
        self.save_as(&self.path.clone());
    }
}

impl Util for NesChr {
    /// Parse the CHR ROM and dump images into PNGs
    fn run(&mut self) {
        self.parse();

        if self.header.is_chr() == false {
            panic!("{}", NesError::MissingChr)
        }

        self.chr_to_img();
    }
}
