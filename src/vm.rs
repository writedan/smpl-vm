pub mod vm {
	use crate::parser::parser::Instruction;
	use bimap::BiMap;

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
					Instruction::MoveRight => self.machine.pointer += 1,

					Instruction::MoveLeft => {
						if self.machine.pointer == 0 {
							self.machine.pointer = usize::max_value();
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

					Instruction::Nop => {},

					_ => println!("{:?}", i)
				}

				idx += 1;
			}
		}
	}
}