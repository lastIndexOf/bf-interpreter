// use brainfuck_interpreter::repl::repl;

use brainfuck_interpreter::{lexer::Lexer, vm::vm::VirtualMachine};

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let filename = &args[1];

    let lexer = Lexer::from(std::fs::File::open(filename)?);
    let mut vm = VirtualMachine::new();

    vm.run(lexer);

    // repl();

    Ok(())
}
