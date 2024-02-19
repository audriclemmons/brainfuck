use std::{
    env, fs,
    io::{self, Read, Write},
};

mod machine;
mod program;
mod value;

use crate::{machine::Machine, program::Program};

struct ReadWrite<'a, R: Read, W: Write> {
    reader: &'a mut R,
    writer: &'a mut W,
}

impl<'a, R: Read, W: Write> ReadWrite<'a, R, W> {
    fn new(reader: &'a mut R, writer: &'a mut W) -> ReadWrite<'a, R, W> {
        ReadWrite { reader, writer }
    }
}

impl<R: Read, W: Write> Read for ReadWrite<'_, R, W> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<R: Read, W: Write> Write for ReadWrite<'_, R, W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
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

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    Machine::<u32>::execute(&program, &mut ReadWrite::new(&mut stdin, &mut stdout));
}
