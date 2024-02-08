use anyhow::anyhow;
/// JIT(just in time) can run with bytecode
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use std::io::{Read, Write};

use super::VirtualMachine;
use crate::lexer::{bytecode::Bytecode, Lexer};

pub trait JitVM {
    fn run_with_jit(&mut self, lexer: Lexer) -> anyhow::Result<()>;
}

unsafe extern "C" fn output_char(char: u8) {
    std::io::stdout().write_all(&[char]).unwrap()
}

impl JitVM for VirtualMachine {
    fn run_with_jit(&mut self, lexer: Lexer) -> anyhow::Result<()> {
        let ins = &lexer.ins;
        let ins = Bytecode::from_opcode(&ins);
        let mut jump_tables = vec![];

        let mut ops = dynasmrt::aarch64::Assembler::new()?;
        let entry_point = ops.offset();

        // ARM64(aarch64) calling convention
        // https://en.wikipedia.org/wiki/Calling_convention#ARM_(A64)

        dynasm!(ops
            ; .arch aarch64
            // move x0 (caller first param) to x20 (general purpose register)
            ; mov x20, x0
            ; ->output_char:
            ; .qword output_char as _
        );

        while self.pc < ins.len() {
            let instruct = &ins[self.pc];

            match instruct {
                &Bytecode::SHR(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldr w21, val as u32
                       ; sub x20, x20, w21
                    )
                }
                &Bytecode::SHL(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldr w21, val as u32
                       ; add x20, x20, w21
                    )
                }
                &Bytecode::ADD(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldr w21, val as i8
                       ; ldr w22, [x20]
                       ; add w22, w22, w21
                       ; str w22, [x20]
                    )
                }
                &Bytecode::SUB(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldr w21, val as i8
                       ; ldr w22, [x20]
                       ; sub w22, w22, w21
                       ; str w22, [x20]
                    )
                }

                &Bytecode::OUTPUT => if !cfg!(feature = "no_output") {},
                &Bytecode::INPUT => {}
                &Bytecode::LR(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    jump_tables.push((l, r));
                    dynasm!(ops
                        ; .arch aarch64
                        ; ldrb w21, [x20]
                        // x21 == 0
                        ; cbz w21, => r
                        // x21 != 0
                        ; => l
                    )
                }
                &Bytecode::LB(_) => {
                    match jump_tables.pop() {
                        Some((l, r)) => {
                            dynasm!(ops
                                ; .arch aarch64
                                ; ldrb w21, [x20]
                                // x21 != 0
                                ; cbnz w21, => l
                                // x21 == 0
                                ; => r
                            )
                        }
                        None => return Err(anyhow!("Unmatched ]")),
                    }
                }
            }

            self.pc += 1;
        }

        dynasm!(ops
            ; .arch aarch64
            ; ret
        );

        let exec_buf = ops.finalize().expect("Failed to finalize");
        // vm stack to store buffer
        let mut stack = vec![0; 65536].into_boxed_slice();
        let stack_ptr = stack.as_mut_ptr();
        let runner: extern "C" fn(*mut u8) =
            unsafe { std::mem::transmute(exec_buf.ptr(entry_point)) };

        runner(stack_ptr);

        Ok(())
    }
}
