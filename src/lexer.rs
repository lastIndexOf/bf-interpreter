use std::collections::HashMap;

pub enum Opcode {
    // >
    SHR = 0x3e,
    // <
    SHL = 0x3c,
    // +
    ADD = 0x2b,
    // -
    SUB = 0x2d,
    // .
    OUTPUT = 0x2e,
    // ,
    INPUT = 0x2c,
    // [
    LR = 0x5b,
    // ]
    LB = 0x5d,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x3e => Opcode::SHR,
            0x3c => Opcode::SHL,
            0x2b => Opcode::ADD,
            0x2d => Opcode::SUB,
            0x2e => Opcode::OUTPUT,
            0x2c => Opcode::INPUT,
            0x5b => Opcode::LR,
            0x5d => Opcode::LB,
            _ => unreachable!(),
        }
    }
}

pub struct Lexer {
    // instructions
    pub ins: Vec<Opcode>,
    pub jump_table: HashMap<usize, usize>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            ins: vec![],
            jump_table: HashMap::new(),
        }
    }
}
