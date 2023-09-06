pub mod lexer;
pub mod parser;
pub mod vm;

use crate::lexer::lexer::*;

use crate::vm::vm::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: smpl-vm <file>");
        return;
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let code: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line."))
        .collect();

    match Program::load(lexify(&code)) {
        Ok(mut program) => {
            match program.run() {
                Err((msg, line, character)) => {
                    println!("Encountered a runtime error on line {}", line + 1);
                    println!("{}", code[line]);
                    println!(
                        "{}^{} {}",
                        "-".repeat(character),
                        "-".repeat(code[line].len() - character - 1),
                        msg
                    );
                }
                _ => {} // who cares
            }
        }
        Err((msg, line, character)) => {
            println!(
                "Encountered an error while parsing program on line {}.",
                line + 1
            );
            println!("{}", code[line]);
            println!(
                "{}^{} {}",
                "-".repeat(character),
                "-".repeat(code[line].len() - character - 1),
                msg
            );
        }
    }

    println!("\nExecution completed.");
}
