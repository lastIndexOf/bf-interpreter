use anyhow::anyhow;

#[derive(Debug, PartialEq)]
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

impl TryFrom<u8> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x3e => Ok(Opcode::SHR),
            0x3c => Ok(Opcode::SHL),
            0x2b => Ok(Opcode::ADD),
            0x2d => Ok(Opcode::SUB),
            0x2e => Ok(Opcode::OUTPUT),
            0x2c => Ok(Opcode::INPUT),
            0x5b => Ok(Opcode::LR),
            0x5d => Ok(Opcode::LB),
            _ => Err(anyhow!("Invalid opcode")),
        }
    }
}
