use std::io::Write;

pub fn repl() {
    let mut stdout = std::io::stdout();
    let prelude = ">>> ".as_bytes();
    stdout.write(prelude).unwrap();
    stdout.flush().unwrap();

    for line in std::io::stdin().lines() {
        println!("{line:?}");
        stdout.write(prelude).unwrap();
        stdout.flush().unwrap();
    }
}
