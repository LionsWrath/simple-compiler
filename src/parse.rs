//mod lex;
use crate::lex;

pub struct Parser {
    pub lexer: lex::Lexer,
    pub cur_token: lex::Token,
    pub peek_token: lex::Token
}

impl Parser {
    
    pub fn new(mut lexer: lex::Lexer) -> Self {

        let cur_token = lexer.get_token();
        let peek_token = lexer.get_token();

        Parser{
            lexer,
            cur_token,
            peek_token
        }
    }

    fn check_token(&self, kind: lex::TokenType) -> bool {
        self.cur_token.kind == kind
    }

    pub fn check_peek(&self, kind: lex::TokenType) -> bool {
        self.peek_token.kind == kind
    }

    pub fn match_token(&mut self, kind: lex::TokenType) {
        if !self.check_token(kind) {

            let current = self.cur_token.kind.to_string();
            let expected = kind.clone().to_string();

            panic!("[PARSER] ERROR: Expecting token of type {expected}, got {current}")
        }

        self.next_token()
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    pub fn program(&mut self) {
        println!("PROGRAM")
    }

    pub fn statement(&mut self) {

        if self.check_token(lex::TokenType::PRINT) {
            self.next_token();

            if self.check_token(lex::TokenType::STRING) {
                self.next_token();
            } else {
                self.expression();
            }

        }

        self.nl()
    }

    pub fn nl(&mut self) {
        println!("NEWLINE");

        self.match_token(lex::TokenType::NEWLINE);
        while self.check_token(lex::TokenType::NEWLINE) {
            self.next_token();
        }
    }

    pub fn expression(&self) {
        println!("EXPRESSION");
    }
}
