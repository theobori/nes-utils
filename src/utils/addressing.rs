#[derive(Debug, Clone, Copy, PartialEq)]
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
            Self::Absolute => format!("${}", arg),
            Self::AbsoluteX => format!("${}, x", arg),
            Self::AbsoluteY => format!("${}, y", arg),
            Self::Immediate => format!("#${}", arg),
            Self::IndexedIndirect => format!("(${}, x)", arg),
            Self::Indirect => format!("${}", arg),
            Self::IndirectIndexed => format!("(${}), y", arg),
            Self::ZeroPage => format!("${}", arg),
            Self::ZeroPageX => format!("${}, x", arg),
            Self::ZeroPageY => format!("${}, y", arg),
            Self::Implied => "".to_string()
        }
    }

    pub fn fmt_arg_with_reg(&self, arg: &str) -> String {
        let mut ret = self.fmt_arg(arg);

        ret = ret.replace("$", "");
        ret = ret.replace("#", "");

        ret
    }
}