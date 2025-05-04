use aeiou::compiler::Compiler;
use aeiou::{lexer, parser};
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let tokens = match lexer::tokenize(&input) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("\x1b[31m{err}\x1b[0m");
            return;
        }
    };

    let parsed = parser::parse(&tokens);
    if let Err(err) = parsed {
        eprintln!("\x1b[31m{err}\x1b[0m");
        return;
    }

    let parsed = parsed.unwrap();
    let mut c = Compiler::new(&parsed);
    if let Err(err) = c.compile() {
        eprintln!("\x1b[31m{err}\x1b[0m");
    }
}
