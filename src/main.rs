use std::io::{ self, BufRead };

mod tokens;
mod ast;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main () {
    let mut buf = String::with_capacity(128);
    let mut stdin = io::stdin().lock();

    loop {
        stdin.read_line(&mut buf).expect("STDIO failed");
        
        let tokens = Lexer::from_str(buf.to_string()).lex().expect("Lexing failed");
        println!("Tokens: {:?}", tokens);
        println!("=== Lexer Done ===\n");

        let instructions = Parser::from(tokens).parse();
        println!("=== Parser Done ===\n");

        let mut interpreter = Interpreter::new();
        interpreter.interpret(instructions);
        println!("=== Interpreter Done ===\n");
    }
}
