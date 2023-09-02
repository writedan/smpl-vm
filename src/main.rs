use std::env;

#[derive(Debug)]
enum Instruction {
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

fn main() {
	let args: Vec<String> = env::args().collect();
	let code = &args[1];

	let mut memory: Vec<u8> = Vec::with_capacity(u16::max_value() as usize);
	let mut instr: Vec<Instruction> = Vec::with_capacity(code.len());

	for instruction in code.chars() {
		let instruction = match instruction {
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
			_ => Instruction::Nop // specification defiens other symbols to be ignored
		};

		instr.push(instruction);
	}

	let mut pointer: usize = 0;
}