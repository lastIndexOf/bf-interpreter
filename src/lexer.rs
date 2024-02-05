use std::{
    collections::HashMap,
    io::{BufReader, Read},
};

use self::opcode::Opcode;

pub mod opcode;

#[cfg(feature = "ir")]
pub mod bytecode;

// TODO: add multiple input support
#[derive(Debug)]
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

impl<T: Read> From<T> for Lexer {
    fn from(value: T) -> Self {
        let mut reader = BufReader::new(value);

        let mut buf = vec![];
        reader
            .read_to_end(&mut buf)
            .expect("Failed to read source code");

        let ins = buf
            .iter()
            .filter_map(|op| Opcode::try_from(*op).ok())
            .collect::<Vec<_>>();

        let mut jump_table = HashMap::new();
        let mut jump_stack = vec![];

        for (pos, op) in ins.iter().enumerate() {
            match op {
                &Opcode::LR => jump_stack.push(pos),
                &Opcode::LB => {
                    let start = jump_stack.pop().expect("Unmatched ]");
                    jump_table.insert(start, pos);
                    jump_table.insert(pos, start);
                }
                _ => {}
            }
        }

        Lexer { ins, jump_table }
    }
}
