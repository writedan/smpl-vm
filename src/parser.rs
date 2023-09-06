pub mod parser {
    use crate::lexer::lexer::*;

    use bimap::BiMap;
    use std::mem::{discriminant, Discriminant};

    #[derive(Debug)]
    pub enum Instruction {
        MoveRight(usize, Token), // pointer += 1
        MoveLeft(usize, Token),  // pointer -= 1
        Increment(usize, Token), // memory[pointer] += 1
        Decrement(usize, Token), // memory[pointer] -= 1
        Output(Token),           // print memory[pointer]
        Input(Token),            // input -> memory[pointer]
        Branch(Token),           // if memory[pointer] = 0, jump to matching ]
        Return(Token),           // if memory[pointer] != 0, jump to matchong [
        Jump(Token),             // pointer = memory[pointer]
        Restore(Token),          // restore pointer to value before jump
        Alloc(Token),            // memory[pointer] = address of first `memory[pointer]` empty cells
    } // usize field is how many times in a row the token appears; Token is a reference to the corresponding token for its line and character index

    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, (String, usize, usize)> {
        let mut instr: Vec<Instruction> = Vec::new();

        let mut idx = 0;
        let mut last_instruction = &Instruction::Branch(Token::Nop(0, 0));
        while idx < tokens.len() {
            let token = &tokens[idx];
            match token {
                Token::MoveRight(_, _) => {
                    if let Instruction::MoveRight(num, token) = last_instruction {
                        instr.push(Instruction::MoveRight(num + 1, *token));
                        instr.remove(instr.len() - 2);
                    } else {
                        instr.push(Instruction::MoveRight(1, *token));
                    }
                }

                Token::MoveLeft(_, _) => {
                    if let Instruction::MoveLeft(num, token) = last_instruction {
                        instr.push(Instruction::MoveLeft(num + 1, *token));
                        instr.remove(instr.len() - 2);
                    } else {
                        instr.push(Instruction::MoveLeft(1, *token));
                    }
                }

                Token::Increment(_, _) => {
                    if let Instruction::Increment(num, token) = last_instruction {
                        instr.push(Instruction::Increment(num + 1, *token));
                        instr.remove(instr.len() - 2);
                    } else {
                        instr.push(Instruction::Increment(1, *token));
                    }
                }

                Token::Decrement(_, _) => {
                    if let Instruction::Decrement(num, token) = last_instruction {
                        instr.push(Instruction::Decrement(num + 1, *token));
                        instr.remove(instr.len() - 2);
                    } else {
                        instr.push(Instruction::Decrement(1, *token));
                    }
                }

                Token::Output(_, _) => {
                    instr.push(Instruction::Output(*token));
                }

                Token::Input(_, _) => {
                    instr.push(Instruction::Input(*token));
                }

                Token::Branch(_, _) => {
                    instr.push(Instruction::Branch(*token));
                }

                Token::Return(_, _) => {
                    instr.push(Instruction::Return(*token));
                }

                Token::Jump(_, _) => {
                    instr.push(Instruction::Jump(*token));
                }

                Token::Restore(_, _) => {
                    instr.push(Instruction::Restore(*token));
                }

                Token::Alloc(_, _) => {
                    instr.push(Instruction::Alloc(*token));
                }

                Token::Nop(line, character) => {
                    return Err((
                        format!("Unexpected token {:?} encountered", token),
                        *line,
                        *character,
                    ));
                }
            }
 
            last_instruction = instr.last().unwrap();
            idx += 1;
        }

        return Ok(instr);
    }

    pub fn calculate_branches(
        instr: &Vec<Instruction>,
    ) -> Result<BiMap<usize, usize>, (String, usize, usize)> {
        let mut branches: BiMap<usize, usize> = BiMap::new();

        for (position, instruction) in instr.iter().enumerate() {
            if discriminant(instruction) == discriminant(&Instruction::Branch(Token::Nop(0, 0))) {
                let open = position;
                let mut offset = 0;
                for (position, instruction) in instr[position + 1..].iter().enumerate() {
                    if discriminant(instruction)
                        == discriminant(&Instruction::Branch(Token::Nop(0, 0)))
                    {
                        offset += 1;
                    } else if discriminant(instruction)
                        == discriminant(&Instruction::Return(Token::Nop(0, 0)))
                    {
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
                            return Err((
                                format!("Branch ('[') has no return (']')"),
                                *line,
                                *character,
                            ));
                        }
                    }
                }
            } else if discriminant(instruction)
                == discriminant(&Instruction::Return(Token::Nop(0, 0)))
            {
                if !branches.contains_right(&position) {
                    if let Instruction::Return(token) = instruction {
                        if let Token::Return(line, character) = token {
                            return Err((
                                format!("Return (']') has no branch ('[')"),
                                *line,
                                *character,
                            ));
                        }
                    }
                }
            }
        }

        return Ok(branches);
    }
}
