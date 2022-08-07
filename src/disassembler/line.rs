use crate::utils::{
    opcode::OpCode,
    util::{
        u16_from_mem,
        unwrap_str, vec_bytes_to_string
    }, registers::get_mapped_register
};

use std::fmt;
use std::cmp::Ordering;

use super::disassembler::EquConst;

pub struct Line {
    pub bytes: Vec<u8>,
    pub opcode: OpCode,
    pub label: Option<String>,
    pub fmt_arg: String,
    pub comment: Option<String>
}

impl Line {
    pub fn len(&self) -> usize {
        let mut ret: usize = 0;

        if let Some(comment) = &self.comment {
            ret += comment.len();
        }
        if let Some(label) = &self.label {
            ret += label.len();
        }

        ret += self.opcode.mnemonic.len() + self.fmt_arg.len();

        ret
    }

    fn fmt_special(&mut self) -> bool {
        let arg_bytes = self.bytes[1..].to_vec();

        match self.opcode.mnemonic {
            "hex" => {
                self.fmt_arg = vec_bytes_to_string(&arg_bytes);
            },
            _ => return false
        };

        true
    }

    pub fn fmt(&mut self) -> Option<EquConst> {
        if self.fmt_special() {
            return None
        }

        self.fmt_arg()
    }

    fn fmt_arg(&mut self) -> Option<EquConst> {
        let mut arg_bytes = self.bytes[1..].to_vec();

        // Because of the endianess (little)
        arg_bytes.reverse();


        // Normal format
        let mut arg_str = OpCode::arg_to_string(&arg_bytes);
        let mut ret = None;

        arg_str = self.opcode.mode.fmt_arg(&arg_str);

        arg_str = match self.arg_to_le_u16() {
            Some(value) => {
                match get_mapped_register(value) {
                    Some(name) => {
                        ret = Some((value, name.clone()));
                        self.opcode.mode.fmt_arg_with_reg(&name)
                    },
                    None => arg_str
                }
            },
            None => arg_str
        };

        self.fmt_arg = arg_str;
    
        ret
    }

    fn arg_to_le_u16(&mut self) -> Option<u16> {
        let arg_bytes = self.bytes[1..].to_vec();

        if arg_bytes.len() < 2 {
            return None;
        }

        Some(u16_from_mem(arg_bytes[0], arg_bytes[1]))
    }
}

impl Eq for Line { }

impl Ord for Line {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp: i32 = self.len() as i32 - other.len() as i32;
        let ret: Ordering;

        if cmp < 0 {
            ret = Ordering::Less;
        } else if cmp > 0 {
            ret = Ordering::Greater
        } else {
            ret = Ordering::Equal
        }

        ret
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = unwrap_str(&self.label, "", ":\n");
        let comment = unwrap_str(&self.comment, "", "");

        write!(f, "{}{} {}{}\n",
            label,
            self.opcode.mnemonic,
            self.fmt_arg,
            comment
        )
    }
}
