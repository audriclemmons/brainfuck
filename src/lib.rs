use num::{FromPrimitive, Zero};
use std::ops::{AddAssign, SubAssign};

#[derive(Debug)]
pub enum Instruction {
    Right { distance: usize }, // >
    Left { distance: usize },  // <

    Add { value: u32 }, // +
    Sub { value: u32 }, // -

    Output, // .
    Input,  // ,

    Open { close_index: usize }, // [
    Close { open_index: usize }, // ]
}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn compile(source: &str) -> Result<Self, &'static str> {
        let mut instructions = Vec::new();
        let mut open_stack = Vec::new();

        for character in source.chars() {
            match character {
                '>' => {
                    if let Some(Instruction::Right { ref mut distance }) = instructions.last_mut() {
                        *distance += 1;
                    } else {
                        instructions.push(Instruction::Right { distance: 1 });
                    }
                }

                '<' => {
                    if let Some(Instruction::Left { ref mut distance }) = instructions.last_mut() {
                        *distance += 1;
                    } else {
                        instructions.push(Instruction::Left { distance: 1 });
                    }
                }

                '+' => {
                    if let Some(Instruction::Add { ref mut value }) = instructions.last_mut() {
                        *value += 1;
                    } else {
                        instructions.push(Instruction::Add { value: 1 });
                    }
                }

                '-' => {
                    if let Some(Instruction::Sub { ref mut value }) = instructions.last_mut() {
                        *value += 1;
                    } else {
                        instructions.push(Instruction::Sub { value: 1 });
                    }
                }

                '.' => {
                    instructions.push(Instruction::Output);
                }

                ',' => {
                    instructions.push(Instruction::Input);
                }

                '[' => {
                    instructions.push(Instruction::Open { close_index: 0 });
                    open_stack.push(instructions.len() - 1);
                }

                ']' => {
                    let open_index = open_stack.pop().ok_or("unmatched closing bracket(s)")?;
                    instructions.push(Instruction::Close { open_index });
                    instructions[open_index] = Instruction::Open {
                        close_index: instructions.len() - 1,
                    };
                }

                _ => {}
            }
        }

        if !open_stack.is_empty() {
            return Err("unmatched opening bracket(s)");
        }

        Ok(Program { instructions })
    }
}

const MEMORY_SIZE: usize = 65536;

pub struct Machine<T: Copy + AddAssign + SubAssign + Zero + FromPrimitive> {
    memory: [T; MEMORY_SIZE],

    pc: usize,
    pointer: usize,

    input: fn() -> T,
    output: fn(T),
}

impl<T: Copy + AddAssign + SubAssign + Zero + FromPrimitive> Machine<T> {
    pub fn new(input: fn() -> T, output: fn(T)) -> Machine<T> {
        Machine {
            memory: [T::zero(); MEMORY_SIZE],

            pc: 0,
            pointer: 0,
            
            input,
            output,
        }
    }

    #[inline(always)]
    fn current(&mut self) -> &mut T {
        &mut self.memory[self.pointer]
    }

    pub fn execute(&mut self, program: Program) {
        let instructions = program.instructions;

        while let Some(instruction) = instructions.get(self.pc) {
            match instruction {
                Instruction::Right { distance } => {
                    self.pointer += *distance;

                    if self.pointer >= MEMORY_SIZE {
                        self.pointer -= MEMORY_SIZE;
                    }
                }

                Instruction::Left { distance } => {
                    if *distance > self.pointer {
                        self.pointer += MEMORY_SIZE;
                    }

                    self.pointer -= *distance;
                }

                Instruction::Add { value } => {
                    *self.current() += T::from_u32(*value).unwrap();
                }

                Instruction::Sub { value } => {
                    *self.current() -= T::from_u32(*value).unwrap();
                }

                Instruction::Output => {
                    (self.output)(*self.current());
                }

                Instruction::Input => {
                    *self.current() = (self.input)();
                }

                Instruction::Open { close_index } => {
                    if self.current().is_zero() {
                        self.pc = *close_index;
                    }
                }

                Instruction::Close { open_index } => {
                    if !self.current().is_zero() {
                        self.pc = *open_index;
                    }
                }
            }

            self.pc += 1;
        }
    }
}
