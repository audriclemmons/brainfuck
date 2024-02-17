use std::{env, fs, io::{self, Read}};

mod machine;
mod program;
mod value;

use crate::{machine::Machine, program::Program};

type T = u32;

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
    
    let mut machine: Machine<T> = Machine::new(std::io::stdin(), std::io::stdout());
    machine.execute(program);
}