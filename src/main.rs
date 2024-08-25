mod lexer;
mod parser;
mod ast;
mod analyzer;
mod codegen;

use std::fs;
use crate::lexer::{Lexer, Token};

fn main() {
    println!("Welcome to Raptor Compiler!");
    
    let input = fs::read_to_string("examples/hello_world.rap")
        .expect("Failed to read the input file");

    let mut lexer = Lexer::new(&input);
    
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}