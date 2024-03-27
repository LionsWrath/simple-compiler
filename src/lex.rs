use std::fmt;

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

pub struct Token {
    pub text: Vec<char>,
    pub kind: TokenType
}

impl Token {
    pub fn new(text: Vec<char>, kind: TokenType) -> Self {
        Token {
            text,
            kind
        }
    }
}

impl fmt::Debug for Token {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.text)
            .field(&self.kind)
            .finish()
    }
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::EOF => write!(f, "eof"),
            TokenType::NEWLINE => write!(f, "newline"),
            TokenType::NUMBER => write!(f, "number"),
            TokenType::IDENT => write!(f, "ident"),
            TokenType::STRING => write!(f, "string"),
            TokenType::LABEL => write!(f, "label"),
            TokenType::GOTO => write!(f, "goto"),
            TokenType::PRINT => write!(f, "print"),
            TokenType::INPUT => write!(f, "input"),
            TokenType::LET => write!(f, "let"),
            TokenType::IF => write!(f, "if"),
            TokenType::THEN => write!(f, "then"),
            TokenType::ENDIF => write!(f, "endif"),
            TokenType::WHILE => write!(f, "while"),
            TokenType::REPEAT => write!(f, "repeat"),
            TokenType::ENDWHILE => write!(f, "endwhile"),
            TokenType::EQ => write!(f, "eq"),
            TokenType::PLUS => write!(f, "plus"),
            TokenType::MINUS => write!(f, "minus"),
            TokenType::ASTERISK => write!(f, "asterisk"),
            TokenType::SLASH => write!(f, "slash"),
            TokenType::EQEQ => write!(f, "eqeq"),
            TokenType::NOTEQ => write!(f, "noteq"),
            TokenType::LT => write!(f, "lt"),
            TokenType::LTEQ => write!(f, "lteq"),
            TokenType::GT => write!(f, "gt"),
            TokenType::GTEQ => write!(f, "gteq"),
        }
    }
}

pub struct Lexer {
    pub source: Vec<char>,
    pub cur_char: char,
    pub cur_pos: Option<usize>,
}

impl Lexer {

    pub fn new(source: Vec<char>) -> Self {
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

    pub fn abort(&self, message: String) {
        panic!("[LEXER ERROR] {message}");
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
            '\n' => Token::new(vec![self.cur_char], TokenType::NEWLINE),
            '\0' => Token::new(vec![], TokenType::EOF),
            other => panic!("[LEXER] ERROR: Unknown token {other}")
        };

        self.next_char();
        token
    }
}
