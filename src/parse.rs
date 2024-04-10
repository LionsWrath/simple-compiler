use std::collections::HashSet;
use crate::lex;

pub struct Parser {
    pub lexer: lex::Lexer,
    pub cur_token: lex::Token,
    pub peek_token: lex::Token,
    pub symbols: HashSet<String>,
    pub labels_declared: HashSet<String>,
    pub labels_gotoed: HashSet<String>,
}

impl Parser {
    
    pub fn new(mut lexer: lex::Lexer) -> Self {

        let cur_token = lexer.get_token();
        let peek_token = lexer.get_token();
        let symbols = HashSet::new();
        let labels_declared = HashSet::new();
        let labels_gotoed = HashSet::new();

        Parser{
            lexer,
            cur_token,
            peek_token,
            symbols,
            labels_declared,
            labels_gotoed,
        }
    }

    pub fn check_token(&self, kind: lex::TokenType) -> bool {
        self.cur_token.kind == kind
    }

    pub fn check_peek(&self, kind: lex::TokenType) -> bool {
        self.peek_token.kind == kind
    }

    pub fn check_comparison_operator(&self) -> bool {
        match self.cur_token.kind {
            lex::TokenType::EQ | lex::TokenType::EQEQ 
                | lex::TokenType::LT | lex::TokenType::LTEQ 
                | lex::TokenType::GT | lex::TokenType::GTEQ 
                | lex::TokenType::NOTEQ => true,
            _ => false,
        }
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
        println!("PROGRAM");

        while self.check_token(lex::TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(lex::TokenType::EOF) {
            self.statement();
        }

        for label in self.labels_gotoed.iter() {
            if !self.labels_declared.contains(label) {
                panic!("[PARSER] Error: GOTO to undeclared label {label}")
            }
        }
    }

    pub fn statement(&mut self) {

        if self.check_token(lex::TokenType::PRINT) {
            println!("PRINT");
            self.next_token();

            if self.check_token(lex::TokenType::STRING) {
                self.next_token();
            } else {
                self.expression();
            }

        } else if self.check_token(lex::TokenType::IF) {
            println!("IF");
            self.next_token();
            self.comparison();

            self.match_token(lex::TokenType::THEN); 
            self.nl();

            while !self.check_token(lex::TokenType::ENDIF) {
                self.statement()
            }

            self.match_token(lex::TokenType::ENDIF)
        } else if self.check_token(lex::TokenType::WHILE) {
            println!("WHILE");
            self.next_token();
            self.comparison();

            self.match_token(lex::TokenType::REPEAT);
            self.nl();

            while !self.check_token(lex::TokenType::ENDWHILE) {
                self.statement();
            }

            self.match_token(lex::TokenType::ENDWHILE);
        } else if self.check_token(lex::TokenType::LABEL) {
            println!("LABEL");
            self.next_token();

            let token_text = self.cur_token.get_text();
            if self.labels_declared.contains(&token_text) {
                panic!("[PARSER] Error: Label {token_text} already exists");
            }

            self.labels_declared.insert(token_text);
            self.match_token(lex::TokenType::IDENT);
        } else if self.check_token(lex::TokenType::GOTO) {
            println!("GOTO");
            self.next_token();
            self.labels_declared.insert(self.cur_token.get_text());
            self.match_token(lex::TokenType::GOTO);
        } else if self.check_token(lex::TokenType::LET) {
            println!("LET");
            self.next_token();

            let token_text = self.cur_token.get_text();
            if !self.symbols.contains(&token_text) {
                self.symbols.insert(token_text);
            }

            self.match_token(lex::TokenType::IDENT);
            self.match_token(lex::TokenType::EQ);
            self.expression();
        } else if self.check_token(lex::TokenType::INPUT) {
            println!("INPUT");
            self.next_token();

            let token_text = self.cur_token.get_text();
            if !self.symbols.contains(&token_text) {
                self.symbols.insert(token_text);
            }

            self.match_token(lex::TokenType::IDENT);
        } else {
            panic!("[PARSER] Error: Token not valid!");
        }

        self.nl();
    }

    pub fn nl(&mut self) {
        println!("NEWLINE");

        self.match_token(lex::TokenType::NEWLINE);
        while self.check_token(lex::TokenType::NEWLINE) {
            self.next_token();
        }
    }

    pub fn expression(&mut self) {
        println!("EXPRESSION");

        self.term();

        while self.check_token(lex::TokenType::PLUS) || self.check_token(lex::TokenType::MINUS) {
            self.next_token();
            self.term();
        }
    }

    pub fn comparison(&mut self) {
        println!("COMPARISON");

        self.expression();

        if self.check_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            let current = self.cur_token.kind.to_string();
            panic!("[PARSER] Error: Expected comparison operator at: {current}");
        }
    }

    pub fn term(&mut self) {
        println!("TERM");

        self.unary();

        while self.check_token(lex::TokenType::ASTERISK) || self.check_token(lex::TokenType::SLASH) {
            self.next_token();
            self.unary();
        }
    }

    pub fn unary(&mut self) {
        println!("UNARY");

        if self.check_token(lex::TokenType::PLUS) || self.check_token(lex::TokenType::MINUS) {
            self.next_token();
        }

        self.primary();
    }

    pub fn primary(&mut self) {
        println!("PRIMARY ({})", self.cur_token.to_string());

        if self.check_token(lex::TokenType::NUMBER) {
            self.next_token();
        } else if self.check_token(lex::TokenType::IDENT) {

            let token_text = self.cur_token.get_text();
            if !self.symbols.contains(&token_text) {
                panic!("[PARSER] Error: Refencing variable {token_text} before assignment");
            }

            self.next_token();
        } else {
            let current = self.cur_token.kind.to_string();
            panic!("[PARSER] Error: Unexpected token at {current}");
        }
    }
}
