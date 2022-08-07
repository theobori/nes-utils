extern crate image;

use std::path::Path;

use crate::models::nesutil_model::Save;

type Rgb = (u8, u8, u8);

const BLACK_PIXEL: Rgb = (0, 0, 0);
const COLOR_SCHEME: [Rgb; 4] = [
    (0, 0, 0),
    (126, 126, 126),
    (189, 189, 189),
    (255, 255, 255)
];

fn bits_to_rgb(left: u8, right: u8) -> Rgb {
    let color = right << 1 | left;

    COLOR_SCHEME[color as usize]
}

pub struct NesImage {
    path: String,
    mem: Vec<Rgb>
}

impl NesImage {
    const W: usize = 128;
    const H: usize = 128;
    const TILE_W: usize = 8;
    const TILE_H: usize = 8;

    pub fn new(path: &String) -> Self {
        Self {
            path: path.to_string(),
            mem: vec![BLACK_PIXEL; NesImage::W * NesImage::H]
        }
    }

    fn put_pixel(&mut self, x: usize, y: usize, pixel: Rgb) {
        let pos = y * NesImage::W + x;

        self.mem[pos] = pixel;
    }

    pub fn fill_with_bank(&mut self, bank: &[u8]) {
        let mut mem_x = 0;
        let mut mem_y = 0;

        for byte in (0..bank.len()).step_by(16) {
            for y in 0..8 {
                if mem_x >= NesImage::W {
                    mem_y += NesImage::TILE_H;
                    mem_x = 0;
                }

                let lower = bank[byte + y];
                let upper = bank[byte + y + 8];

                for bit in 0..8 {
                    let pixel = bits_to_rgb(
                        lower >> (7 - bit) & 1,
                        upper >> (7 - bit) & 1
                    );
                    self.put_pixel(bit + mem_x, y + mem_y, pixel);
                }
            }
            mem_x += NesImage::TILE_W;
        }
    }
}

impl Save for NesImage {
    fn save(&mut self) {
        self.save_as(&self.path.clone());
    }

    fn save_as(&mut self, path: &str) {
        let mut buffer: Vec<u8> = Vec::new();

        for pixel in &self.mem {
            let (r, g, b) = pixel;

            buffer.push(*r);
            buffer.push(*g);
            buffer.push(*b);
        }

        image::save_buffer(
            &Path::new(&path),
            &buffer,
            NesImage::W as u32,
            NesImage::H as u32,
            image::ColorType::Rgb8
        ).expect("Unable to dump this bank");
    }
}
