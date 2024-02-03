// use brainfuck_interpreter::repl::repl;

use brainfuck_interpreter::lexer::Lexer;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let filename = &args[1];

    let code = std::fs::read(filename)?;
    let lexer = Lexer::from(&code[..]);

    println!("{lexer:?}");

    // repl();

    Ok(())
}
