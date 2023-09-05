pub mod parser {
	use bimap::BiMap;

	#[derive(Debug, PartialEq)]
	pub enum Instruction {
		MoveRight,		// increment the pointer by one
		MoveLeft,		// decrement the pointer by one
		Increment,		// increment the memory at pointer by one
		Decrement,		// decrement the memory at pointer by one
		Output,			// output byte from memory
		Input,			// input byte into memory
		Loop,			// jump past the matching ] if memory is zero
		Continue,		// jump to matching [ if memory is nonzero
		Jump,			// set pointer to value in memory
		Return,			// restore pointer to value before jump
		Alloc,			// with the value of memory, jump to first instance of that many 0s in memory
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