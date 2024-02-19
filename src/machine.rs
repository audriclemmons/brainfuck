use crate::{
    program::{Instruction, Program},
    value::Value,
};
use std::io::{Read, Write};

const MEMORY_SIZE: usize = 65536;

pub struct Machine<T: Value> {
    memory: [T; MEMORY_SIZE],
    pointer: usize,

    pc: usize,
}

impl<T: Value> Machine<T> {
    fn new() -> Machine<T> {
        Machine {
            memory: [(); MEMORY_SIZE].map(|_| T::ZERO),
            pointer: 0,

            pc: 0,
        }
    }

    #[inline]
    fn get_memory(&mut self, offset: isize) -> &mut T {
        &mut self.memory[self.pointer.wrapping_add_signed(offset)]
    }

    pub fn execute(program: &Program, io: &mut (impl Read + Write)) {
        let mut machine: Machine<T> = Machine::new();

        while let Some(instruction) = program.get(machine.pc) {
            match instruction {
                Instruction::Add { offset, n } => {
                    machine.get_memory(*offset).add(*n);
                }

                Instruction::Input { offset } => {
                    let mut buffer: [u8; 1] = [0];
                    let _ = io.read(&mut buffer);
                    machine.get_memory(*offset).set_byte(buffer[0]);
                }

                Instruction::Output { offset } => {
                    let buffer: [u8; 1] = [machine.get_memory(*offset).get_byte()];
                    let _ = io.write(&buffer);
                }

                Instruction::Open {
                    offset,
                    close_index,
                } => {
                    if machine.get_memory(*offset).is_zero() {
                        machine.pc = *close_index;
                    }

                    machine.pointer = machine.pointer.wrapping_add_signed(*offset);
                }

                Instruction::Close { offset, open_index } => {
                    if !machine.get_memory(*offset).is_zero() {
                        machine.pc = *open_index;
                    }

                    machine.pointer = machine.pointer.wrapping_add_signed(*offset);
                }
            }

            machine.pc += 1;
        }
    }
}
