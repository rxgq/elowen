use std::fs;
use std::env;

use lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("expected source file path as argument");
        return
    }

    if fs::metadata(&args[1]).is_err() {
        println!("unable to open source file");
        return
    }

    let source = fs::read_to_string(&args[1])
        .expect("error reading from source file.");

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    for token in tokens {
        println!("{:?}", token)
    }

}