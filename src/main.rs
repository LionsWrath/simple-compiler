mod lex;

use std::fs::File;
use std::io::Read;

use failure::Error;

fn main() -> Result<(), Error>{
    let mut f = File::open("src/examples/file.txt")?;
    let mut raw: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut raw);
    let source: Vec<char> = raw.iter().map(|b| *b as char).collect::<Vec<_>>();

    let mut lexer = lex::Lexer::new(source);

    while lexer.peek() != '\0' {
        println!("{}", lexer.cur_char);
        lexer.next_char();
    }

    return Ok(());
}
