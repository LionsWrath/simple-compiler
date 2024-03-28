mod lex;
#[path = "utils/utils.rs"] mod utils;

fn main() {
    let mut lexer = lex::Lexer::new(utils::read_file("src/examples/file2.txt"));
    let mut token = lexer.get_token();

    while !matches!(token.kind, lex::TokenType::EOF) {
        println!("{:?} POS: {:?}", token, lexer.cur_pos);
        token = lexer.get_token();
    }
}
