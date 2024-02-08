use anyhow::anyhow;
/// JIT(just in time) can run with bytecode
use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};
use std::io::{Read, Write};

use super::VirtualMachine;
use crate::lexer::{bytecode::Bytecode, Lexer};

pub trait JitVM {
    fn run_with_jit(&mut self, lexer: Lexer) -> anyhow::Result<()>;
}

unsafe extern "C" fn output_char(word: *const u8) {
    std::io::stdout().write_all(&[unsafe { *word }]).unwrap()
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
            ; ->output_char:
            ; .qword output_char as _
        );

        let entry_point = ops.offset();

        dynasm!(ops
            ; .arch aarch64
            ; str x30, [sp, #-16]!
            ; stp x0, x1, [sp, #-16]!
            ; stp x2, x3, [sp, #-16]!
        );

        while self.pc < ins.len() {
            let instruct = &ins[self.pc];

            match instruct {
                &Bytecode::SHR(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; sub x0, x0, val as u32
                    )
                }
                &Bytecode::SHL(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; add x0, x0, val as u32
                    )
                }
                &Bytecode::ADD(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldrb w9, [x0]
                       ; add w9, w9, val as u32
                       ; strb w9, [x0]
                    )
                }
                &Bytecode::SUB(val) => {
                    dynasm!(ops
                       ; .arch aarch64
                       ; ldrb w9, [x0]
                       ; cmp w9, 0
                       ; b.eq >wrap
                       ; sub w9, w9, val as u32
                       ; strb w9, [x0]
                       ;wrap:
                    )
                }
                &Bytecode::OUTPUT => {
                    if !cfg!(feature = "no_output") {
                        dynasm!(ops
                            // ; .arch aarch64
                            // ; str x0, [sp, #-8]!
                            // ; ldrb w0, [x0]
                            // ; ldr x9, ->output_char
                            // ; blr x9
                            // ; ldr x0, [sp], #8
                            ; str x1, [sp, #24]
                            ; ldr x9, ->output_char
                            ; blr x9
                            ; mov x9, x0
                            ; ldp x0, x1, [sp, #16]
                            ; ldp x2, x3, [sp]
                        );
                    }
                }
                &Bytecode::INPUT => {
                    // TODO: implement input
                }
                &Bytecode::LR(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    jump_tables.push((l, r));
                    dynasm!(ops
                        ; .arch aarch64
                        ; ldrb w9, [x0]
                        ; cbz w9, => r
                        ;=>l
                    )
                }
                &Bytecode::LB(_) => match jump_tables.pop() {
                    Some((l, r)) => {
                        dynasm!(ops
                            ; .arch aarch64
                            ; ldrb w9, [x0]
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
            ; mov x0, 0
            ; add sp, sp, #32
            ; ldr x30, [sp], #16
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
