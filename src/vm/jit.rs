/// JIT(just in time) can run with bytecode
use dynasmrt::{dynasm, DynasmApi};
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

        let mut ops = dynasmrt::aarch64::Assembler::new()?;
        let entry_point = ops.offset();

        // ARM64(aarch64) calling convention
        // https://en.wikipedia.org/wiki/Calling_convention#ARM_(A64)

        dynasm!(ops
            ; .arch aarch64
            // ; mov rcx, rdi
        );

        while self.pc < ins.len() {
            let instruct = &ins[self.pc];

            match instruct {
                &Bytecode::SHR(val) => {}
                &Bytecode::SHL(val) => {}
                &Bytecode::ADD(val) => {}
                &Bytecode::SUB(val) => {}
                &Bytecode::OUTPUT => if !cfg!(feature = "no_output") {},
                &Bytecode::INPUT => {}
                &Bytecode::LR(val) => {}
                &Bytecode::LB(val) => {}
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
