pub mod parser {
	use bimap::BiMap;

	#[derive(Debug, PartialEq)]
	pub enum Instruction {
		MoveRight,		// pointer += 1
		MoveLeft,		// pointer -= 1
		Increment,		// memory[pointer] += 1
		Decrement,		// memory[pointer] -= 1
		Output,			// print memory[pointer]
		Input,			// memory[pointer] input
		Loop,			// jump past the matching ] if memory[pointer] = 0
		Continue,		// jump to matching [ if memory[pointer] > 0
		Jump,			// pointer = memory[pointer]
		Return,			// restore pointer to value before jump
		Alloc,			// with the value of memory, set memory[pointer] to address of first instance of that many free spaces, defined as memory[pointer] = 0
		Nop				// no operation
	}

	impl PartialEq<Option<Instruction>> for &Instruction {
		fn eq(&self, other: &Option<Instruction>) -> bool {
	        match other {
	            Some(other) => return other == *self,
	            None => return false,
	        }
	    }
	}

	pub fn parse_instructions(commands: Vec<char>) -> Vec<Instruction> {
		let mut instr: Vec<Instruction> = Vec::with_capacity(commands.len());

		for command in commands {
			instr.push(match command {
				'>' => Instruction::MoveRight,
				'<' => Instruction::MoveLeft,
				'+' => Instruction::Increment,
				'-' => Instruction::Decrement,
				'.' => Instruction::Output,
				',' => Instruction::Input,
				'[' => Instruction::Loop,
				']' => Instruction::Continue,
				'*' => Instruction::Jump,
				'&' => Instruction::Return,
				'?' => Instruction::Alloc,
				_ => Instruction::Nop
			});
		}

		return instr;
	}

	pub fn calculate_branches(instr: &Vec<Instruction>) -> Result<BiMap<usize, usize>, String> {
		let mut branches: BiMap<usize, usize>  = BiMap::new(); // <open, close>

		for (position, instruction) in instr.iter().enumerate() {
			if instruction == &Instruction::Loop {
				let open = position;
				let mut offset = 0;
				for (position, instruction) in instr[position + 1..].iter().enumerate() {
					if instruction == &Instruction::Loop {
						offset += 1;
					} else if instruction == &Instruction::Continue {
						if offset == 0 {
							branches.insert(open, position + open + 1);
							break;
						} else {
							offset -= 1;
						}
					}
				}

				if !branches.contains_left(&open) {
					return Err(format!("Loop defined at instruction position {} has no continue.", open));
				}
			} else if instruction == &Instruction::Continue {
				if !branches.contains_right(&position) {
					return Err(format!("Continue defined at instruction position {} has no loop.", position));
				}
			}
		}

		return Ok(branches);
	}
}