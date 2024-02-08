/// IR can run with bytecode
use std::io::{Read, Write};

use super::VirtualMachine;
use crate::lexer::{bytecode::Bytecode, Lexer};

pub trait IrVM {
    fn run_with_ir(&mut self, lexer: Lexer);
}

impl IrVM for VirtualMachine {
    fn run_with_ir(&mut self, lexer: Lexer) {
        let ins = &lexer.ins;
        let ins = Bytecode::from_opcode(&ins);

        while self.pc < ins.len() {
            let instruct = &ins[self.pc];

            match instruct {
                &Bytecode::SHR(val) => {
                    self.sp += val;
                    for _ in 0..val {
                        if self.sp >= self.stack.len() {
                            self.stack.push(0);
                        } else {
                            break;
                        }
                    }
                }
                &Bytecode::SHL(val) => {
                    for _ in 0..val {
                        if self.sp != 0 {
                            self.sp -= 1;
                        } else {
                            break;
                        }
                    }
                }
                &Bytecode::ADD(val) => {
                    self.stack[self.sp] = self.stack[self.sp].overflowing_add(val).0;
                }
                &Bytecode::SUB(val) => {
                    self.stack[self.sp] = self.stack[self.sp].overflowing_sub(val).0;
                }
                &Bytecode::OUTPUT => {
                    if !cfg!(feature = "no_output") {
                        std::io::stdout()
                            .write_all(&[self.stack[self.sp]])
                            .expect("Failed to write")
                    }
                }
                &Bytecode::INPUT => {
                    let mut input = [0; 1];
                    std::io::stdin()
                        .read_exact(&mut input)
                        .expect("Failed to read");
                    self.stack[self.sp] = input[0];
                }
                &Bytecode::LR(val) => {
                    if self.stack[self.sp] == 0 {
                        self.pc = val;
                    }
                }
                &Bytecode::LB(val) => {
                    if self.stack[self.sp] != 0 {
                        self.pc = val;
                    }
                }
            }

            self.pc += 1;
        }
    }
}
