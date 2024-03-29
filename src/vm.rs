use std::io::{Read, Write};

use crate::lexer::{opcode::Opcode, Lexer};

#[cfg(all(feature = "ir", feature = "jit"))]
compile_error!("Cannot enable both features `ir` and `jit` at the same time");

#[cfg(feature = "ir")]
pub mod ir;

#[cfg(feature = "jit")]
pub mod jit;

pub trait VM {
    fn run(&mut self, lexer: Lexer);
}

pub struct VirtualMachine {
    // program counter
    pc: usize,
    // stack pointer
    sp: usize,
    stack: Vec<u8>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        VirtualMachine {
            pc: 0,
            sp: 0,
            stack: vec![0; 1],
        }
    }
}

impl VM for VirtualMachine {
    // TODO: add out pipeline settings
    fn run(&mut self, lexer: Lexer) {
        let ins = &lexer.ins;

        while self.pc < ins.len() {
            let instruct = &ins[self.pc];

            match instruct {
                &Opcode::SHR => {
                    self.sp += 1;
                    if self.sp == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                &Opcode::SHL => {
                    if self.sp != 0 {
                        self.sp -= 1;
                    }
                }
                &Opcode::ADD => {
                    self.stack[self.sp] = self.stack[self.sp].overflowing_add(1).0;
                }
                &Opcode::SUB => {
                    self.stack[self.sp] = self.stack[self.sp].overflowing_sub(1).0;
                }
                &Opcode::OUTPUT => {
                    if !cfg!(feature = "no_output") {
                        std::io::stdout()
                            .write_all(&[self.stack[self.sp]])
                            .expect("Failed to write")
                    }
                }
                &Opcode::INPUT => {
                    let mut input = [0; 1];
                    std::io::stdin()
                        .read_exact(&mut input)
                        .expect("Failed to read");
                    self.stack[self.sp] = input[0];
                }
                &Opcode::LR => {
                    if self.stack[self.sp] == 0 {
                        self.pc = lexer.jump_table[&self.pc];
                    }
                }
                &Opcode::LB => {
                    if self.stack[self.sp] != 0 {
                        self.pc = lexer.jump_table[&self.pc];
                    }
                }
            }

            self.pc += 1;
        }
    }
}
