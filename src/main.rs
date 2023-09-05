use std::env;

pub mod vm;
pub mod parser;
use crate::parser::parser::*;
use crate::vm::vm::Program;


fn main() {
	let args: Vec<String> = env::args().collect();
	let code = args[1].chars().collect::<Vec<char>>();

	let instructions: Vec<Instruction> = parse_instructions(code);
	let branches = calculate_branches(&instructions).unwrap();

	let mut vm = Program::new(instructions, branches);

	println!("Instructions: {:?}", vm.instructions);
	println!("Branch table: {:?}", vm.branches);

	println!("Begin execution:\n");

	vm.run();

	println!();
}