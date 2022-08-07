use crate::{
    models::nesutil_model::{
        NesUtil,
        Util,
        Save
    }
};

fn letter_to_bin(letter: char) -> Option<u8> {
    let ret = match letter {
        'A' => 0b000_0000,
        'P' => 0b000_0001,
        'Z' => 0b000_0010,
        'L' => 0b000_0011,
        'G' => 0b000_0100,
        'I' => 0b000_0101,
        'T' => 0b000_0110,
        'Y' => 0b000_0111,
        'E' => 0b000_1000,
        'O' => 0b000_1001,
        'X' => 0b000_1010,
        'U' => 0b000_1011,
        'K' => 0b000_1100,
        'S' => 0b000_1101,
        'V' => 0b000_1110,
        'N' => 0b000_1111,
        _ => return None
    };
    
    Some(ret)
}

type Cell = [char; 4];

const VALUE_ORDER: &'static str = "12345678";
const ADDRESS_ORDER: &'static str = "ABCDEFGHIJKLMNO";
const COMPARE_VALUE_ORDER: &'static str = "!@#$%^&*";

const TABLE_6: [Cell; 6] = [
    ['1', '6', '7', '8'],
    ['H', '2', '3', '4'],
    ['-', 'I', 'J', 'K'],
    ['L', 'A', 'B', 'C'],
    ['D', 'M', 'N', 'O'],
    ['5', 'E', 'F', 'G']
];

const TABLE_8: [Cell; 8] = [
    ['1', '6', '7', '8'],
    ['H', '2', '3', '4'],
    ['-', 'I', 'J', 'K'],
    ['L', 'A', 'B', 'C'],
    ['D', 'M', 'N', 'O'],
    ['%', 'E', 'F', 'G'],
    ['!', '^', '&', '*'],
    ['5', '@', '#', '$']
];

fn code_pos(symbol: char, table: &[Cell]) -> (usize, usize) {
    for y in 0..table.len() {
        for x in 0..table[y].len() {
            if table[y][x] == symbol {
                return (x, y);
            }
        }
    }

    (9, 4)
}

fn convert_code(code: &String) -> Vec<u8> {
    let mut ret = Vec::new();

    for letter in code.chars() {
        match letter_to_bin(letter) {
            Some(value) => ret.push(value),
            None => panic!("Invalid letter")
        };
    }

    ret
}

fn get_value(code: &[char], order: &str, table: &[Cell]) -> usize {
    let mut ret: usize = 0;

    // Iterate throught the needed order
    for symbol in order.chars() {
        // Get the location
        let (x, y) = code_pos(symbol, table);

        // Linked code letter
        let letter = code[y as usize];

        // Letter value
        match letter_to_bin(letter) {
            Some(letter_value) => {
                // Get the good bit value
                let bit = letter_value >> (3 - x) & 1;
                ret = (ret << 1) | bit as usize;
            },
            None => continue
        };
    }

    ret
}

/// Interacting with Game Genie code
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nes_utils::game_genie::game_genie::NesGameGenie;
/// 
/// // Pac Man - Both players have infinite lives
/// let code = String::from("SZEKKIVG");
/// 
/// let mut d = NesGameGenie::new(code);
/// d.decode();
/// 
/// let address = d.address;
/// let value = d.value;
/// let compare_value = d.compare;
/// ```
pub struct NesGameGenie {
    code: String,
    code_genie: Vec<u8>,
    pub value: u8,
    pub address: u16,
    pub compare: Option<u8>
}

impl NesGameGenie {
    pub fn new(code: String) -> Self {
        Self {
            code,
            code_genie: Vec::new(),
            value: 0,
            address: 0,
            compare: None
        }
    }

    fn decode_with_table(&mut self, table: &[Cell]) {
        let code: Vec<char> = self.code.chars().collect();

        self.value = get_value(&code, VALUE_ORDER, table) as u8;
        self.address = get_value(&code, ADDRESS_ORDER, table) as u16;

        if code.len() == 8 {
            let value = get_value(&code, COMPARE_VALUE_ORDER, table) as u8;
            self.compare = Some(value);
        }
    }

    /// Decoding the Game Genie code
    pub fn decode(&mut self) {
        self.code_genie = convert_code(&self.code);

        match self.code.len() {
            6 => self.decode_with_table(&TABLE_6),
            8 => self.decode_with_table(&TABLE_8),
            _ => panic!("Invalid code lenght")
        }
    }
}

impl NesUtil for NesGameGenie { }

impl Util for NesGameGenie {
    /// Decoding the Game Genie code
    fn run(&mut self) {
        self.decode();
    }
}

impl Save for NesGameGenie {
    /// Dumping in a formatted way
    fn save_as(&mut self, _path: &str) {
        println!("Address 0x{:02x?}", self.address);
        println!("Value 0x{:02x?}", self.value);

        if let Some(compare) = self.compare {
            println!("Compare value 0x{:02x?}", compare);
        }
    }

    /// Same as `save_as`
    fn save(&mut self) {
        self.save_as("");
    }
}

/// Decoding a Game Genie code
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use nes_utils::game_genie::game_genie::decode;
/// 
/// // Pac Man - Both players have infinite lives
/// let code = String::from("SZEKKIVG");
/// 
/// let data = decode(code);
/// 
/// let address = data.address;
/// let value = data.value;
/// let compare_value = data.compare;
/// ```
pub fn decode(code: String) -> NesGameGenie {
    let mut d = NesGameGenie::new(code);
    d.decode();

    d
}
