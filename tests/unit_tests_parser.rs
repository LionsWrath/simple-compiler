#[path = "../src/lex.rs"] mod lex;
#[path = "../src/parse.rs"] mod parse;
#[path = "../src/utils/utils.rs"] mod utils;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut parser = parse::Parser::new(
            lex::Lexer::new(utils::read_file("src/examples/tests/test_5.txt"))
        );

        parser.program();

        assert!(true);
    }

    #[test]
    fn test_equation() {
        let mut parser = parse::Parser::new(
            lex::Lexer::new(utils::read_file("src/examples/tests/test_6.txt"))
        );

        parser.program();

        assert!(true);
    }

    #[test]
    fn test_loop() {
        let mut parser = parse::Parser::new(
            lex::Lexer::new(utils::read_file("src/examples/tests/test_7.txt"))
        );

        parser.program();

        assert!(true);
    }

    #[test]
    fn test_validity() {
        let mut parser = parse::Parser::new(
            lex::Lexer::new(utils::read_file("src/examples/tests/test_8.txt"))
        );

        parser.program();

        assert!(true);
    }
}
