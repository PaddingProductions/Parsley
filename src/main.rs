use std::io::{ self, BufRead, Write };

mod tokens;
mod ast;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main () {
    let mut stdout = io::stdout().lock();
    let mut stdin = io::stdin().lock();
    let mut interpreter = Interpreter::new();

    loop {
        print!("syn > ");
        stdout.flush().unwrap();
       
        let mut buf = String::with_capacity(128);
        stdin.read_line(&mut buf).expect("STDIO failed");
        
        let tokens = Lexer::from_str(buf).lex().expect("Lexing failed");
        println!("Tokens: {:?}", tokens);
        println!("=== Lexer Done ===\n");

        let instructions = Parser::from(tokens).parse();
        println!("=== Parser Done ===\n");

        interpreter.interpret(instructions);
        println!("=== Interpreter Done ===\n");
    }
}
