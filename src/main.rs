use scanner::*;
use std::{
    env::args,
    io::{self, BufRead},
};
mod error;
use error::*;
mod scanner;
mod token;
mod token_type;
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: lox-ast [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[1]).expect("Failed to run file");
    } else {
        run_prompt();
    }
}

pub fn run_file(path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

pub fn run_prompt() -> io::Result<()> {
    let stdin = io::stdin();
    println!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(m) => m.report("".to_string()),
            }
        } else {
            break;
        }
    }
    Ok(())
}

pub fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
