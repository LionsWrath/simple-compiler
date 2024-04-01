//mod lex;
use crate::lex;

pub struct Parser {
    pub lexer: lex::Lexer,
    pub cur_token: lex::Token,
    pub peek_token: lex::Token
}

impl Parser {
    
    pub fn new(lexer: lex::Lexer) -> Self {

        let cur_token = lexer.get_token();
        let peek_token = lexer.get_token();

        Parser{
            lexer,
            cur_token,
            peek_token
        }
    }

    fn check_token(&self, kind: lex::TokenType) -> bool {
        matches!(self.cur_token.kind, kind)
    }

    pub fn check_peek(&self, kind: lex::TokenType) -> bool {
        matches!(self.peek_token, kind) 
        
    }

    pub fn match_token(&mut self, kind: lex::TokenType) {
        if !self.check_token(kind) {

            let current = self.cur_token.kind.to_string();
            let expected = kind.to_string();

            panic!("[PARSER] ERROR: Expecting token of type {expected}, got {current}")
        }

        self.next_token()
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.lexer.get_token();
    }

    pub fn program(&self) {

    }
}
