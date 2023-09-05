pub mod lexer {

	#[derive(PartialEq, Debug, Copy, Clone)]
	pub enum Token {
		MoveRight(usize, usize),		// pointer += 1
		MoveLeft(usize, usize),			// pointer -= 1
		Increment(usize, usize),		// memory[pointer] += 1
		Decrement(usize, usize),		// memory[pointer] -= 1
		Output(usize, usize),			// print memory[pointer]
		Input(usize, usize),			// input -> memory[pointer]
		Branch(usize, usize),			// if memory[pointer] = 0, jump to matching ]
		Return(usize, usize),			// if memory[pointer] != 0, jump to matchong [
		Jump(usize, usize),				// pointer = memory[pointer]
		Restore(usize, usize),			// restore pointer to value before jump
		Alloc(usize, usize),			// memory[pointer] = address of first `memory[pointer]` empty cells
		Nop								// placeholder empty value, will be pruned
	} // fields are (line, character)

	pub fn lexify(code: Vec<String>) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		for (idx, line) in code.iter().enumerate() {
			let line_num = idx;
			for (idx, character) in line.chars().enumerate() {
				tokens.push(match character {
					'>' => Token::MoveRight(line_num, idx),
					'<' => Token::MoveLeft(line_num, idx),
					'+' => Token::Increment(line_num, idx),
					'-' => Token::Decrement(line_num, idx),
					'.' => Token::Output(line_num, idx),
					',' => Token::Input(line_num, idx),
					'[' => Token::Branch(line_num, idx),
					']' => Token::Return(line_num, idx),
					'*' => Token::Jump(line_num, idx),
					'&' => Token::Restore(line_num, idx),
					'?' => Token::Alloc(line_num, idx),
					_ => Token::Nop
				});
			}
		}

		return tokens.into_iter().filter(|token| token != &Token::Nop).collect::<Vec<Token>>();
	}
}