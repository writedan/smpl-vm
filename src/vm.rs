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
		pub fn load(program: Vec<Token>) -> Program {
			Program {
				instructions: parse(program).unwrap(),
				branches: BiMap::new(),
				jumps: Vec::new(),
				vm: Machine {
					pointer: 0,
					memory: Vec::new()
				}
			}
		}
	}
}