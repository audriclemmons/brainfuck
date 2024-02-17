use crate::{value::Value, program::{Program, Instruction}};

const MEMORY_SIZE: usize = 65536;

pub struct Machine<T: Value> {
    memory: [T; MEMORY_SIZE],

    pc: usize,
    pointer: usize,

    input: fn() -> T,
    output: fn(&T),
}

impl<T: Value> Machine<T> {
    pub fn new(input: fn() -> T, output: fn(&T)) -> Machine<T> {
        Machine {
            memory: [(); MEMORY_SIZE].map(|_| T::ZERO),

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
                Instruction::Add { offset, n } => {
                    self.get_memory(*offset).add(*n);
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
