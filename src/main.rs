mod parser;

use std::env;
use crate::parser::*;


fn main() {
	let args: Vec<String> = env::args().collect();
	let code = args[1].chars().collect::<Vec<char>>();

	let mut memory: Vec<u8> = vec![0; u16::max_value() as usize];

	let instr: Vec<Instruction> = parse_instructions(code);
	let branches = calculate_branches(&instr).unwrap();

	let mut ptr_jumps: Vec<usize> = vec![0, u16::max_value() as usize];
	let mut pointer: usize = 0;

	println!("Instructions: {:?}", instr);
	println!("Branch table: {:?}", branches);

	let mut iter = instr.iter().enumerate();
	while let Some((position, instruction)) = iter.next() {
		println!("execute: {:?} at {}", instruction, position);
		match instruction {
			Instruction::MoveRight => pointer += 1,

			Instruction::MoveLeft => {
				if pointer == 0 {
					pointer = usize::max_value();
				} else {
					pointer -= 1;
				}
			},

			Instruction::Increment => memory[pointer] += 1,

			Instruction::Decrement => {
				if memory[pointer] == 0 {
					memory[pointer] = u8::max_value();
				} else {
					memory[pointer] -= 1;
				}
			},

			Instruction::Output => print!("{:#?}", memory[pointer]),

			Instruction::Input => panic!("Unsupported operation {:?} at position {}", instruction, position),

			_ => println!("??? {:?} at {}", instruction, position)
		}
	}

	println!();
}