pub mod parser {
	use crate::lexer::lexer::*;
	use crate::vm::vm::*;
	use bimap::BiMap;

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
		let instr: Vec<Instruction> = Vec::new();
		return Ok(instr);
	}
}