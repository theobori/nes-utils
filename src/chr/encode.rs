use crate::{
    models::nesutil_model::{
        NesUtil,
        Util,
        Save
    },
    disassembler::header::NesHeader
};

#[allow(dead_code)]
const COLOR_SCHEME: [u8; 4] = [
    0,
    126,
    189,
    255,
];

#[allow(dead_code)]
pub struct NesChrEncode {
    path: String,
    rgb_data: Vec<u8>,
    mem: [u8; NesHeader::CHR_ROM_UNIT_SIZE]
}

impl NesChrEncode {
    pub fn new(path: &String) -> Self {

        let img = image::open(path);
        let rgb_data = img.unwrap().to_rgb8().as_raw().to_vec();
        
        Self {
            path: path.to_string(),
            rgb_data,
            mem: [0x00; NesHeader::CHR_ROM_UNIT_SIZE]
        }
    }
}

impl NesUtil for NesChrEncode { }

impl Util for NesChrEncode {
    fn run(&mut self) {
        todo!()
    }
}

impl Save for NesChrEncode {
    fn save(&mut self) {
        todo!()
    }

    fn save_as(&mut self, _path: &str) {
        todo!()
    }
}
