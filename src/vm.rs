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
			self.vm.memory.resize(u16::max_value() as usize, 0); 

			let mut idx = 0;
			while idx < self.instructions.len() {
				let i = &self.instructions[idx];

				match i {
					Instruction::MoveRight(mut num, token) => {
						self.vm.pointer += num;
						if self.vm.pointer >= self.vm.memory.len() {
							self.vm.pointer = self.vm.pointer - self.vm.memory.len();
						}
					},

					Instruction::MoveLeft(mut num, token) => {
						if num > self.vm.pointer {
							num -= self.vm.pointer;
							self.vm.pointer = self.vm.memory.len();
						}

						self.vm.pointer -= num;
					},

					Instruction::Increment(num, token) => {
						let mut summation = self.vm.memory[self.vm.pointer] as usize + num;
						if summation > u8::max_value() as usize {
							summation -= u8::max_value() as usize + 1;
						}

						self.vm.memory[self.vm.pointer] = summation as u8;
					},

					Instruction::Decrement(mut num, token) => {
						if num > self.vm.memory[self.vm.pointer] as usize {
							num -= (self.vm.memory[self.vm.pointer] as usize) + 1;
							self.vm.memory[self.vm.pointer] = u8::max_value();
						}

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
					},

					Instruction::Jump(token) => {
						self.jumps.push(self.vm.pointer);
						self.vm.pointer = self.vm.memory[self.vm.pointer] as usize;
					},

					Instruction::Restore(token) => {
						if let Some(jump) = self.jumps.pop() {
							self.vm.pointer = jump;
						} else {
							if let Token::Restore(line, character) = token {
								return Err((format!("Runtime error: no saved jumps."), *line, *character));
							}
						}
					},

					Instruction::Alloc(token) => {
						let space_req = self.vm.memory[self.vm.pointer];
						let mut freed = 0;
						let mut address = 0;
						for (idx, value) in self.vm.memory[(idx + 1)..].iter().enumerate() {
							if value == &0 {
								freed += 1;
								address = idx;
							} else {
								freed = 0;
								address = 0;
							}

							if freed == space_req {
								address -= freed as usize;
								break;
							}
						}

						if freed < space_req {
							if let Token::Alloc(line, character) = token {
								return Err((format!("Runtime error: insufficient memeory to free {} bytes.", space_req), *line, *character));
							}
						}

						self.vm.memory[self.vm.pointer] = address as u8;
					}
				}

				idx += 1;
			}

			Ok(())
		}
	}
}