use num::{FromPrimitive, Zero};
use std::collections::HashMap;
use std::ops::{AddAssign, SubAssign};

#[derive(Debug)]
pub enum Instruction {
    Add { offset: isize, value: i32 },

    Output { offset: isize },
    Input { offset: isize },

    Open { offset: isize, close_index: usize },
    Close { offset: isize, open_index: usize },
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
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
                            .map(|(offset, value)| Instruction::Add { offset, value }),
                    );

                    instructions.push(Instruction::Output { offset });
                }

                ',' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, value)| Instruction::Add { offset, value }),
                    );

                    instructions.push(Instruction::Input { offset });
                }

                '[' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, value)| Instruction::Add { offset, value }),
                    );

                    instructions.push(Instruction::Open { offset, close_index: 0 });
                    open_stack.push(instructions.len() - 1);

                    offset = 0;
                }

                ']' => {
                    instructions.extend(
                        add_map
                            .drain()
                            .map(|(offset, value)| Instruction::Add { offset, value }),
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
                .map(|(offset, value)| Instruction::Add { offset, value }),
        );

        if !open_stack.is_empty() {
            return Err("unmatched opening bracket(s)");
        }

        Ok(Program { instructions })
    }
}

const MEMORY_SIZE: usize = 65536;

pub struct Machine<T: AddAssign + SubAssign + Zero + FromPrimitive> {
    memory: [T; MEMORY_SIZE],

    pc: usize,
    pointer: usize,

    input: fn() -> T,
    output: fn(&T),
}

impl<T: AddAssign + SubAssign + Zero + FromPrimitive> Machine<T> {
    pub fn new(input: fn() -> T, output: fn(&T)) -> Machine<T> {
        Machine {
            memory: [(); MEMORY_SIZE].map(|_| T::zero()),

            pc: 0,
            pointer: 0,

            input,
            output,
        }
    }

    #[inline]
    fn get_memory(&mut self, offset: isize) -> &mut T {
        &mut self.memory[self.pointer.wrapping_add_signed(offset)]
    }

    pub fn execute(&mut self, program: Program) {
        let instructions = program.instructions;

        while let Some(instruction) = instructions.get(self.pc) {
            match instruction {
                Instruction::Add { offset, value } => {
                    if *value > 0 {
                        *self.get_memory(*offset) += T::from_i32(*value).unwrap();
                    } else {
                        *self.get_memory(*offset) -= T::from_i32(-*value).unwrap();
                    }
                }

                Instruction::Input { offset } => {
                    *self.get_memory(*offset) = (self.input)();
                }

                Instruction::Output { offset } => {
                    (self.output)(self.get_memory(*offset));
                }

                Instruction::Open { offset, close_index } => {
                    if self.get_memory(*offset).is_zero() {
                        self.pc = *close_index;
                    }

                    self.pointer = self.pointer.wrapping_add_signed(*offset);
                }

                Instruction::Close { offset, open_index } => {
                    if !self.get_memory(*offset).is_zero() {
                        self.pc = *open_index;
                    }

                    self.pointer = self.pointer.wrapping_add_signed(*offset);
                }
            }

            self.pc += 1;
        }
    }
}
