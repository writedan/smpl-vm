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

	pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, String> {
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

				Token::Output(line, character) => {
					instr.push(Instruction::Output(*token));
				}

				_ => println!("unsupported token {:?}", token)
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

	pub fn calculate_branches(instr: Vec<Instruction>) -> Result<BiMap<usize, usize>, String> {
		return Err(format!("unsupported"));
	}
}