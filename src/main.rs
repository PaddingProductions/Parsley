mod tokens;
mod ast;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main () {
    let str = "1 + 2 * 5 + 4 / 3";

    let tokens = Lexer::from_str(str.to_string()).lex().expect("Lexing failed");
    println!("=== Lexer Done, Result: {:?} ===", tokens);
    let instructions = Parser::from(tokens).parse();
    println!("=== Parser Done ===");
    let mut interpreter = Interpreter::new();
    interpreter.interpret(instructions);
    println!("=== Interpreter Done ===");
}
