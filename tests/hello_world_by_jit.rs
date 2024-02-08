use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi};

use std::io::Write;
use std::{io, mem, slice};

#[test]
fn print_hello_world_by_jit() {
    let mut ops = dynasmrt::aarch64::Assembler::new().unwrap();
    let string = "print by aarch64 asm: Hello World\n";

    // ⬇️ x64
    // dynasm!(ops
    //     ; .arch x64
    //     ; ->hello:
    //     ; .bytes string.as_bytes()
    // );

    // let hello = ops.offset();
    // dynasm!(ops
    //     ; .arch x64
    //     ; lea rcx, [->hello]
    //     ; xor edx, edx
    //     ; mov dl, BYTE string.len() as _
    //     ; mov rax, QWORD print as _
    //     ; sub rsp, BYTE 0x28
    //     ; call rax
    //     ; add rsp, BYTE 0x28
    //     ; ret
    // );

    dynasm!(ops
        ; .arch aarch64
        ; ->hello:
        ; .bytes string.as_bytes()
        ; .align 4
        ; ->print:
        ; .qword print as _
    );

    let hello = ops.offset();
    dynasm!(ops
        ; .arch aarch64
        ; adr x0, ->hello
        ; movz x1, string.len() as u32
        ; ldr x9, ->print
        ; str x30, [sp, #-16]!
        ; blr x9
        ; ldr x30, [sp], #16
        ; ret
    );

    let buf = ops.finalize().unwrap();

    let hello_fn: extern "C" fn() -> bool = unsafe { mem::transmute(buf.ptr(hello)) };

    assert!(hello_fn());
}

pub extern "C" fn print(buffer: *const u8, length: u64) -> bool {
    io::stdout()
        .write_all(unsafe { slice::from_raw_parts(buffer, length as usize) })
        .is_ok()
}
