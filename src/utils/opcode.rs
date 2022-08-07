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


pub fn get_nes_opcode(code: &u8) -> Option<OpCode> {
    let ret = match code {
        0x69 => ("adc", 2, AddressingMode::Immediate),
        0x65 => ("adc", 2, AddressingMode::ZeroPage),
        0x75 => ("adc", 2, AddressingMode::ZeroPageX),
        0x6d => ("adc", 3, AddressingMode::Absolute),
        0x7d => ("adc", 3, AddressingMode::AbsoluteX),
        0x79 => ("adc", 3, AddressingMode::AbsoluteY),
        0x61 => ("adc", 2, AddressingMode::IndexedIndirect),
        0x71 => ("adc", 2, AddressingMode::IndirectIndexed),

        0x29 => ("and", 2, AddressingMode::Immediate),
        0x25 => ("and", 2, AddressingMode::ZeroPage),
        0x35 => ("and", 2, AddressingMode::ZeroPageX),
        0x2d => ("and", 3, AddressingMode::Absolute),
        0x3d => ("and", 3, AddressingMode::AbsoluteX),
        0x39 => ("and", 3, AddressingMode::AbsoluteY),
        0x21 => ("and", 2, AddressingMode::IndexedIndirect),
        0x31 => ("and", 2, AddressingMode::IndirectIndexed),

        0x0a => ("asl", 1, AddressingMode::Implied),
        0x06 => ("asl", 2, AddressingMode::ZeroPage),
        0x16 => ("asl", 2, AddressingMode::ZeroPageX),
        0x0e => ("asl", 3, AddressingMode::Absolute),
        0x1e => ("asl", 3, AddressingMode::AbsoluteX),

        0x90 => ("bcc", 2, AddressingMode::Implied),

        0xb0 => ("bcs", 2, AddressingMode::Implied),

        0xf0 => ("beq", 2, AddressingMode::Implied),

        0x24 => ("bit", 2, AddressingMode::ZeroPage),
        0x2c => ("bit", 3, AddressingMode::Absolute),

        0x30 => ("bmi", 2, AddressingMode::Absolute),

        0xd0 => ("bne", 2, AddressingMode::Absolute),

        0x10 => ("bpl", 2, AddressingMode::Absolute),

        0x00 => ("brk", 1, AddressingMode::Implied),

        0x50 => ("bvc", 2, AddressingMode::Absolute),

        0x70 => ("bvs", 2, AddressingMode::Absolute),

        0x18 => ("clc", 1, AddressingMode::Implied),

        0xd8 => ("cld", 1, AddressingMode::Implied),

        0x58 => ("cli", 1, AddressingMode::Implied),

        0xb8 => ("clv", 1, AddressingMode::Implied),

        0xc9 => ("cmp", 2, AddressingMode::Immediate),
        0xc5 => ("cmp", 2, AddressingMode::ZeroPage),
        0xd5 => ("cmp", 2, AddressingMode::ZeroPageX),
        0xcd => ("cmp", 3, AddressingMode::Absolute),
        0xdd => ("cmp", 3, AddressingMode::AbsoluteX),
        0xd9 => ("cmp", 3, AddressingMode::AbsoluteY),
        0xc1 => ("cmp", 2, AddressingMode::IndexedIndirect),
        0xd1 => ("cmp", 2, AddressingMode::IndirectIndexed),

        0xe0 => ("cpx", 2, AddressingMode::Immediate),
        0xe4 => ("cpx", 2, AddressingMode::ZeroPage),
        0xec => ("cpx", 3, AddressingMode::Absolute),

        0xc0 => ("cpy", 2, AddressingMode::Immediate),
        0xc4 => ("cpy", 2, AddressingMode::ZeroPage),
        0xcc => ("cpy", 3, AddressingMode::Absolute),

        0xc6 => ("dec", 2, AddressingMode::ZeroPage),
        0xd6 => ("dec", 2, AddressingMode::ZeroPageX),
        0xce => ("dec", 3, AddressingMode::Absolute),
        0xde => ("dec", 3, AddressingMode::AbsoluteX),

        0xca => ("dex", 1, AddressingMode::Implied),

        0x88 => ("dey", 1, AddressingMode::Implied),

        0x49 => ("eor", 2, AddressingMode::Immediate),
        0x45 => ("eor", 2, AddressingMode::ZeroPage),
        0x55 => ("eor", 2, AddressingMode::ZeroPageX),
        0x4d => ("eor", 3, AddressingMode::Absolute),
        0x5d => ("eor", 3, AddressingMode::AbsoluteX),
        0x59 => ("eor", 3, AddressingMode::AbsoluteY),
        0x41 => ("eor", 2, AddressingMode::IndexedIndirect),
        0x51 => ("eor", 2, AddressingMode::IndirectIndexed),

        0xe6 => ("inc", 2, AddressingMode::ZeroPage),
        0xf6 => ("inc", 2, AddressingMode::ZeroPageX),
        0xee => ("inc", 3, AddressingMode::Absolute),
        0xfe => ("inc", 3, AddressingMode::AbsoluteX),

        0xe8 => ("inx", 1, AddressingMode::Implied),

        0xc8 => ("iny", 1, AddressingMode::Implied),

        0x4c => ("jmp", 3, AddressingMode::Absolute), 
        0x6c => ("jmp", 3, AddressingMode::Indirect),

        0x20 => ("jsr", 3, AddressingMode::Absolute),

        0xa9 => ("lda", 2, AddressingMode::Immediate),
        0xa5 => ("lda", 2, AddressingMode::ZeroPage),
        0xb5 => ("lda", 2, AddressingMode::ZeroPageX),
        0xad => ("lda", 3, AddressingMode::Absolute),
        0xbd => ("lda", 3, AddressingMode::AbsoluteX),
        0xb9 => ("lda", 3, AddressingMode::AbsoluteY),
        0xa1 => ("lda", 2, AddressingMode::IndexedIndirect),
        0xb1 => ("lda", 2, AddressingMode::IndirectIndexed),
    
        0xa2 => ("ldx", 2, AddressingMode::Immediate),
        0xa6 => ("ldx", 2, AddressingMode::ZeroPage),
        0xb6 => ("ldx", 2, AddressingMode::ZeroPageY),
        0xae => ("ldx", 3, AddressingMode::Absolute),
        0xbe => ("ldx", 3, AddressingMode::AbsoluteY),

        0xa0 => ("ldy", 2, AddressingMode::Immediate),
        0xa4 => ("ldy", 2, AddressingMode::ZeroPage),
        0xb4 => ("ldy", 2, AddressingMode::ZeroPageX),
        0xac => ("ldy", 3, AddressingMode::Absolute),
        0xbc => ("ldy", 3, AddressingMode::AbsoluteX),

        0x4a => ("lsr", 1, AddressingMode::Implied),
        0x46 => ("lsr", 2, AddressingMode::ZeroPage),
        0x56 => ("lsr", 2, AddressingMode::ZeroPageX),
        0x4e => ("lsr", 3, AddressingMode::Absolute),
        0x5e => ("lsr", 3, AddressingMode::AbsoluteX),
        
        0xea => ("hex", 3, AddressingMode::AbsoluteX),

        0x09 => ("ora", 2, AddressingMode::Immediate),
        0x05 => ("ora", 2, AddressingMode::ZeroPage),
        0x15 => ("ora", 2, AddressingMode::ZeroPageX),
        0x0d => ("ora", 3, AddressingMode::Absolute),
        0x1d => ("ora", 3, AddressingMode::AbsoluteX),
        0x19 => ("ora", 3, AddressingMode::AbsoluteY),
        0x01 => ("ora", 2, AddressingMode::IndexedIndirect),
        0x11 => ("ora", 2, AddressingMode::IndirectIndexed),

        0x48 => ("pha", 1, AddressingMode::Implied),

        0x08 => ("php", 1, AddressingMode::Implied),

        0x68 => ("pla", 1, AddressingMode::Implied),

        0x28 => ("plp", 1, AddressingMode::Implied),

        0x2a => ("rol", 1, AddressingMode::Implied),
        0x26 => ("rol", 2, AddressingMode::ZeroPage),
        0x36 => ("rol", 2, AddressingMode::ZeroPageX),
        0x2e => ("rol", 3, AddressingMode::Absolute),
        0x3e => ("rol", 3, AddressingMode::AbsoluteX),

        0x6a => ("ror", 1, AddressingMode::Implied),
        0x66 => ("ror", 2, AddressingMode::ZeroPage),
        0x76 => ("ror", 2, AddressingMode::ZeroPageX),
        0x6e => ("ror", 3, AddressingMode::Absolute),
        0x7e => ("ror", 3, AddressingMode::AbsoluteX),

        0x40 => ("rti", 1, AddressingMode::Implied),

        0x60 => ("rts", 1, AddressingMode::Implied),

        0xe9 => ("sbc", 2, AddressingMode::Immediate),
        0xe5 => ("sbc", 2, AddressingMode::ZeroPage),
        0xf5 => ("sbc", 2, AddressingMode::ZeroPageX),
        0xed => ("sbc", 3, AddressingMode::Absolute),
        0xfd => ("sbc", 3, AddressingMode::AbsoluteX),
        0xf9 => ("sbc", 3, AddressingMode::AbsoluteY),
        0xe1 => ("sbc", 2, AddressingMode::IndexedIndirect),
        0xf1 => ("sbc", 2, AddressingMode::IndirectIndexed),

        0x38 => ("sec", 1, AddressingMode::Implied),

        0xf8 => ("sed", 1, AddressingMode::Implied),

        0x78 => ("sei", 1, AddressingMode::Implied),

        0x85 => ("sta", 2, AddressingMode::ZeroPage),
        0x95 => ("sta", 2, AddressingMode::ZeroPageX),
        0x8d => ("sta", 3, AddressingMode::Absolute),
        0x9d => ("sta", 3, AddressingMode::AbsoluteX),
        0x99 => ("sta", 3, AddressingMode::AbsoluteY),
        0x81 => ("sta", 2, AddressingMode::IndexedIndirect),
        0x91 => ("sta", 2, AddressingMode::IndirectIndexed),

        0x86 => ("stx", 2, AddressingMode::ZeroPage),
        0x96 => ("stx", 2, AddressingMode::ZeroPageY),
        0x8e => ("stx", 3, AddressingMode::Absolute),

        0x84 => ("sty", 2, AddressingMode::ZeroPage),
        0x94 => ("sty", 2, AddressingMode::ZeroPageX),
        0x8c => ("sty", 3, AddressingMode::Absolute),

        0xaa => ("tax", 1, AddressingMode::Implied),

        0xa8 => ("tay", 1, AddressingMode::Implied),

        0xba => ("tsx", 1, AddressingMode::Implied),

        0x8a => ("txa", 1, AddressingMode::Implied),

        0x9a => ("txs", 1, AddressingMode::Implied),

        0x98 => ("tya", 1, AddressingMode::Implied),


        // Unofficials


        0x93 => ("ahx", 2, AddressingMode::IndirectIndexed),
        0x9f => ("ahx", 3, AddressingMode::AbsoluteY),

        0x4b => ("alr", 2, AddressingMode::Immediate),

        0x0b => ("anc", 2, AddressingMode::Immediate),
        0x2b => ("anc", 2, AddressingMode::Immediate),

        0x6b => ("arr", 2, AddressingMode::Immediate),

        0xcb => ("axs", 2, AddressingMode::Immediate),

        0xc7 => ("dcp", 2, AddressingMode::ZeroPage),
        0xd7 => ("dcp", 2, AddressingMode::ZeroPageX),
        0xcf => ("dcp", 3, AddressingMode::Absolute),
        0xdf => ("dcp", 3, AddressingMode::AbsoluteX),
        0xdb => ("dcp", 3, AddressingMode::AbsoluteY),
        0xd3 => ("dcp", 2, AddressingMode::IndirectIndexed),
        0xc3 => ("dcp", 2, AddressingMode::IndexedIndirect),

        0xe7 => ("isb", 2, AddressingMode::ZeroPage),
        0xf7 => ("isb", 2, AddressingMode::ZeroPageX),
        0xef => ("isb", 3, AddressingMode::Absolute),
        0xff => ("isb", 3, AddressingMode::AbsoluteX),
        0xfb => ("isb", 3, AddressingMode::AbsoluteY),
        0xe3 => ("isb", 2, AddressingMode::IndexedIndirect),
        0xf3 => ("isb", 2, AddressingMode::IndirectIndexed),

        0xbb => ("las", 3, AddressingMode::AbsoluteY),

        0xa7 => ("lax", 2, AddressingMode::ZeroPage),
        0xb7 => ("lax", 2, AddressingMode::ZeroPageY),
        0xaf => ("lax", 3, AddressingMode::Absolute),
        0xbf => ("lax", 3, AddressingMode::AbsoluteY),
        0xa3 => ("lax", 2, AddressingMode::IndexedIndirect),
        0xb3 => ("lax", 2, AddressingMode::IndirectIndexed),
        0xab => ("lxa", 2, AddressingMode::Immediate),

        0x80 => ("hex", 2, AddressingMode::Immediate),
        0x82 => ("hex", 2, AddressingMode::Immediate),
        0x89 => ("hex", 2, AddressingMode::Immediate),
        0xc2 => ("hex", 2, AddressingMode::Immediate),
        0xe2 => ("hex", 2, AddressingMode::Immediate),
        0x04 => ("hex", 2, AddressingMode::ZeroPage),
        0x44 => ("hex", 2, AddressingMode::ZeroPage),
        0x64 => ("hex", 2, AddressingMode::ZeroPage),
        0x14 => ("hex", 2, AddressingMode::ZeroPageX),
        0x34 => ("hex", 2, AddressingMode::ZeroPageX),
        0x54 => ("hex", 2, AddressingMode::ZeroPageX),
        0x74 => ("hex", 2, AddressingMode::ZeroPageX),
        0xd4 => ("hex", 2, AddressingMode::ZeroPageX),
        0xf4 => ("hex", 2, AddressingMode::ZeroPageX),
        0x0c => ("hex", 3, AddressingMode::Absolute),
        0x1c => ("hex", 3, AddressingMode::AbsoluteX),
        0x3c => ("hex", 3, AddressingMode::AbsoluteX),
        0x5c => ("hex", 3, AddressingMode::AbsoluteX),
        0x7c => ("hex", 3, AddressingMode::AbsoluteX),
        0xdc => ("hex", 3, AddressingMode::AbsoluteX),
        0xfc => ("hex", 3, AddressingMode::AbsoluteX),
        0x02 => ("nop", 1, AddressingMode::Implied),
        0x12 => ("nop", 1, AddressingMode::Implied),
        0x22 => ("nop", 1, AddressingMode::Implied),
        0x32 => ("nop", 1, AddressingMode::Implied),
        0x42 => ("nop", 1, AddressingMode::Implied),
        0x52 => ("nop", 1, AddressingMode::Implied),
        0x62 => ("nop", 1, AddressingMode::Implied),
        0x72 => ("nop", 1, AddressingMode::Implied),
        0x92 => ("nop", 1, AddressingMode::Implied),
        0xb2 => ("nop", 1, AddressingMode::Implied),
        0xd2 => ("nop", 1, AddressingMode::Implied),
        0xf2 => ("nop", 1, AddressingMode::Implied),
        0x1a => ("nop", 1, AddressingMode::Implied),
        0x3a => ("nop", 1, AddressingMode::Implied),
        0x5a => ("nop", 1, AddressingMode::Implied),
        0x7a => ("nop", 1, AddressingMode::Implied),
        0xda => ("nop", 1, AddressingMode::Implied),
        0xfa => ("nop", 1, AddressingMode::Implied),

        0x27 => ("rla", 2, AddressingMode::ZeroPage),
        0x37 => ("rla", 2, AddressingMode::ZeroPageX),
        0x2f => ("rla", 3, AddressingMode::Absolute),
        0x3f => ("rla", 3, AddressingMode::AbsoluteX),
        0x3b => ("rla", 3, AddressingMode::AbsoluteY),
        0x33 => ("rla", 2, AddressingMode::IndirectIndexed),
        0x23 => ("rla", 2, AddressingMode::IndexedIndirect),

        0x67 => ("rra", 2, AddressingMode::ZeroPage),
        0x77 => ("rra", 2, AddressingMode::ZeroPageX),
        0x6f => ("rra", 3, AddressingMode::Absolute),
        0x7f => ("rra", 3, AddressingMode::AbsoluteX),
        0x7b => ("rra", 3, AddressingMode::AbsoluteY),
        0x63 => ("rra", 2, AddressingMode::IndexedIndirect),
        0x73 => ("rra", 2, AddressingMode::IndirectIndexed),

        0x87 => ("sax", 2, AddressingMode::ZeroPage),
        0x97 => ("sax", 2, AddressingMode::ZeroPageY),
        0x8f => ("sax", 3, AddressingMode::Absolute),
        0x83 => ("sax", 2, AddressingMode::IndexedIndirect),

        0xeb => ("sbc", 2, AddressingMode::Immediate),

        0x9e => ("shx", 3, AddressingMode::AbsoluteY),

        0x9c => ("shy", 3, AddressingMode::AbsoluteX),

        0x07 => ("slo", 2, AddressingMode::ZeroPage),
        0x17 => ("slo", 2, AddressingMode::ZeroPageX),
        0x0f => ("slo", 3, AddressingMode::Absolute),
        0x1f => ("slo", 3, AddressingMode::AbsoluteX),
        0x1b => ("slo", 3, AddressingMode::AbsoluteY),
        0x03 => ("slo", 2, AddressingMode::IndexedIndirect),
        0x13 => ("slo", 2, AddressingMode::IndirectIndexed),

        0x47 => ("sre", 2, AddressingMode::ZeroPage),
        0x57 => ("sre", 2, AddressingMode::ZeroPageX),
        0x4f => ("sre", 3, AddressingMode::Absolute),
        0x5f => ("sre", 3, AddressingMode::AbsoluteX),
        0x5b => ("sre", 3, AddressingMode::AbsoluteY),
        0x43 => ("sre", 2, AddressingMode::IndexedIndirect),
        0x53 => ("sre", 2, AddressingMode::IndirectIndexed),

        0x9b => ("tas", 3, AddressingMode::AbsoluteY),

        0x8b => ("xaa", 2, AddressingMode::Immediate)
    };

    let (mnemonic, len, mode) = ret;
    let opcode = OpCode::new(*code, mnemonic, len, mode);

    Some(opcode)
}
