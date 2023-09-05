pub mod parser {
	use crate::lexer::lexer::*;
	use crate::vm::vm::*;
	use bimap::BiMap;
	use std::mem::{discriminant,Discriminant};

	#[derive(Debug)]
	pub enum Instruction {
		MoveRight(usize, Token),	// pointer += 1
		MoveLeft(usize, Token),		// pointer -= 1
		Increment(usize, Token),	// memory[pointer] += 1
		Decrement(usize, Token),	// memory[pointer] -= 1
		Output(Token),				// print memory[pointer]
		Input(Token),				// input -> memory[pointer]
		Branch(Token),				// if memory[pointer] = 0, jump to matching ]
		Return(Token),				// if memory[pointer] != 0, jump to matchong [
		Jump(Token),				// pointer = memory[pointer]
		Restore(Token),				// restore pointer to value before jump
		Alloc(Token),				// memory[pointer] = address of first `memory[pointer]` empty cells
	} // usize field is how many times in a row the token appears; Token is a reference to the corresponding token for its line and character index

	pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, (String, usize, usize)> {
		let mut instr: Vec<Instruction> = Vec::new();

		let mut idx = 0;
		while idx < tokens.len() {
			let token = &tokens[idx];
			match token {
				Token::MoveRight(line, character) => {
					let num = count_tokens(tokens[idx..].to_vec(), discriminant(token));
					idx += num;
					instr.push(Instruction::MoveRight(num, *token));
				},

				Token::MoveLeft(line, character) => {
					let num = count_tokens(tokens[idx..].to_vec(), discriminant(token));
					idx += num;
					instr.push(Instruction::MoveLeft(num, *token));
				},

				Token::Increment(line, character) => {
					let num = count_tokens(tokens[idx..].to_vec(), discriminant(token));
					idx += num;
					instr.push(Instruction::Increment(num, *token));
				},

				Token::Decrement(line, chracter) => {
					let num = count_tokens(tokens[idx..].to_vec(), discriminant(token));
					idx += num;
					instr.push(Instruction::Decrement(num, *token));
				}

				Token::Output(line, character) => {
					instr.push(Instruction::Output(*token));
				},

				Token::Input(line, character) => {
					instr.push(Instruction::Input(*token));
				},

				Token::Branch(line, character) => {
					instr.push(Instruction::Branch(*token));
				},

				Token::Return(line, character) => {
					instr.push(Instruction::Return(*token));
				},

				Token::Jump(line, character) => {
					instr.push(Instruction::Jump(*token));
				},

				Token::Restore(line, character) => {
					instr.push(Instruction::Restore(*token));
				},

				Token::Alloc(line, character) => {
					instr.push(Instruction::Alloc(*token));
				},

				Token::Nop(line, character) => {
					return Err((format!("Unexpected token {:?} encountered on line {} at {}", token, line, character), *line, *character));
				}
			}

			idx += 1;
		}

		return Ok(instr);
	}

	fn count_tokens(tokens: Vec<Token>, token: Discriminant<Token>) -> usize {
		let mut idx = 0;
		let mut num = 0;
		while idx < tokens.len() {
			let tok = &tokens[idx];
			if discriminant(tok) == token {
				num += 1;
				idx += 1;
			} else {
				break;
			}
		}

		return num;
	}

	pub fn calculate_branches(instr: &Vec<Instruction>) -> Result<BiMap<usize, usize>, (String, usize, usize)> {
		let mut branches: BiMap<usize, usize> = BiMap::new();

		for (position, instruction) in instr.iter().enumerate() {
			if discriminant(instruction) == discriminant(&Instruction::Branch(Token::Nop(0,0))) {
				let open = position;
				let mut offset = 0;
				for (position, instruction) in instr[position + 1..].iter().enumerate() {
					if discriminant(instruction) == discriminant(&Instruction::Branch(Token::Nop(0,0))) {
						offset += 1;
					} else if discriminant(instruction) == discriminant(&Instruction::Return(Token::Nop(0,0))) {
						if offset == 0 {
							branches.insert(open, position + open + 1);
							break;
						} else {
							offset -= 1;
						}
					}
				}

				if !branches.contains_left(&open) {
					if let Instruction::Branch(token) = instruction {
						if let Token::Branch(line, character) = token {
							return Err((format!("Branch ('[') defined on line {} at {} has no return (']')", line, character), *line, *character));
						}
					}
				}
			} else if discriminant(instruction) == discriminant(&Instruction::Return(Token::Nop(0,0))) {
				if !branches.contains_right(&position) {
					if let Instruction::Return(token) = instruction {
						if let Token::Return(line, character) = token {
							return Err((format!("Return (']') defined on line {} at {} has no branch ('[')", line, character), *line, *character));
						}
					}
				}
			}
		}

		return Ok(branches);
	}
}