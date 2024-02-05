use super::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum Bytecode {
    // >
    SHR(usize),
    // <
    SHL(usize),
    // +
    ADD(usize),
    // -
    SUB(usize),
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

        for opcode in opcodes {
            match opcode {
                &Opcode::SHR => {
                    if let Some(mut last_code) = bytecode.last_mut() {
                        match &mut last_code {
                            Bytecode::SHR(val) => *val += 1,
                            _ => {
                                bytecode.push(Bytecode::SHR(1));
                            }
                        }
                    }
                }
                &Opcode::SHL => {
                    if let Some(mut last_code) = bytecode.last_mut() {
                        match &mut last_code {
                            Bytecode::SHL(val) => *val += 1,
                            _ => {
                                bytecode.push(Bytecode::SHL(1));
                            }
                        }
                    }
                }
                &Opcode::ADD => {
                    if let Some(mut last_code) = bytecode.last_mut() {
                        match &mut last_code {
                            Bytecode::ADD(val) => *val += 1,
                            _ => {
                                bytecode.push(Bytecode::ADD(1));
                            }
                        }
                    }
                }
                &Opcode::SUB => {
                    if let Some(mut last_code) = bytecode.last_mut() {
                        match &mut last_code {
                            Bytecode::SUB(val) => *val += 1,
                            _ => {
                                bytecode.push(Bytecode::SUB(1));
                            }
                        }
                    }
                }
                &Opcode::OUTPUT => {
                    bytecode.push(Bytecode::OUTPUT);
                }
                &Opcode::INPUT => {
                    bytecode.push(Bytecode::INPUT);
                }
                &Opcode::LR => {
                    bytecode.push(Bytecode::LR(0));
                }
                &Opcode::LB => {
                    bytecode.push(Bytecode::LB(0));
                }
            }
        }

        bytecode
    }
}
