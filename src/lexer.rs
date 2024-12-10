use std::num::ParseIntError;

pub struct Lexer<'a>{
    input: &'a String,
    position: usize,
    read_position: usize,
    pub ch: Option<char>
}


impl Lexer<'_> {

    #[allow(dead_code)]
    pub fn build_lexer(input: & String) -> Lexer{
        // Lexer is initiated, then reads in first character 

        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: Some(' ')
        };
        l.read_char();
        l
    }

    // 12345
    // *+
    pub fn read_char(&mut self){
        // should maintain the len of the string somewhere cause this is O(n)
        if self.read_position > self.input.len() { 
            self.ch = Some(' ')
        }else{
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> Option<char>{
        return self.input.chars().nth(self.read_position);
    }

    pub fn curr_char(&self) -> Option<char>{
        return self.input.chars().nth(self.position);
    }

    pub fn read_integer(&mut self) -> Result<u32, ParseIntError>{
        // consume all numeric characters in a row
        let position = self.position;
        while self.curr_char().is_some() && self.curr_char().unwrap().is_numeric() {
            self.read_char();
        }
        self.input[position .. self.position].parse::<u32>()
    }
    
    fn on_whitespace(& self) -> bool{
        // Is the position currently on a whitespace
        match self.ch{
            Some(c)=> {
                c == ' ' || c == '\t' || c == '\r' || c == '\n' 
            },
            _ => { false }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.on_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<u32, ParseIntError>{
        self.skip_whitespace();
        let tok = self.read_integer();
        tok
    }

}

impl Iterator for Lexer<'_> {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item>{
        match self.next_token() {
            Ok(number) => Some(number),
            Err(_e) => None
        }
    }
}
