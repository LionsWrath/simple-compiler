#[path = "../src/lex.rs"] mod lex;
#[path = "../src/utils/utils.rs"] mod utils;

use std::path::PathBuf;

fn get_tokens(filename: &str) -> Vec<lex::Token> {
    let mut lexer = lex::Lexer::new(
        utils::read_file(&PathBuf::from(filename))
    );
    let mut token = lexer.get_token();
    let mut res: Vec<lex::Token> = Vec::new();

    while !matches!(token.kind, lex::TokenType::EOF) {
        res.push(token);
        token = lexer.get_token();
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings() {
        let res = get_tokens("src/examples/tests/test_1.txt");
        assert!(matches!(res[0].kind, lex::TokenType::STRING));
        assert!(matches!(res[1].kind, lex::TokenType::STRING));
        assert!(matches!(res[2].kind, lex::TokenType::STRING));
    }

    #[test]
    fn test_operators() {
        let res = get_tokens("src/examples/tests/test_2.txt");
        assert!(matches!(res[0].kind, lex::TokenType::PLUS));
        assert!(matches!(res[1].kind, lex::TokenType::GTEQ));
        assert!(matches!(res[2].kind, lex::TokenType::EQEQ));
        assert!(matches!(res[3].kind, lex::TokenType::EQ));
    }

    #[test]
    fn test_ident() {
        let res = get_tokens("src/examples/tests/test_3.txt");
        assert!(matches!(res[0].kind, lex::TokenType::GOTO));
        assert!(matches!(res[1].kind, lex::TokenType::IDENT));
        assert!(matches!(res[2].kind, lex::TokenType::IDENT));
        assert!(matches!(res[3].kind, lex::TokenType::IF));
    }
}
