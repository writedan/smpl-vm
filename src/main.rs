pub mod lexer;
pub mod parser;
pub mod vm;

use crate::lexer::lexer::*;
use crate::parser::parser::*;
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

	let mut file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", display, why),
        Ok(file) => file,
    };

    let code: Vec<String> = io::BufReader::new(file).lines().map(|l| l.expect("Could not parse line.")).collect();

    let mut program = Program::load(lexify(code));
}