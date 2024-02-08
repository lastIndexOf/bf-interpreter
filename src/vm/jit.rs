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

        // ARM64(aarch64) calling convention
        // https://en.wikipedia.org/wiki/Calling_convention#ARM_(A64)

        dynasm!(ops
            ; .arch aarch64
            // move x0 (caller first param) to x20 (general purpose register)
            ; ->output_char:
            ; .qword output_char as _
        );

        let entry_point = ops.offset();

        dynasm!(ops
            ; .arch aarch64
            ; mov x23, x20
            ; mov x24, x21
            ; mov x25, x22
            ; mov x20, x0
        );

        while self.pc < ins.len() {
            let instruct = &ins[self.pc];

            match instruct {
                &Bytecode::SHR(val) => {
                    // let lower_val = (val & 0xffff_ffff) as u32;
                    // let upper_val = ((val >> 32) & 0xffff_ffff) as u32;

                    // dynasm!(ops
                    //     ; .arch aarch64
                    //     ; mov x21, lower_val as u64
                    //     ; movk x21, upper_val, lsl 32
                    //     ; sub x20, x20, x21
                    // )

                    dynasm!(ops
                       ; .arch aarch64
                       ; mov x21, val as u64
                       ; sub x20, x20, x21
                    )
                }
                &Bytecode::SHL(val) => {
                    // let lower_val = (val & 0xffff_ffff) as u32;
                    // let upper_val = ((val >> 32) & 0xffff_ffff) as u32;

                    // dynasm!(ops
                    //     ; .arch aarch64
                    //     ; mov x21, lower_val as u64
                    //     ; movk x21, upper_val, lsl 32
                    //     ; add x20, x20, x21
                    // )

                    dynasm!(ops
                       ; .arch aarch64
                       ; mov x21, val as u64
                       ; add x20, x20, x21
                    )
                }
                &Bytecode::ADD(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldr x21, val as i8
                       ; ldr x22, [x20]
                       ; add x22, x22, x21
                       ; str x22, [x20]
                    )
                }
                &Bytecode::SUB(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldr x21, val as i8
                       ; ldr x22, [x20]
                       ; sub x22, x22, x21
                       ; str x22, [x20]
                    )
                }

                &Bytecode::OUTPUT => {
                    if !cfg!(feature = "no_output") {
                        dynasm!(ops
                            ; .arch aarch64
                            ; mov x0, 0 
                            ; ldrb w0, [x20]
                            ; ldr x9, ->output_char
                            ; str x30, [sp, #-16]!
                            ; blr x9
                            ; ldr x30, [sp], #16
                            ; ret
                        );
                    }
                }
                &Bytecode::INPUT => {}
                &Bytecode::LR(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    jump_tables.push((l, r));
                    dynasm!(ops
                        ; .arch aarch64
                        ; ldrb w9, [x20]
                        ; cbz w9, => r
                        ;=>l
                    )
                }
                &Bytecode::LB(_) => match jump_tables.pop() {
                    Some((l, r)) => {
                        dynasm!(ops
                            ; .arch aarch64
                            ; ldrb w9, [x20]
                            ; cbnz w9, => l
                            ;=>r
                        )
                    }
                    None => return Err(anyhow!("Unmatched ]")),
                },
            }

            self.pc += 1;
        }

        dynasm!(ops
            ; .arch aarch64
            ; mov x20, x23
            ; mov x21, x24
            ; mov x22, x25
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
