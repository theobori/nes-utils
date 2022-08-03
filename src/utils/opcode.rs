use std::collections::HashMap;

use crate::utils::addressing::AddressingMode;

#[derive(Clone, Copy)]
pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub mode: AddressingMode
}

impl OpCode {
    fn new(
        code: u8,
        mnemonic: &'static str,
        len: u8,
        mode: AddressingMode
) -> Self {
        Self { code, mnemonic, len, mode }
    }

    pub fn arg_to_string(arg_bytes: &[u8]) -> String {
        let mut ret = String::from("");

        for byte in arg_bytes {
            let byte_str = format!("{:02x?}", byte);
            ret.push_str(&byte_str);
        }

        ret
    }
}

lazy_static! {
    pub static ref NES_OP_CODES: HashMap<u8, OpCode> = {
        let mut m = HashMap::new();

        m.insert(0x69, OpCode::new(0x69, "adc", 2, AddressingMode::Immediate));
        m.insert(0x65, OpCode::new(0x65, "adc", 2, AddressingMode::ZeroPage));
        m.insert(0x75, OpCode::new(0x75, "adc", 2, AddressingMode::ZeroPageX));
        m.insert(0x6d, OpCode::new(0x6d, "adc", 3, AddressingMode::Absolute));
        m.insert(0x7d, OpCode::new(0x7d, "adc", 3, AddressingMode::AbsoluteX));
        m.insert(0x79, OpCode::new(0x79, "adc", 3, AddressingMode::AbsoluteY));
        m.insert(0x61, OpCode::new(0x61, "adc", 2, AddressingMode::IndexedIndirect));
        m.insert(0x71, OpCode::new(0x71, "adc", 2, AddressingMode::IndirectIndexed));

        m.insert(0x29, OpCode::new(0x29, "and", 2, AddressingMode::Immediate));
        m.insert(0x25, OpCode::new(0x25, "and", 2, AddressingMode::ZeroPage));
        m.insert(0x35, OpCode::new(0x35, "and", 2, AddressingMode::ZeroPageX));
        m.insert(0x2d, OpCode::new(0x2d, "and", 3, AddressingMode::Absolute));
        m.insert(0x3d, OpCode::new(0x3d, "and", 3, AddressingMode::AbsoluteX));
        m.insert(0x39, OpCode::new(0x39, "and", 3, AddressingMode::AbsoluteY));
        m.insert(0x21, OpCode::new(0x21, "and", 2, AddressingMode::IndexedIndirect));
        m.insert(0x31, OpCode::new(0x31, "and", 2, AddressingMode::IndirectIndexed));

        m.insert(0x0a, OpCode::new(0x0a, "asl", 1, AddressingMode::Implied));
        m.insert(0x06, OpCode::new(0x06, "asl", 2, AddressingMode::ZeroPage));
        m.insert(0x16, OpCode::new(0x16, "asl", 2, AddressingMode::ZeroPageX));
        m.insert(0x0e, OpCode::new(0x0e, "asl", 3, AddressingMode::Absolute));
        m.insert(0x1e, OpCode::new(0x1e, "asl", 3, AddressingMode::AbsoluteX));

        m.insert(0x90, OpCode::new(0x90, "bcc", 2, AddressingMode::Implied));

        m.insert(0xb0, OpCode::new(0xb0, "bcs", 2, AddressingMode::Implied));

        m.insert(0xf0, OpCode::new(0xf0, "beq", 2, AddressingMode::Implied));

        m.insert(0x24, OpCode::new(0x24, "bit", 2, AddressingMode::ZeroPage));
        m.insert(0x2c, OpCode::new(0x2c, "bit", 3, AddressingMode::Absolute));

        m.insert(0x30, OpCode::new(0x30, "bmi", 2, AddressingMode::Implied));

        m.insert(0xd0, OpCode::new(0xd0, "bne", 2, AddressingMode::Implied));

        m.insert(0x10, OpCode::new(0x10, "bpl", 2, AddressingMode::Implied));

        m.insert(0x00, OpCode::new(0x00, "brk", 1, AddressingMode::Implied));

        m.insert(0x50, OpCode::new(0x50, "bvc", 2, AddressingMode::Implied));

        m.insert(0x70, OpCode::new(0x70, "bvs", 2, AddressingMode::Implied));

        m.insert(0x18, OpCode::new(0x18, "clc", 1, AddressingMode::Implied));

        m.insert(0xD8, OpCode::new(0xD8, "cld", 1, AddressingMode::Implied));

        m.insert(0x58, OpCode::new(0x58, "cli", 1, AddressingMode::Implied));

        m.insert(0xb8, OpCode::new(0xb8, "clv", 1, AddressingMode::Implied));

        m.insert(0xc9, OpCode::new(0xc9, "cmp", 2, AddressingMode::Immediate));
        m.insert(0xc5, OpCode::new(0xc5, "cmp", 2, AddressingMode::ZeroPage));
        m.insert(0xd5, OpCode::new(0xd5, "cmp", 2, AddressingMode::ZeroPageX));
        m.insert(0xcd, OpCode::new(0xcd, "cmp", 3, AddressingMode::Absolute));
        m.insert(0xdd, OpCode::new(0xdd, "cmp", 3, AddressingMode::AbsoluteX));
        m.insert(0xd9, OpCode::new(0xd9, "cmp", 3, AddressingMode::AbsoluteY));
        m.insert(0xc1, OpCode::new(0xc1, "cmp", 2, AddressingMode::IndexedIndirect));
        m.insert(0xd1, OpCode::new(0xd1, "cmp", 2, AddressingMode::IndirectIndexed));

        m.insert(0xe0, OpCode::new(0xe0, "cpx", 2, AddressingMode::Immediate));
        m.insert(0xe4, OpCode::new(0xe4, "cpx", 2, AddressingMode::ZeroPage));
        m.insert(0xec, OpCode::new(0xec, "cpx", 3, AddressingMode::Absolute));

        m.insert(0xc0, OpCode::new(0xc0, "cpy", 2, AddressingMode::Immediate));
        m.insert(0xc4, OpCode::new(0xc4, "cpy", 2, AddressingMode::ZeroPage));
        m.insert(0xcc, OpCode::new(0xcc, "cpy", 3, AddressingMode::Absolute));

        m.insert(0xc6, OpCode::new(0xc6, "dec", 2, AddressingMode::ZeroPage));
        m.insert(0xd6, OpCode::new(0xd6, "dec", 2, AddressingMode::ZeroPageX));
        m.insert(0xce, OpCode::new(0xce, "dec", 3, AddressingMode::Absolute));
        m.insert(0xde, OpCode::new(0xde, "dec", 3, AddressingMode::AbsoluteX));

        m.insert(0xca, OpCode::new(0xca, "dex", 1, AddressingMode::Implied));

        m.insert(0x88, OpCode::new(0x88, "dey", 1, AddressingMode::Implied));

        m.insert(0x49, OpCode::new(0x49, "eor", 2, AddressingMode::Immediate));
        m.insert(0x45, OpCode::new(0x45, "eor", 2, AddressingMode::ZeroPage));
        m.insert(0x55, OpCode::new(0x55, "eor", 2, AddressingMode::ZeroPageX));
        m.insert(0x4d, OpCode::new(0x4d, "eor", 3, AddressingMode::Absolute));
        m.insert(0x5d, OpCode::new(0x5d, "eor", 3, AddressingMode::AbsoluteX));
        m.insert(0x59, OpCode::new(0x59, "eor", 3, AddressingMode::AbsoluteY));
        m.insert(0x41, OpCode::new(0x41, "eor", 2, AddressingMode::IndexedIndirect));
        m.insert(0x51, OpCode::new(0x51, "eor", 2, AddressingMode::IndirectIndexed));

        m.insert(0xe6, OpCode::new(0xe6, "inc", 2, AddressingMode::ZeroPage));
        m.insert(0xf6, OpCode::new(0xf6, "inc", 2, AddressingMode::ZeroPageX));
        m.insert(0xee, OpCode::new(0xee, "inc", 3, AddressingMode::Absolute));
        m.insert(0xfe, OpCode::new(0xfe, "inc", 3, AddressingMode::AbsoluteX));

        m.insert(0xe8, OpCode::new(0xe8, "inx", 1, AddressingMode::Implied));

        m.insert(0xc8, OpCode::new(0xc8, "iny", 1, AddressingMode::Implied));

        m.insert(0x4c, OpCode::new(0x4c, "jmp", 3, AddressingMode::Absolute)); 
        m.insert(0x6c, OpCode::new(0x6c, "jmp", 3, AddressingMode::Indirect));

        m.insert(0x20, OpCode::new(0x20, "jsr", 3, AddressingMode::Implied));

        m.insert(0xa9, OpCode::new(0xa9, "lda", 2, AddressingMode::Immediate));
        m.insert(0xa5, OpCode::new(0xa5, "lda", 2, AddressingMode::ZeroPage));
        m.insert(0xb5, OpCode::new(0xb5, "lda", 2, AddressingMode::ZeroPageX));
        m.insert(0xad, OpCode::new(0xad, "lda", 3, AddressingMode::Absolute));
        m.insert(0xbd, OpCode::new(0xbd, "lda", 3, AddressingMode::AbsoluteX));
        m.insert(0xb9, OpCode::new(0xb9, "lda", 3, AddressingMode::AbsoluteY));
        m.insert(0xa1, OpCode::new(0xa1, "lda", 2, AddressingMode::IndexedIndirect));
        m.insert(0xb1, OpCode::new(0xb1, "lda", 2, AddressingMode::IndirectIndexed));
    
        m.insert(0xa2, OpCode::new(0xa2, "ldx", 2, AddressingMode::Immediate));
        m.insert(0xa6, OpCode::new(0xa6, "ldx", 2, AddressingMode::ZeroPage));
        m.insert(0xb6, OpCode::new(0xb6, "ldx", 2, AddressingMode::ZeroPageY));
        m.insert(0xae, OpCode::new(0xae, "ldx", 3, AddressingMode::Absolute));
        m.insert(0xbe, OpCode::new(0xbe, "ldx", 3, AddressingMode::AbsoluteY));

        m.insert(0xa0, OpCode::new(0xa0, "ldy", 2, AddressingMode::Immediate));
        m.insert(0xa4, OpCode::new(0xa4, "ldy", 2, AddressingMode::ZeroPage));
        m.insert(0xb4, OpCode::new(0xb4, "ldy", 2, AddressingMode::ZeroPageX));
        m.insert(0xac, OpCode::new(0xac, "ldy", 3, AddressingMode::Absolute));
        m.insert(0xbc, OpCode::new(0xbc, "ldy", 3, AddressingMode::AbsoluteX));

        m.insert(0x4a, OpCode::new(0x4a, "lsr", 1, AddressingMode::Implied));
        m.insert(0x46, OpCode::new(0x46, "lsr", 2, AddressingMode::ZeroPage));
        m.insert(0x56, OpCode::new(0x56, "lsr", 2, AddressingMode::ZeroPageX));
        m.insert(0x4e, OpCode::new(0x4e, "lsr", 3, AddressingMode::Absolute));
        m.insert(0x5e, OpCode::new(0x5e, "lsr", 3, AddressingMode::AbsoluteX));
        
        // nop
        m.insert(0xea, OpCode::new(0xea, "nop", 3, AddressingMode::AbsoluteX));

        m.insert(0x09, OpCode::new(0x09, "ora", 2, AddressingMode::Immediate));
        m.insert(0x05, OpCode::new(0x05, "ora", 2, AddressingMode::ZeroPage));
        m.insert(0x15, OpCode::new(0x15, "ora", 2, AddressingMode::ZeroPageX));
        m.insert(0x0d, OpCode::new(0x0d, "ora", 3, AddressingMode::Absolute));
        m.insert(0x1d, OpCode::new(0x1d, "ora", 3, AddressingMode::AbsoluteX));
        m.insert(0x19, OpCode::new(0x19, "ora", 3, AddressingMode::AbsoluteY));
        m.insert(0x01, OpCode::new(0x01, "ora", 2, AddressingMode::IndexedIndirect));
        m.insert(0x11, OpCode::new(0x11, "ora", 2, AddressingMode::IndirectIndexed));

        m.insert(0x48, OpCode::new(0x48, "pha", 1, AddressingMode::Implied));

        m.insert(0x08, OpCode::new(0x08, "php", 1, AddressingMode::Implied));

        m.insert(0x68, OpCode::new(0x68, "pla", 1, AddressingMode::Implied));

        m.insert(0x28, OpCode::new(0x28, "plp", 1, AddressingMode::Implied));

        m.insert(0x2a, OpCode::new(0x2a, "rol", 1, AddressingMode::Implied));
        m.insert(0x26, OpCode::new(0x26, "rol", 2, AddressingMode::ZeroPage));
        m.insert(0x36, OpCode::new(0x36, "rol", 2, AddressingMode::ZeroPageX));
        m.insert(0x2e, OpCode::new(0x2e, "rol", 3, AddressingMode::Absolute));
        m.insert(0x3e, OpCode::new(0x3e, "rol", 3, AddressingMode::AbsoluteX));

        m.insert(0x6a, OpCode::new(0x6a, "ror", 1, AddressingMode::Implied));
        m.insert(0x66, OpCode::new(0x66, "ror", 2, AddressingMode::ZeroPage));
        m.insert(0x76, OpCode::new(0x76, "ror", 2, AddressingMode::ZeroPageX));
        m.insert(0x6e, OpCode::new(0x6e, "ror", 3, AddressingMode::Absolute));
        m.insert(0x7e, OpCode::new(0x7e, "ror", 3, AddressingMode::AbsoluteX));

        m.insert(0x40, OpCode::new(0x40, "rti", 1, AddressingMode::Implied));

        m.insert(0x60, OpCode::new(0x60, "rts", 1, AddressingMode::Implied));

        m.insert(0xe9, OpCode::new(0xe9, "sbc", 2, AddressingMode::Immediate));
        m.insert(0xe5, OpCode::new(0xe5, "sbc", 2, AddressingMode::ZeroPage));
        m.insert(0xf5, OpCode::new(0xf5, "sbc", 2, AddressingMode::ZeroPageX));
        m.insert(0xed, OpCode::new(0xed, "sbc", 3, AddressingMode::Absolute));
        m.insert(0xfd, OpCode::new(0xfd, "sbc", 3, AddressingMode::AbsoluteX));
        m.insert(0xf9, OpCode::new(0xf9, "sbc", 3, AddressingMode::AbsoluteY));
        m.insert(0xe1, OpCode::new(0xe1, "sbc", 2, AddressingMode::IndexedIndirect));
        m.insert(0xf1, OpCode::new(0xf1, "sbc", 2, AddressingMode::IndirectIndexed));

        m.insert(0x38, OpCode::new(0x38, "sec", 1, AddressingMode::Implied));

        m.insert(0xf8, OpCode::new(0xf8, "sed", 1, AddressingMode::Implied));

        m.insert(0x78, OpCode::new(0x78, "sei", 1, AddressingMode::Implied));

        m.insert(0x85, OpCode::new(0x85, "sta", 2, AddressingMode::ZeroPage));
        m.insert(0x95, OpCode::new(0x95, "sta", 2, AddressingMode::ZeroPageX));
        m.insert(0x8d, OpCode::new(0x8d, "sta", 3, AddressingMode::Absolute));
        m.insert(0x9d, OpCode::new(0x9d, "sta", 3, AddressingMode::AbsoluteX));
        m.insert(0x99, OpCode::new(0x99, "sta", 3, AddressingMode::AbsoluteY));
        m.insert(0x81, OpCode::new(0x81, "sta", 2, AddressingMode::IndexedIndirect));
        m.insert(0x91, OpCode::new(0x91, "sta", 2, AddressingMode::IndirectIndexed));

        m.insert(0x86, OpCode::new(0x86, "stx", 2, AddressingMode::ZeroPage));
        m.insert(0x96, OpCode::new(0x96, "stx", 2, AddressingMode::ZeroPageY));
        m.insert(0x8e, OpCode::new(0x8e, "stx", 3, AddressingMode::Absolute));

        m.insert(0x84, OpCode::new(0x84, "sty", 2, AddressingMode::ZeroPage));
        m.insert(0x94, OpCode::new(0x94, "sty", 2, AddressingMode::ZeroPageX));
        m.insert(0x8c, OpCode::new(0x8c, "sty", 3, AddressingMode::Absolute));

        m.insert(0xaa, OpCode::new(0xaa, "tax", 1, AddressingMode::Implied));

        m.insert(0xa8, OpCode::new(0xa8, "tay", 1, AddressingMode::Implied));

        m.insert(0xba, OpCode::new(0xba, "tsx", 1, AddressingMode::Implied));

        m.insert(0x8a, OpCode::new(0x8a, "txa", 1, AddressingMode::Implied));

        m.insert(0x9a, OpCode::new(0x9a, "txs", 1, AddressingMode::Implied));

        m.insert(0x98, OpCode::new(0x98, "tya", 1, AddressingMode::Implied));


        // Unofficials


        m.insert(0x93, OpCode::new(0x93, "ahx", 2, AddressingMode::IndirectIndexed));
        m.insert(0x9f, OpCode::new(0x9f, "ahx", 3, AddressingMode::AbsoluteY));

        m.insert(0x4b, OpCode::new(0x4b, "alr", 2, AddressingMode::Immediate));

        m.insert(0x0b, OpCode::new(0x0b, "anc", 2, AddressingMode::Immediate));
        m.insert(0x2b, OpCode::new(0x2b, "anc", 2, AddressingMode::Immediate));

        m.insert(0x6B, OpCode::new(0x6B, "arr", 2, AddressingMode::Immediate));

        m.insert(0xCB, OpCode::new(0xCB, "axs", 2, AddressingMode::Immediate));

        m.insert(0xc7, OpCode::new(0xc7, "dcp", 2, AddressingMode::ZeroPage));
        m.insert(0xd7, OpCode::new(0xd7, "dcp", 2, AddressingMode::ZeroPageX));
        m.insert(0xCF, OpCode::new(0xCF, "dcp", 3, AddressingMode::Absolute));
        m.insert(0xdF, OpCode::new(0xdF, "dcp", 3, AddressingMode::AbsoluteX));
        m.insert(0xdb, OpCode::new(0xdb, "dcp", 3, AddressingMode::AbsoluteY));
        m.insert(0xd3, OpCode::new(0xd3, "dcp", 2, AddressingMode::IndirectIndexed));
        m.insert(0xc3, OpCode::new(0xc3, "dcp", 2, AddressingMode::IndexedIndirect));

        m.insert(0xe7, OpCode::new(0xe7, "isb", 2, AddressingMode::ZeroPage));
        m.insert(0xf7, OpCode::new(0xf7, "isb", 2, AddressingMode::ZeroPageX));
        m.insert(0xef, OpCode::new(0xef, "isb", 3, AddressingMode::Absolute));
        m.insert(0xff, OpCode::new(0xff, "isb", 3, AddressingMode::AbsoluteX));
        m.insert(0xfb, OpCode::new(0xfb, "isb", 3, AddressingMode::AbsoluteY));
        m.insert(0xe3, OpCode::new(0xe3, "isb", 2, AddressingMode::IndexedIndirect));
        m.insert(0xf3, OpCode::new(0xf3, "isb", 2, AddressingMode::IndirectIndexed));

        m.insert(0xbb, OpCode::new(0xbb, "las", 3, AddressingMode::AbsoluteY));

        m.insert(0xa7, OpCode::new(0xa7, "lax", 2, AddressingMode::ZeroPage));
        m.insert(0xb7, OpCode::new(0xb7, "lax", 2, AddressingMode::ZeroPageY));
        m.insert(0xaf, OpCode::new(0xaf, "lax", 3, AddressingMode::Absolute));
        m.insert(0xbf, OpCode::new(0xbf, "lax", 3, AddressingMode::AbsoluteY));
        m.insert(0xa3, OpCode::new(0xa3, "lax", 2, AddressingMode::IndexedIndirect));
        m.insert(0xb3, OpCode::new(0xb3, "lax", 2, AddressingMode::IndirectIndexed));
        m.insert(0xab, OpCode::new(0xab, "lxa", 2, AddressingMode::Immediate));

        // nop
        m.insert(0x80, OpCode::new(0x80, "nop", 2, AddressingMode::Immediate));
        m.insert(0x82, OpCode::new(0x82, "nop", 2, AddressingMode::Immediate));
        m.insert(0x89, OpCode::new(0x89, "nop", 2, AddressingMode::Immediate));
        m.insert(0xc2, OpCode::new(0xc2, "nop", 2, AddressingMode::Immediate));
        m.insert(0xe2, OpCode::new(0xe2, "nop", 2, AddressingMode::Immediate));
        m.insert(0x04, OpCode::new(0x04, "nop", 2, AddressingMode::ZeroPage));
        m.insert(0x44, OpCode::new(0x44, "nop", 2, AddressingMode::ZeroPage));
        m.insert(0x64, OpCode::new(0x64, "nop", 2, AddressingMode::ZeroPage));
        m.insert(0x14, OpCode::new(0x14, "nop", 2, AddressingMode::ZeroPageX));
        m.insert(0x34, OpCode::new(0x34, "nop", 2, AddressingMode::ZeroPageX));
        m.insert(0x54, OpCode::new(0x54, "nop", 2, AddressingMode::ZeroPageX));
        m.insert(0x74, OpCode::new(0x74, "nop", 2, AddressingMode::ZeroPageX));
        m.insert(0xd4, OpCode::new(0xd4, "nop", 2, AddressingMode::ZeroPageX));
        m.insert(0xf4, OpCode::new(0xf4, "nop", 2, AddressingMode::ZeroPageX));
        m.insert(0x0c, OpCode::new(0x0c, "nop", 3, AddressingMode::Absolute));
        m.insert(0x1c, OpCode::new(0x1c, "nop", 3, AddressingMode::AbsoluteX));
        m.insert(0x3c, OpCode::new(0x3c, "nop", 3, AddressingMode::AbsoluteX));
        m.insert(0x5c, OpCode::new(0x5c, "nop", 3, AddressingMode::AbsoluteX));
        m.insert(0x7c, OpCode::new(0x7c, "nop", 3, AddressingMode::AbsoluteX));
        m.insert(0xdc, OpCode::new(0xdc, "nop", 3, AddressingMode::AbsoluteX));
        m.insert(0xfc, OpCode::new(0xfc, "nop", 3, AddressingMode::AbsoluteX));
        m.insert(0x02, OpCode::new(0x02, "nop", 1, AddressingMode::Implied));
        m.insert(0x12, OpCode::new(0x12, "nop", 1, AddressingMode::Implied));
        m.insert(0x22, OpCode::new(0x22, "nop", 1, AddressingMode::Implied));
        m.insert(0x32, OpCode::new(0x32, "nop", 1, AddressingMode::Implied));
        m.insert(0x42, OpCode::new(0x42, "nop", 1, AddressingMode::Implied));
        m.insert(0x52, OpCode::new(0x52, "nop", 1, AddressingMode::Implied));
        m.insert(0x62, OpCode::new(0x62, "nop", 1, AddressingMode::Implied));
        m.insert(0x72, OpCode::new(0x72, "nop", 1, AddressingMode::Implied));
        m.insert(0x92, OpCode::new(0x92, "nop", 1, AddressingMode::Implied));
        m.insert(0xb2, OpCode::new(0xb2, "nop", 1, AddressingMode::Implied));
        m.insert(0xd2, OpCode::new(0xd2, "nop", 1, AddressingMode::Implied));
        m.insert(0xf2, OpCode::new(0xf2, "nop", 1, AddressingMode::Implied));
        m.insert(0x1a, OpCode::new(0x1a, "nop", 1, AddressingMode::Implied));
        m.insert(0x3a, OpCode::new(0x3a, "nop", 1, AddressingMode::Implied));
        m.insert(0x5a, OpCode::new(0x5a, "nop", 1, AddressingMode::Implied));
        m.insert(0x7a, OpCode::new(0x7a, "nop", 1, AddressingMode::Implied));
        m.insert(0xda, OpCode::new(0xda, "nop", 1, AddressingMode::Implied));
        m.insert(0xfa, OpCode::new(0xfa, "nop", 1, AddressingMode::Implied));

        m.insert(0x27, OpCode::new(0x27, "rla", 2, AddressingMode::ZeroPage));
        m.insert(0x37, OpCode::new(0x37, "rla", 2, AddressingMode::ZeroPageX));
        m.insert(0x2F, OpCode::new(0x2F, "rla", 3, AddressingMode::Absolute));
        m.insert(0x3F, OpCode::new(0x3F, "rla", 3, AddressingMode::AbsoluteX));
        m.insert(0x3b, OpCode::new(0x3b, "rla", 3, AddressingMode::AbsoluteY));
        m.insert(0x33, OpCode::new(0x33, "rla", 2, AddressingMode::IndirectIndexed));
        m.insert(0x23, OpCode::new(0x23, "rla", 2, AddressingMode::IndexedIndirect));

        m.insert(0x67, OpCode::new(0x67, "rra", 2, AddressingMode::ZeroPage));
        m.insert(0x77, OpCode::new(0x77, "rra", 2, AddressingMode::ZeroPageX));
        m.insert(0x6f, OpCode::new(0x6f, "rra", 3, AddressingMode::Absolute));
        m.insert(0x7f, OpCode::new(0x7f, "rra", 3, AddressingMode::AbsoluteX));
        m.insert(0x7b, OpCode::new(0x7b, "rra", 3, AddressingMode::AbsoluteY));
        m.insert(0x63, OpCode::new(0x63, "rra", 2, AddressingMode::IndexedIndirect));
        m.insert(0x73, OpCode::new(0x73, "rra", 2, AddressingMode::IndirectIndexed));

        m.insert(0x87, OpCode::new(0x87, "sax", 2, AddressingMode::ZeroPage));
        m.insert(0x97, OpCode::new(0x97, "sax", 2, AddressingMode::ZeroPageY));
        m.insert(0x8f, OpCode::new(0x8f, "sax", 3, AddressingMode::Absolute));
        m.insert(0x83, OpCode::new(0x83, "sax", 2, AddressingMode::IndexedIndirect));

        m.insert(0xeb, OpCode::new(0xeb, "sbc", 2, AddressingMode::Immediate));

        m.insert(0x9e, OpCode::new(0x9e, "shx", 3, AddressingMode::AbsoluteY));

        m.insert(0x9c, OpCode::new(0x9c, "shy", 3, AddressingMode::AbsoluteX));

        m.insert(0x07, OpCode::new(0x07, "slo", 2, AddressingMode::ZeroPage));
        m.insert(0x17, OpCode::new(0x17, "slo", 2, AddressingMode::ZeroPageX));
        m.insert(0x0F, OpCode::new(0x0F, "slo", 3, AddressingMode::Absolute));
        m.insert(0x1f, OpCode::new(0x1f, "slo", 3, AddressingMode::AbsoluteX));
        m.insert(0x1b, OpCode::new(0x1b, "slo", 3, AddressingMode::AbsoluteY));
        m.insert(0x03, OpCode::new(0x03, "slo", 2, AddressingMode::IndexedIndirect));
        m.insert(0x13, OpCode::new(0x13, "slo", 2, AddressingMode::IndirectIndexed));

        m.insert(0x47, OpCode::new(0x47, "sre", 2, AddressingMode::ZeroPage));
        m.insert(0x57, OpCode::new(0x57, "sre", 2, AddressingMode::ZeroPageX));
        m.insert(0x4F, OpCode::new(0x4F, "sre", 3, AddressingMode::Absolute));
        m.insert(0x5f, OpCode::new(0x5f, "sre", 3, AddressingMode::AbsoluteX));
        m.insert(0x5b, OpCode::new(0x5b, "sre", 3, AddressingMode::AbsoluteY));
        m.insert(0x43, OpCode::new(0x43, "sre", 2, AddressingMode::IndexedIndirect));
        m.insert(0x53, OpCode::new(0x53, "sre", 2, AddressingMode::IndirectIndexed));

        m.insert(0x9b, OpCode::new(0x9b, "tas", 3, AddressingMode::AbsoluteY));

        m.insert(0x8b, OpCode::new(0x8b, "xaa", 2, AddressingMode::Immediate));

        m
    };
}
