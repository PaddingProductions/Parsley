use std::io::{self, BufRead, Write};

mod ast;
mod parser;
mod interpreter;

use parser::*;
use interpreter::Environment;
use parser::operation::operation;

fn main () {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut env = Environment::new();

    println!("\x1b[1m   ===  [Parsley]  ===\x1b[0m");
    loop {
        print!("syn > ");
        stdout.flush().unwrap();

        let mut stream = String::with_capacity(100);
        loop {
            let mut buf = String::with_capacity(100);
            let ret = stdin.read_line(&mut buf).expect("STDIN failed");
            stream += buf.as_str();
            if ret == 0 {
                println!();
                break;
            }
            print!("    > ");
            stdout.flush().unwrap();
        }

        let mut s: &str = stream.trim();
        while !s.is_empty() {
            match operation().parse(s) {
                Ok((buf, op)) => {
                    s = buf;
                    if let Err(err) = op.exec(&mut env) {
                        println!("Interpreter failed with error: {:?}", err);
                    }
                },
                Err(e) => {
                    println!("Invalid Syntax While Parsing: '{}'\nError: {}", s, e);
                    break;
                }
            };
        }
    }
}
