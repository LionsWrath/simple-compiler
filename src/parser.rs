struct Lexer {
    source: String,
    curChar: char,
    curPos: Option<usize>,
};

impl Lexer {

    pub fn new(source: &Vec<char>) -> Self {
        let lexer = Lexer {
           source: Vec<char>,
           curChar: '',
           curPos: None,
        };

        lexer.nextChar();
        lexer
    }

    pub fn nextChar(&mut self) {
        match self.curPos {
            Some(x) =>  {
                self.curPos = Some(x + 1);
                if x + 1 >= self.source.len() {
                    self.curChar = '\0';
                } else {
                    self.curChar = self.source[self.curPos];
                }
            },
            None => {
                self.curPos = Some(0);
                self.curChar = self.source[0];
            }
        }
    }

    pub fn peek(&self) {}
    pub fn abort(&self, message: String) {}
    pub fn skipWhitespace(&self) {}
    pub fn skipComment(&self) {}
    pub fn getToken(&self) {}
}