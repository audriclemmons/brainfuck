use std::{
    env, fs,
    io::{self, Read, Write},
};

mod machine;
mod program;
mod value;

use crate::{machine::Machine, program::Program};

struct ReadWrite<R: Read, W: Write> {
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Read for ReadWrite<R, W> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read, W: Write> Write for ReadWrite<R, W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

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

    let mut readwrite = ReadWrite {
        reader: std::io::stdin(),
        writer: std::io::stdout(),
    };
    
    Machine::<T>::execute(&program, &mut readwrite);
}
