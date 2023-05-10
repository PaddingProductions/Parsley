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

    loop {
        print!("syn > ");
        stdout.flush().unwrap();

        let mut buf = String::with_capacity(100);
        stdin.read_line(&mut buf).expect("STDIN failed");

        let s: &str = buf.as_str();
        match operation().parse(s) {
            Ok((_, op)) => op.exec(&mut env),
            Err(e)      => println!("Invalid Syntax: Error: {}", e)
        };
    }
}
