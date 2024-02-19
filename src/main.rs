use std::{
    env, fs,
    io::{self, Read, Write},
    net::TcpListener,
};

use std::{thread, time};

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
        panic!();
    };

    let program = match Program::compile(&source) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        Machine::<u32>::execute(&program, &mut stream);
        stream.flush().unwrap();
    }
}
