#[path = "../src/lex.rs"] mod lex;
#[path = "../src/parse.rs"] mod parse;
#[path = "../src/emitter.rs"] mod emitter;
#[path = "../src/utils/utils.rs"] mod utils;

use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {

        let input = "src/examples/tests/test_5.txt";

        let mut parser = parse::Parser::new(
            lex::Lexer::new(utils::read_file(&PathBuf::from(input))),
            emitter::Emitter::new(PathBuf::from("out.c")),
        );

        parser.program();

        assert!(true);
    }
}
