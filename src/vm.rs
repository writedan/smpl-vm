pub mod vm {
	use crate::parser::parser::Instruction;
	use bimap::BiMap;
	use std::io::stdin;
	use std::io::Read;

	struct Vm {
		memory: Vec<u8>,
		jumps: Vec<usize>,
		pointer: usize
	}

	pub struct Program {
		pub instructions: Vec<Instruction>,
		pub branches: BiMap<usize, usize>,
		machine: Vm
	}

	impl Default for Vm {
		fn default() -> Vm {
			return Vm {
				memory: vec![0; u16::max_value() as usize],
				jumps: Vec::new(),
				pointer: 0
			}
		}
	}

	impl Program {
		pub fn new(instructions: Vec<Instruction>, branches: BiMap<usize, usize>) -> Program {
			return Program {
				instructions: instructions,
				branches: branches,
				machine: Vm::default()
			}
		}

		pub fn run(&mut self) {
			let mut idx = 0;
			while idx < self.instructions.len() {
				let i = &self.instructions[idx];
				match i {
					Instruction::MoveRight => {
						self.machine.pointer += 1;
						if self.machine.pointer == u16::max_value() as usize {
							self.machine.pointer = 0;
						} // since pointer is indexed as usize, it can overflow
					},

					Instruction::MoveLeft => {
						if self.machine.pointer == 0 {
							self.machine.pointer = (u16::max_value() as usize) - 1;
						} else {
							self.machine.pointer -= 1;
						}
					},

					Instruction::Increment => self.machine.memory[self.machine.pointer] += 1,

					Instruction::Decrement => {
						if self.machine.memory[self.machine.pointer] == 0 {
							self.machine.memory[self.machine.pointer] = u8::max_value();
						} else {
							self.machine.memory[self.machine.pointer] -= 1;
						}
					},

					Instruction::Output => print!("{}", self.machine.memory[self.machine.pointer] as char),

					Instruction::Input => {
						let mut input: [u8; 1] = [0];
						match stdin().read(&mut input) {
							Ok(read) => {
								self.machine.memory[self.machine.pointer] = input[0];
							},

							Err(error) => {
								panic!("{:?}", error);
							}
						}
					},

					Instruction::Loop => {
						if self.machine.memory[self.machine.pointer] == 0 {
							idx = self.branches.get_by_left(&idx).unwrap() - 1;
						} // otherwise do nothing
					},

					Instruction::Continue => {
						if self.machine.memory[self.machine.pointer] != 0 {
							idx = self.branches.get_by_right(&idx).unwrap() - 1;
						} // otherwise do nothing
					},

					Instruction::Jump => {
						self.machine.jumps.push(self.machine.pointer);
						self.machine.pointer = self.machine.memory[self.machine.pointer] as usize;
					},


					Instruction::Return => {
						self.machine.pointer = self.machine.jumps.pop().unwrap();
					},

					Instruction::Alloc => {
						let space_req = self.machine.memory[self.machine.pointer];
						let mut free = 0;
						let mut space;
						for (position, value) in self.machine.memory.iter().enumerate() {
							if value == &0 {
								free += 1;
								space = position;
							} else {
								free = 0;
								space = 0;
							}

							if free == space_req {
								self.machine.memory[self.machine.pointer] = space as u8;
								break;
							}
						}

						if free != space_req {
							panic!("Insufficient memory was available to allocate {} bytes.", space_req);
						}
					}

					Instruction::Nop => {}
				}

				idx += 1;
			}
		}
	}
}