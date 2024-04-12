use std::collections::HashSet;
use crate::lex;
use crate::emitter;

pub struct Parser {
    pub lexer: lex::Lexer,
    pub emitter: emitter::Emitter,
    pub cur_token: lex::Token,
    pub peek_token: lex::Token,
    pub symbols: HashSet<String>,
    pub labels_declared: HashSet<String>,
    pub labels_gotoed: HashSet<String>,
}

impl Parser {
    
    pub fn new(mut lexer: lex::Lexer, emitter: emitter::Emitter) -> Self {

        let cur_token = lexer.get_token();
        let peek_token = lexer.get_token();
        let symbols = HashSet::new();
        let labels_declared = HashSet::new();
        let labels_gotoed = HashSet::new();

        Parser{
            lexer,
            emitter,
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

    pub fn check_comparison_operator(&self) -> bool {
        match self.cur_token.kind {
            lex::TokenType::EQ | lex::TokenType::EQEQ 
                | lex::TokenType::LT | lex::TokenType::LTEQ 
                | lex::TokenType::GT | lex::TokenType::GTEQ 
                | lex::TokenType::NOTEQ => true,
            _ => false,
        }
    }

    pub fn emit(&mut self) {
        self.program();
        self.emitter.write_file();
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
        self.emitter.header_line("#include <stdio.h>");
        self.emitter.header_line("int main(void){");

        while self.check_token(lex::TokenType::NEWLINE) {
            self.next_token();
        }

        while !self.check_token(lex::TokenType::EOF) {
            self.statement();
        }

        self.emitter.emit_line("return 0;");
        self.emitter.emit_line("}");

        for label in self.labels_gotoed.iter() {
            if !self.labels_declared.contains(label) {
                panic!("[PARSER] Error: GOTO to undeclared label {label}")
            }
        }
    }

    pub fn statement(&mut self) {

        if self.check_token(lex::TokenType::PRINT) {
            self.next_token();

            if self.check_token(lex::TokenType::STRING) {
                
                self.emitter.emit("printf(\"");
                self.emitter.emit(self.cur_token.get_text().as_str());
                self.emitter.emit_line("\\n\");");

                self.next_token();
            } else {
                self.emitter.emit("printf(\"%.2f\\n\", (float)(");
                self.expression();
                self.emitter.emit_line("));");
            }

        } else if self.check_token(lex::TokenType::IF) {
            self.next_token();
            self.emitter.emit("if (");
            self.comparison();

            self.match_token(lex::TokenType::THEN); 
            self.nl();
            self.emitter.emit_line(") {");

            while !self.check_token(lex::TokenType::ENDIF) {
                self.statement()
            }

            self.match_token(lex::TokenType::ENDIF);
            self.emitter.emit_line("}");
        } else if self.check_token(lex::TokenType::WHILE) {
            self.next_token();
            self.emitter.emit("while (");
            self.comparison();

            self.match_token(lex::TokenType::REPEAT);
            self.nl();
            self.emitter.emit_line(") {");

            while !self.check_token(lex::TokenType::ENDWHILE) {
                self.statement();
            }

            self.match_token(lex::TokenType::ENDWHILE);
            self.emitter.emit_line("}");
        } else if self.check_token(lex::TokenType::LABEL) {
            self.next_token();

            let token_text = self.cur_token.get_text();
            if self.labels_declared.contains(&token_text) {
                panic!("[PARSER] Error: Label {token_text} already exists");
            }

            self.labels_declared.insert(token_text);
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.emitter.emit_line(":");
            self.match_token(lex::TokenType::IDENT);
        } else if self.check_token(lex::TokenType::GOTO) {
            self.next_token();
            self.labels_gotoed.insert(self.cur_token.get_text());
            self.emitter.emit("goto");
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.emitter.emit_line(";");
            self.match_token(lex::TokenType::GOTO);
        } else if self.check_token(lex::TokenType::LET) {
            self.next_token();

            let token_text = self.cur_token.get_text();
            if !self.symbols.contains(&token_text) {
                self.symbols.insert(token_text);
                self.emitter.header("float ");
                self.emitter.header(self.cur_token.get_text().as_str());
                self.emitter.header_line(";");
            }

            self.emitter.emit(self.cur_token.get_text().as_str());
            self.emitter.emit(" = ");
            self.match_token(lex::TokenType::IDENT);
            self.match_token(lex::TokenType::EQ);
            self.expression();
            self.emitter.emit_line(";");
        } else if self.check_token(lex::TokenType::INPUT) {
            self.next_token();

            let token_text = self.cur_token.get_text();
            if !self.symbols.contains(&token_text) {
                self.symbols.insert(token_text);
                self.emitter.header("float ");
                self.emitter.header(self.cur_token.get_text().as_str());
                self.emitter.header_line(";");
            }

            self.emitter.emit("if(0 == scanf(\"%f\", &");
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.emitter.emit_line(")) {");
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.emitter.emit_line(" = 0;");
            self.emitter.emit_line("scanf(\"%*s\");");
            self.emitter.emit_line("}");
            self.match_token(lex::TokenType::IDENT);
        } else {
            panic!("[PARSER] Error: Token not valid!");
        }

        self.nl();
    }

    pub fn nl(&mut self) {
        self.match_token(lex::TokenType::NEWLINE);
        while self.check_token(lex::TokenType::NEWLINE) {
            self.next_token();
        }
    }

    pub fn expression(&mut self) {
        self.term();

        while self.check_token(lex::TokenType::PLUS) || self.check_token(lex::TokenType::MINUS) {
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
            self.term();
        }
    }

    pub fn comparison(&mut self) {
        self.expression();

        if self.check_comparison_operator() {
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
            self.expression();
        } else {
            let current = self.cur_token.kind.to_string();
            panic!("[PARSER] Error: Expected comparison operator at: {current}");
        }

        while self.check_comparison_operator() {
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
            self.expression();
        }
    }

    pub fn term(&mut self) {
        self.unary();

        while self.check_token(lex::TokenType::ASTERISK) || self.check_token(lex::TokenType::SLASH) {
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
            self.unary();
        }
    }

    pub fn unary(&mut self) {
        if self.check_token(lex::TokenType::PLUS) || self.check_token(lex::TokenType::MINUS) {
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
        }

        self.primary();
    }

    pub fn primary(&mut self) {
        if self.check_token(lex::TokenType::NUMBER) {
            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
        } else if self.check_token(lex::TokenType::IDENT) {

            let token_text = self.cur_token.get_text();
            if !self.symbols.contains(&token_text) {
                panic!("[PARSER] Error: Refencing variable {token_text} before assignment");
            }

            self.emitter.emit(self.cur_token.get_text().as_str());
            self.next_token();
        } else {
            let current = self.cur_token.kind.to_string();
            panic!("[PARSER] Error: Unexpected token at {current}");
        }
    }
}
