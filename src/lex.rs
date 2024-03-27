pub struct Lexer {
    pub source: Vec<char>,
    pub cur_char: char,
    pub cur_pos: Option<usize>,
}

impl Lexer {

    pub fn new(source: Vec<char>) -> Self {
        let mut lexer = Lexer {
           source: source,
           cur_char: '\0',
           cur_pos: None,
        };

        lexer.next_char();
        lexer
    }

    pub fn next_char(&mut self) {
        match self.cur_pos {
            Some(x) =>  {
                self.cur_pos = Some(x + 1);
                if x + 1 >= self.source.len() {
                    self.cur_char = '\0';
                } else {
                    self.cur_char = self.source[x + 1];
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

    pub fn abort(&self, message: String) {}
    pub fn skip_whitespace(&self) {}
    pub fn skip_comment(&self) {}
    pub fn get_token(&self) {}
}
