use num::{FromPrimitive, ToPrimitive};
use std::{env, fs, io::Read, num::Wrapping, time::Instant};
use brainfuck::{Machine, Program};

type T = Wrapping<u16>;

fn output(value: T) {
    print!("{}", char::from_u32(T::to_u32(&value).unwrap() & 0xFF).unwrap());
}

fn input() -> T {
    T::from_u8(std::io::stdin().bytes().next().unwrap().unwrap()).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(path) = &args.get(1) else {
        eprintln!("no file specified");
        return;
    };

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
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