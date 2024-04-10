use std::fmt;

const RADIX: u32 = 10;
type RawSource = Vec<char>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
	EOF = -1,
	NEWLINE = 0,
	NUMBER = 1,
	IDENT = 2,
	STRING = 3,
	// Keywords
	LABEL = 101,
	GOTO = 102,
	PRINT = 103,
	INPUT = 104,
	LET = 105,
	IF = 106,
	THEN = 107,
	ENDIF = 108,
	WHILE = 109,
	REPEAT = 110,
	ENDWHILE = 111,
	// Operators
	EQ = 201,
	PLUS = 202,
	MINUS = 203,
	ASTERISK = 204,
	SLASH = 205,
	EQEQ = 206,
	NOTEQ = 207,
	LT = 208,
	LTEQ = 209,
	GT = 210,
	GTEQ = 211,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub text: RawSource,
    pub kind: TokenType
}

impl Token {
    pub fn new(text: RawSource, kind: TokenType) -> Self {
        Token {
            text,
            kind
        }
    }

    pub fn check_keyword(text_token: &RawSource) -> Option<TokenType> {
        match text_token[..] {
            ['L', 'A', 'B','E', 'L'] => Some(TokenType::LABEL),
            ['G', 'O', 'T', 'O'] => Some(TokenType::GOTO),
            ['P', 'R', 'I', 'N', 'T'] => Some(TokenType::PRINT),
            ['I', 'N', 'P', 'U', 'T'] => Some(TokenType::INPUT),
            ['L', 'E', 'T'] => Some(TokenType::LET),
            ['I', 'F'] => Some(TokenType::IF),
            ['T', 'H', 'E', 'N'] => Some(TokenType::THEN),
            ['E', 'N', 'D', 'I', 'F'] => Some(TokenType::ENDIF),
            ['W', 'H', 'I', 'L', 'E'] => Some(TokenType::WHILE),
            ['R', 'E', 'P', 'E', 'A', 'T'] => Some(TokenType::REPEAT),
            ['E', 'N', 'D', 'W', 'H', 'I', 'L', 'E'] => Some(TokenType::ENDWHILE),
            _ => None
        }
    }

    pub fn get_text(&self) -> String {
        self.text.iter().collect::<String>()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.get_text())
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::EOF => write!(f, "EOF"),
            TokenType::NEWLINE => write!(f, "NEWLINE"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::IDENT => write!(f, "IDENT"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::LABEL => write!(f, "LABEL"),
            TokenType::GOTO => write!(f, "GOTO"),
            TokenType::PRINT => write!(f, "PRINT"),
            TokenType::INPUT => write!(f, "INPUT"),
            TokenType::LET => write!(f, "LET"),
            TokenType::IF => write!(f, "IF"),
            TokenType::THEN => write!(f, "THEN"),
            TokenType::ENDIF => write!(f, "ENDIF"),
            TokenType::WHILE => write!(f, "WHILE"),
            TokenType::REPEAT => write!(f, "REPEAT"),
            TokenType::ENDWHILE => write!(f, "ENDWHILE"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::ASTERISK => write!(f, "ASTERISK"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::EQEQ => write!(f, "EQEQ"),
            TokenType::NOTEQ => write!(f, "NOTEQ"),
            TokenType::LT => write!(f, "LT"),
            TokenType::LTEQ => write!(f, "LTEQ"),
            TokenType::GT => write!(f, "GT"),
            TokenType::GTEQ => write!(f, "GTEQ"),
        }
    }
}

pub struct Lexer {
    pub source: RawSource,
    pub cur_char: char,
    pub cur_pos: Option<usize>,
}

impl Lexer {

    pub fn new(source: RawSource) -> Self {
        let mut lexer = Lexer {
           source,
           cur_char: '\0',
           cur_pos: None,
        };

        lexer.next_char();
        lexer
    }

    pub fn next_char(&mut self) {
        match self.cur_pos {
            Some(x) =>  {
                if x + 1 >= self.source.len() {
                    self.cur_char = '\0';
                } else {
                    self.cur_char = self.source[x + 1];
                    self.cur_pos = Some(x + 1);
                }
            },
            None => {
                self.cur_pos = Some(0);
                self.cur_char = self.source[0];
            }
        }
    }

    pub fn peek(&self) -> char {
        match self.cur_pos {
            Some(x) => {
                if x + 1 >= self.source.len() {
                    return '\0'
                }
                return self.source[x + 1];
            },
            None => return '\0',
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }
    
    pub fn get_token(&mut self) -> Token {

        let token: Token;

        self.skip_whitespace();
        self.skip_comment();

        token = match self.cur_char {
            '+' => Token::new(vec![self.cur_char], TokenType::PLUS),
            '-' => Token::new(vec![self.cur_char], TokenType::MINUS),
            '*' => Token::new(vec![self.cur_char], TokenType::ASTERISK),
            '/' => Token::new(vec![self.cur_char], TokenType::SLASH),
            '>' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char;
                    self.next_char();

                    Token::new(vec![last_char, self.cur_char], TokenType::GTEQ)
                } else {
                    Token::new(vec![self.cur_char], TokenType::GT)
                }
            },
            '<' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char;
                    self.next_char();

                    Token::new(vec![last_char, self.cur_char], TokenType::LTEQ)
                } else {
                    Token::new(vec![self.cur_char], TokenType::LT)
                }
            },
            '!' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char;
                    self.next_char();

                    Token::new(vec![last_char, self.cur_char], TokenType::NOTEQ)
                } else {
                    panic!("[LEXER] ERROR: Expected !=, got !{}", self.peek())
                }
            },
            '=' => {
                if self.peek() == '=' {
                    let last_char = self.cur_char;
                    self.next_char();

                    Token::new(vec![last_char, self.cur_char], TokenType::EQEQ)
                } else {
                    Token::new(vec![self.cur_char], TokenType::EQ)
                }
            },
            '\"' => {
                self.next_char();
                let start_pos: usize = self.cur_pos.unwrap() as usize;

                while self.cur_char != '\"' {
                    if self.cur_char == '\r' 
                        || self.cur_char == '\n' 
                        || self.cur_char == '\t' 
                        || self.cur_char == '\\' 
                        || self.cur_char == '%' {
                        panic!("[LEXER] ERROR: Illegal character in string.")
                    }
                    self.next_char();
                }

                let token_text = self.source[start_pos..self.cur_pos.unwrap() as usize].to_vec();
                Token::new(token_text, TokenType::STRING)
            },
            '0'..='9' => {
                let start_pos: usize = self.cur_pos.unwrap();

                while self.peek().is_digit(RADIX) {
                    self.next_char();
                }

                if self.peek() == '.' {
                    self.next_char();

                    if !self.peek().is_digit(RADIX) {
                        panic!("[LEXER] ERROR: Illegal character in decimal.");
                    }

                    while self.peek().is_digit(RADIX) {
                        self.next_char();
                    }
                }

                let token_text = self.source[start_pos..=self.cur_pos.unwrap() as usize].to_vec();
                Token::new(token_text, TokenType::NUMBER)
            },
            'A'..='Z' | 'a'..='z' => {
                let start_pos: usize = self.cur_pos.unwrap() as usize;

                while self.peek().is_alphanumeric() {
                    self.next_char();
                }

                let token_text = self.source[start_pos..=self.cur_pos.unwrap() as usize].to_vec();
                match Token::check_keyword(&token_text) {
                    Some(keyword) => Token::new(token_text, keyword),
                    None => Token::new(token_text, TokenType::IDENT)
                }
            },
            '\n' => Token::new(vec![self.cur_char], TokenType::NEWLINE),
            '\0' => Token::new(vec![], TokenType::EOF),
            other => panic!("[LEXER] ERROR: Unknown token {other}")
        };

        self.next_char();
        token
    }
}
