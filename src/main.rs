use lex::Lexer;
use std::fs::File;
use std::io::Read;
use std::env;

fn main() {
    let mut f = File::open("examples/file.txt")?;
    let mut source: Vec<u8> = Vec::new();

    f.read_to_end(&mut source);

    let mut lexer = new Lexer(&source);

    while lexer.peek() != '\0' {
        println!(lexer.curChar);
        lexer.nextChar();
    }
}