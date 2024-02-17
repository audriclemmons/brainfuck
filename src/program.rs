use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
    Add { offset: isize, n: i32 },

    Output { offset: isize },
    Input { offset: isize },

    Open { offset: isize, close_index: usize },
    Close { offset: isize, open_index: usize },
}

#[derive(Debug)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn compile(source: &str) -> Result<Self, &'static str> {
        let mut instructions = Vec::new();

        let mut add_map: HashMap<isize, i32> = HashMap::new();
        let mut offset: isize = 0;

        let mut open_stack = Vec::new();

        for character in source.chars() {
            match character {
                '>' => {
                    offset += 1;
                }

                '<' => {
                    offset -= 1;
                }

                '+' => {
                    if let Some(ref mut offset) = add_map.get_mut(&offset) {
                        **offset += 1;
                    } else {
                        add_map.insert(offset, 1);
                    }
                }

                '-' => {
                    if let Some(ref mut offset) = add_map.get_mut(&offset) {
                        **offset -= 1;
                    } else {
                        add_map.insert(offset, -1);
                    }
                }

                '.' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, n)| Instruction::Add { offset, n }),
                    );

                    instructions.push(Instruction::Output { offset });
                }

                ',' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, n)| Instruction::Add { offset, n }),
                    );

                    instructions.push(Instruction::Input { offset });
                }

                '[' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, n)| Instruction::Add { offset, n }),
                    );

                    instructions.push(Instruction::Open { offset, close_index: 0 });
                    open_stack.push(instructions.len() - 1);

                    offset = 0;
                }

                ']' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, n)| Instruction::Add { offset, n }),
                    );

                    let open_index = open_stack.pop().ok_or("unmatched closing bracket(s)")?;

                    instructions.push(Instruction::Close { offset, open_index });
                    
                    let current_index = instructions.len() - 1;
                    if let Instruction::Open { offset: _, ref mut close_index } = instructions[open_index] {
                        *close_index = current_index;
                    }

                    offset = 0;
                }

                _ => {}
            }
        }

        instructions.extend(
            add_map
                .drain()
                .map(|(offset, n)| Instruction::Add { offset, n }),
        );

        if !open_stack.is_empty() {
            return Err("unmatched opening bracket(s)");
        }

        Ok(Program { instructions })
    }
}

