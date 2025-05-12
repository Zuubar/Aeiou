use aeiou::compiler::Compiler;
use aeiou::{lexer, parser};
use std::{env, fs};

fn read_source() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("source file is required.");
    }
    let name = &args[1];
    if !name.ends_with(".aeiou") {
        return Err("source file should must have \".aeiou\" extension.");
    }

    match fs::read_to_string(name) {
        Ok(str) => Ok(str),
        Err(e) => Err("Could not read source file."),
    }
}

fn display_err(err: &str) {
    eprintln!("\x1b[31m{err}\x1b[0m");
}

fn main() {
    // Todo: Terminal emulators add backspace and it screws up input, when entering Georgian

    let source = read_source();
    if let Err(err) = source {
        display_err(err);
        return;
    }
    
    let tokens = match lexer::tokenize(&source.unwrap()) {
        Ok(result) => result,
        Err(err) => {
            display_err(err);
            return;
        }
    };

    let parsed = parser::parse(tokens);
    if let Err(err) = parsed {
        display_err(err);
        return;
    }

    let parsed = parsed.unwrap();
    let mut c = Compiler::new();
    if let Err(err) = c.compile(parsed) {
        display_err(err.to_string().as_str());
    }
}
