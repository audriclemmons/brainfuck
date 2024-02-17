use std::io::{Read, Write};
use crate::{value::Value, program::{Program, Instruction}};

const MEMORY_SIZE: usize = 65536;

pub struct Machine<T: Value> {
    memory: [T; MEMORY_SIZE],

    pc: usize,
    pointer: usize,

    input: Box<dyn Read>,
    output: Box<dyn Write>,
}

impl<T: Value> Machine<T> {
    pub fn new(input: impl Read + 'static, output: impl Write + 'static) -> Machine<T> {
        Machine {
            memory: [(); MEMORY_SIZE].map(|_| T::ZERO),

            pc: 0,
            pointer: 0,

            input: Box::new(input),
            output: Box::new(output),
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
                    let mut buffer: [u8; 1] = [0];
                    self.input.read(&mut buffer);
                    self.get_memory(*offset).set_byte(buffer[0]);
                }

                Instruction::Output { offset } => {
                    let buffer: [u8; 1] = [self.get_memory(*offset).get_byte()];
                    self.output.write(&buffer);
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
