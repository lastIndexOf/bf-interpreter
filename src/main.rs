// use brainfuck_interpreter::repl::repl;

use brainfuck_interpreter::{
    lexer::Lexer,
    vm::{
        // ir::IrVM,
        jit::JitVM,
        VirtualMachine,
        VM,
    },
};

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let filename = &args[1];

    let lexer = Lexer::from(std::fs::File::open(filename)?);
    let mut vm = VirtualMachine::new();

    vm.run_with_jit(lexer)?;
    // vm.run_with_ir(lexer);
    // vm.run(lexer);

    // repl();

    Ok(())
}
