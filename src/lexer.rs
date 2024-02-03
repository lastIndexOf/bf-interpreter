use std::collections::HashMap;

use self::opcode::Opcode;

pub mod opcode;

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

impl From<&[u8]> for Lexer {
    fn from(value: &[u8]) -> Self {
        let ins = value
            .iter()
            .filter_map(|op| Opcode::try_from(*op).ok())
            .collect::<Vec<_>>();

        let mut jump_table = HashMap::new();
        let mut jump_stack = vec![];

        for (pos, op) in value.iter().enumerate() {
            match op {
                0x5b => jump_stack.push(pos),
                0x5d => {
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
