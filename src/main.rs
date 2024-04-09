mod lex;
mod parse;
#[path = "utils/utils.rs"] mod utils;

fn main() {
    let mut parser = parse::Parser::new(
        lex::Lexer::new(utils::read_file("src/examples/tests/test_6.txt"))
    );

    parser.program()
}
