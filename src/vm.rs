pub mod vm {
	use bimap::BiMap;
	use crate::lexer::lexer::*;
	use crate::parser::parser::*;
	use std::io::stdin;
	use std::io::Read;

	#[derive(Debug)]
	struct Machine {
		pointer: usize,
		memory: Vec<u8>
	} // basic commands operate on the machine alone

	#[derive(Debug)]
	pub struct Program {
		instructions: Vec<Instruction>,
		branches: BiMap<usize, usize>,
		jumps: Vec<usize>,
		vm: Machine
	} // more complex commands operate on the program

	impl Program {
		pub fn load(program: Vec<Token>) -> Result<Program, (String, usize, usize)> {
			let instructions = match parse(program) {
				Ok(instructions) => instructions,
				Err((msg, line, chracter)) => return Err((msg, line, chracter))
			};

			let branches = match calculate_branches(&instructions) {
				Ok(branches) => branches,
				Err((msg, line, chracter)) => return Err((msg, line, chracter))
			};

			Ok(Program {
				instructions: instructions,
				branches: branches,
				jumps: Vec::new(),
				vm: Machine {
					pointer: 0,
					memory: Vec::new()
				}
			})
		}

		pub fn run(&mut self) -> Result<(), (String, usize, usize)> {
			self.vm.memory.resize(6, 0); // initialize memory with 6 registers

			let mut idx = 0;
			while idx < self.instructions.len() {
				let i = &self.instructions[idx];

				match i {
					Instruction::MoveRight(num, token) => {
						self.vm.pointer += num;

						if self.vm.pointer >= self.vm.memory.len() {
							self.vm.memory.resize(self.vm.pointer + 1, 0);
						}

						if self.vm.memory.len() > u8::max_value() as usize {
							self.vm.pointer = 0;
						}
					},

					Instruction::MoveLeft(mut num, token) => {
						self.vm.pointer -= num;
					},

					Instruction::Increment(num, token) => {
						self.vm.memory[self.vm.pointer] += *num as u8;
					},

					Instruction::Decrement(mut num, token) => {
						self.vm.memory[self.vm.pointer] -= num as u8;
					},

					Instruction::Output(token) => {
						print!("{}", self.vm.memory[self.vm.pointer] as char);
					},

					Instruction::Input(token) => {
						let mut input: [u8; 1] = [0];
						match stdin().read(&mut input) {
							Ok(read) => {
								self.vm.memory[self.vm.pointer] = input[0];
							},

							Err(error) => {
								if let Token::Nop(line, character) = token {
									return Err((format!("Runtime error occurred getting input: {}", error), *line, *character));
								}
							}
						}
					},

					Instruction::Branch(token) => {
						if self.vm.memory[self.vm.pointer] == 0 {
							idx = self.branches.get_by_left(&idx).unwrap() - 1 ;
						}
					},

					Instruction::Return(token) => {
						if self.vm.memory[self.vm.pointer] != 0 {
							idx = self.branches.get_by_right(&idx).unwrap() - 1;
						}
					}

					_ => println!("todo: execute: {:?}", i)
				}

				idx += 1;
			}

			Ok(())
		}
	}
}