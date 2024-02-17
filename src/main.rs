use num::{FromPrimitive, ToPrimitive};
use std::{env, fs, io::{self, Read}, num::Wrapping, time::Instant};
use brainfuck::{Machine, Program};

type T = Wrapping<u32>;

fn output(value: T) {
    print!("{}", char::from_u32(T::to_u32(&value).unwrap() & 0xFF).unwrap());
}

fn input() -> T {
    T::from_u8(std::io::stdin().bytes().next().unwrap().unwrap()).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let source = if let Some(path) = &args.get(1) {
        match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    } else {
        let mut bytes = Vec::new();
        let _ = io::stdin().read_to_end(&mut bytes);

        match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
    };

    let program = match Program::compile(&source) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut machine: Machine<T> = Machine::new();
    machine.bind_io(input, output);

    let now = Instant::now();
    machine.execute(program);
    println!("Finished in {:.2?}.", now.elapsed());
}