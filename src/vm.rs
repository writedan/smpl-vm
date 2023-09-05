pub mod vm {
	use bimap::BiMap;
	use crate::lexer::lexer::*;
	use crate::parser::parser::*;

	struct Machine {
		pointer: usize,
		memory: Vec<u8>
	} // basic commands operate on the machine alone

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

		pub fn run(&mut self) {
			let mut idx = 0;
			while idx < self.instructions.len() {
				println!("execute {:?}", self.instructions[idx]);
				idx += 1;
			}
		}
	}
}