use super::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum Bytecode {
    // >
    SHR(usize),
    // <
    SHL(usize),
    // +
    ADD(u8),
    // -
    SUB(u8),
    // .
    OUTPUT,
    // ,
    INPUT,
    // [
    LR(usize),
    // ]
    LB(usize),
}

impl Bytecode {
    pub fn from_opcode(opcodes: &[Opcode]) -> Vec<Bytecode> {
        let mut bytecode = vec![];

        let mut jump_stack = vec![];

        for opcode in opcodes {
            match opcode {
                &Opcode::SHR => match bytecode.last_mut() {
                    Some(Bytecode::SHR(val)) => *val += 1,
                    _ => {
                        bytecode.push(Bytecode::SHR(1));
                    }
                },
                &Opcode::SHL => match bytecode.last_mut() {
                    Some(Bytecode::SHL(val)) => *val += 1,
                    _ => {
                        bytecode.push(Bytecode::SHL(1));
                    }
                },
                &Opcode::ADD => match bytecode.last_mut() {
                    Some(Bytecode::ADD(val)) => *val += 1,
                    _ => {
                        bytecode.push(Bytecode::ADD(1));
                    }
                },
                &Opcode::SUB => match bytecode.last_mut() {
                    Some(Bytecode::SUB(val)) => *val += 1,
                    _ => {
                        bytecode.push(Bytecode::SUB(1));
                    }
                },
                &Opcode::OUTPUT => {
                    bytecode.push(Bytecode::OUTPUT);
                }
                &Opcode::INPUT => {
                    bytecode.push(Bytecode::INPUT);
                }
                &Opcode::LR => {
                    bytecode.push(Bytecode::LR(0));
                    jump_stack.push(bytecode.len() - 1);
                }
                &Opcode::LB => {
                    let start = jump_stack.pop().expect("Unmatched ]");
                    bytecode.push(Bytecode::LB(start));
                    let len = bytecode.len() - 1;
                    match bytecode[start] {
                        Bytecode::LR(ref mut val) => *val = len,
                        _ => unreachable!(),
                    }
                }
            }
        }

        bytecode
    }
}
