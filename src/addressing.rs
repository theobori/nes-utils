#[derive(Debug, Clone, Copy)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    IndexedIndirect,
    Indirect,
    IndirectIndexed,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Implied
}

impl AddressingMode {
    pub fn fmt_arg(&self, arg: &str) -> String {
        match self {
            AddressingMode::Absolute => format!("${}", arg),
            AddressingMode::AbsoluteX => format!("${}, x", arg),
            AddressingMode::AbsoluteY => format!("${}, y", arg),
            AddressingMode::Immediate => format!("#${}", arg),
            AddressingMode::IndexedIndirect => format!("(${}, x)", arg),
            AddressingMode::Indirect => format!("${}", arg),
            AddressingMode::IndirectIndexed => format!("(${}), y", arg),
            AddressingMode::ZeroPage => format!("${}", arg),
            AddressingMode::ZeroPageX => format!("${}, x", arg),
            AddressingMode::ZeroPageY => format!("${}, y", arg),
            AddressingMode::Implied => "".to_string()
        }
    }
}