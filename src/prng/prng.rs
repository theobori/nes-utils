use crate::models::nesutil_model::{
    Save,
    Util,
    NesUtil
};

/// Manage Nes PRNG
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nes_utils::prng::prng::NesPrng;
///
/// let mut prng = NesPrng::new(13, None);
/// prng.random();
/// 
/// let number = prng.get_number();
/// ```
pub struct NesPrng {
    seed: [u8; 2],
    y: u8,
    a: u8,
    it: Option<u16>
}

impl NesPrng {
    pub fn new(seed: u16, it: Option<u16>) -> Self {
        Self {
            seed: [
                (seed >> 8) as u8,
                (seed & 0xff) as u8
                ],
                y: 0,
                a: 0,
                it
            }
        }
        
        pub fn get_number(&self) -> u8 {
            self.a
        }

        pub fn set_it(&mut self, it: u16) {
            self.it = Some(it);
        }
        
        #[allow(dead_code)]
        fn seed_u16(&self) -> u16 {
            let low = self.seed[1];
            let high = self.seed[0] << 8;
            
            high as u16 | low as u16
        }
        
        fn lsr(&mut self) {
            self.a >>= 1;
        }
        
        fn eor(&mut self, value: u8) {
            self.a ^= value;
    }

    fn lda(&mut self, value: u8) {
        self.a = value;
    }
    
    fn tay(&mut self) {
        self.y = self.a;
    }
    
    fn tya(&mut self) {
        self.a = self.y;
    }
    
    fn asl(&mut self) {
        self.a <<= 1;
    }
    
    /// Linear feedback shift register
    fn overlapped(&mut self) {
        self.lda(self.seed[1]);
        self.tay();
        
        self.lsr();
        self.lsr();
        self.lsr();
        
        // sta seed+1
        self.seed[1] = self.a;
        
        self.lsr();
        
        self.eor(self.seed[1]);
        self.eor(self.seed[0]);

        // sta seed+1
        self.seed[1] = self.a;

        self.tya();

        // sta seed+0
        self.seed[0] = self.a;

        self.asl();
        self.eor(self.seed[0]);
        self.asl();
        self.eor(self.seed[0]);
        self.asl();
        self.asl();
        self.asl();
        self.eor(self.seed[0]);

        // sta seed+0
        self.seed[0] = self.a;
    }

    /// Generating a pseudo random number
    pub fn random(&mut self) -> u8 {
        match self.it {
            Some(it) => {
                for _ in 0..it {
                    self.overlapped();
                }
            },
            None => self.overlapped()
        };

        self.a
    }
}

impl NesUtil for NesPrng { }

impl Util for NesPrng {
    /// Generate a pseudo random number
    fn run(&mut self) {
        self.random();
    }
}

impl Save for NesPrng {
    /// Print the random number
    fn save_as(&mut self, _path: &str) {
        println!("{}", self.a);
    }

    /// Same as `save_as`
    fn save(&mut self) {
        println!("{}", self.a);
    }
}

/// Return a random number between 0 and 0xff
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nes_utils::prng::prng::random;
/// 
/// let seed = 2;
/// let iterations = 14;
/// 
/// let number = random(seed, Some(iterations));
/// ```
pub fn random(seed: u16, it: Option<u16>) -> u8 {
    let mut prng = NesPrng::new(seed, it);

    prng.random();

    prng.get_number()
}
