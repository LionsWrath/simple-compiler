#[cfg(test)]
mod tests {
    use super::lex::Lexer;

    #[test]
    fn test_lex() {

        let mut lexer = new Lexer(&source);

        while lexer.peek() != '\0' {
            println!(lexer.curChar);
            lexer.nextChar();
        }

        assert_eq!(add(1, 2), 3);
    }
}