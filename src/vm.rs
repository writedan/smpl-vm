pub mod vm {
    use crate::lexer::lexer::*;
    use crate::parser::parser::*;
    use bimap::BiMap;
    
    use std::io::BufRead;
    use std::io::{Write};

    #[derive(Debug)]
    struct Machine {
        pointer: usize,
        memory: Vec<usize>,
    } // basic commands operate on the machine alone

    #[derive(Debug)]
    pub struct Program {
        instructions: Vec<Instruction>,
        branches: BiMap<usize, usize>,
        jumps: Vec<usize>,
        vm: Machine,
    } // more complex commands operate on the program

    impl Program {
        pub fn load(program: Vec<Token>) -> Result<Program, (String, usize, usize)> {
            let instructions = match parse(program) {
                Ok(instructions) => instructions,
                Err((msg, line, chracter)) => return Err((msg, line, chracter)),
            };

            let branches = match calculate_branches(&instructions) {
                Ok(branches) => branches,
                Err((msg, line, chracter)) => return Err((msg, line, chracter)),
            };

            Ok(Program {
                instructions,
                branches,
                jumps: Vec::new(),
                vm: Machine {
                    pointer: 0,
                    memory: Vec::new(),
                },
            })
        }

        pub fn run(&mut self) -> Result<(), (String, usize, usize)> {
            self.vm.memory.resize(u16::max_value() as usize, 0);

            let mut idx = 0;
            let mut clock = 0;
            while idx < self.instructions.len() {
                let i = &self.instructions[idx];

                match i {
                    Instruction::MoveRight(num, _) => {
                        self.vm.pointer += num;
                        if self.vm.pointer >= self.vm.memory.len() {
                            self.vm.pointer -= self.vm.memory.len();
                        }
                    }

                    Instruction::MoveLeft(mut num, _) => {
                        if num > self.vm.pointer {
                            num -= self.vm.pointer;
                            self.vm.pointer = self.vm.memory.len();
                        }

                        self.vm.pointer -= num;
                    }

                    Instruction::Increment(num, _) => {
                        let mut summation = self.vm.memory[self.vm.pointer] + num;
                        if summation > u8::max_value() as usize {
                            summation -= u8::max_value() as usize + 1;
                        }

                        self.vm.memory[self.vm.pointer] = summation;
                    }

                    Instruction::Decrement(mut num, _) => {
                        if num > self.vm.memory[self.vm.pointer] {
                            num -= self.vm.memory[self.vm.pointer] + 1;
                            self.vm.memory[self.vm.pointer] = u8::max_value() as usize;
                        }

                        self.vm.memory[self.vm.pointer] -= num;
                    }

                    Instruction::Output(token) => {
                        if self.vm.memory[self.vm.pointer] > u8::max_value() as usize {
                            if let Token::Output(line, character) = token {
                                return Err((
                                    format!(
                                        "Runtime error: mem[{}] = {} > {}",
                                        self.vm.pointer,
                                        self.vm.memory[self.vm.pointer],
                                        u8::max_value()
                                    ),
                                    *line,
                                    *character,
                                ));
                            }
                        } else {
                            print!("{}", (self.vm.memory[self.vm.pointer] as u32) as u8 as char);
                            match std::io::stdout().flush() {
                                Ok(_) => {}
                                Err(msg) => {
                                    if let Token::Output(line, character) = token {
                                        return Err((
                                            format!("Runtime error flushing output: {}", msg),
                                            *line,
                                            *character,
                                        ));
                                    }
                                }
                            }

                            match std::io::stdout().flush() {
                                Ok(_) => {}
                                Err(msg) => {
                                    if let Token::Output(line, character) = token {
                                        return Err((
                                            format!("Runtime error flushing output: {}", msg),
                                            *line,
                                            *character,
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    Instruction::Input(token) => {
                        let mut line = String::new();
                        let stdin = std::io::stdin();
                        match stdin.lock().read_line(&mut line) {
                            Ok(_) => {
                                line = line.trim().to_string();
                                match line.chars().nth(0) {
                                    Some(byte) => {
                                        self.vm.memory[self.vm.pointer] = byte as usize;
                                    }
                                    None => {
                                        self.vm.memory[self.vm.pointer] = 0;
                                    }
                                }
                            }
                            Err(msg) => {
                                if let Token::Input(line, character) = token {
                                    return Err((
                                        format!("Runtime error reading input: {}", msg),
                                        *line,
                                        *character,
                                    ));
                                }
                            }
                        }
                    }

                    Instruction::Branch(_) => {
                        if self.vm.memory[self.vm.pointer] == 0 {
                            idx = self.branches.get_by_left(&idx).unwrap() - 1;
                        }
                    }

                    Instruction::Return(_) => {
                        if self.vm.memory[self.vm.pointer] != 0 {
                            idx = self.branches.get_by_right(&idx).unwrap() - 1;
                        }
                    }

                    Instruction::Jump(_) => {
                        self.jumps.push(self.vm.pointer);
                        self.vm.pointer = self.vm.memory[self.vm.pointer];
                    }

                    Instruction::Restore(token) => {
                        if let Some(jump) = self.jumps.pop() {
                            self.vm.pointer = jump;
                        } else if let Token::Restore(line, character) = token {
                            return Err((
                                "Runtime error: no saved jumps.".to_string(),
                                *line,
                                *character,
                            ));
                        }
                    }

                    Instruction::Alloc(token) => {
                        let space_req = self.vm.memory[self.vm.pointer];
                        let base = idx;
                        let mut freed: usize = 0;
                        let mut address = 0;
                        for (idx, value) in self.vm.memory[(idx + 1)..].iter().enumerate() {
                            if value == &0 {
                                freed += 1;
                                address = idx + base + 1;
                            } else {
                                freed = 0;
                                address = 0;
                            }

                            if freed == space_req {
                                address -= freed;
                                break;
                            }
                        }

                        if freed < space_req {
                            if let Token::Alloc(line, character) = token {
                                return Err((
                                    format!(
                                        "Runtime error: insufficient memeory to free {} bytes.",
                                        space_req
                                    ),
                                    *line,
                                    *character,
                                ));
                            }
                        }

                        self.vm.memory[self.vm.pointer] = address;
                    }
                }

                idx += 1;
                clock += 1;
            }

            Ok(())
        }
    }
}
