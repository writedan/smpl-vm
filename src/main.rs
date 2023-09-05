mod parser;

use std::env;
use crate::parser::*;


fn main() {
	let args: Vec<String> = env::args().collect();
	let code = args[1].chars().collect::<Vec<char>>();

	let mut memory: Vec<u8> = vec![0; u16::max_value() as usize];

	let mut instr: Vec<Instruction> = parse_instructions(code);
	let branches = calculate_branches(&instr).unwrap();

	let mut ptr_jumps: Vec<usize> = vec![0, u16::max_value() as usize];
	let mut pointer: usize = 0;

	println!("Instructions: {:?}", instr);
	println!("Branch table: {:?}", branches);

	println!();
}