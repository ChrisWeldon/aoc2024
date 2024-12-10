use std::fs::File;
use std::io::{BufReader, BufRead};
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


pub fn run(problem_num: i32, file_path: String) -> i32{
    let input = load_input(file_path);

    match problem_num {
        1 => problem_1(input),
        2 => problem_2(input),
        _ => panic!("{} does not exist!", problem_num)
    }
}

pub fn problem_1(input: Vec<Vec<u32>>) -> i32{
    let mut tot = 0 ;
    for v in input{
        if let None = unsafe_index(&v) {
            tot += 1;
        }
    }

    tot
}

pub fn problem_2(input: Vec<Vec<u32>>) -> i32{
    let mut tot = 0;
    for mut v in input{
        // there exists some unsafe value in the array
        if let Some(i) = unsafe_index(&v){
            for i in 0..v.len(){
                let mut save = v.clone();
                save.remove(i);
                if let None = unsafe_index(&save){
                    tot+=1;
                    break;
                }
            }
        }else{
            tot += 1;
        }
    };

    tot
}

// index of the first item in the vector that is unsafe
fn unsafe_index(input: &Vec<u32>) -> Option<usize>{
    
    if input.len() == 1{
        // Always safe
        return None
    }

    if input[0]==input[1] {
        return Some(1.try_into().unwrap());
    }

    let start_up = (input[1] as i32) - (input[0] as i32) > 0;
    for i in 1..input.len(){
        let dist = (input[i] as i32)-(input[i-1] as i32);
        let up = dist > 0;

        if i32::abs(dist)>3 || dist==0 || start_up != up{
            return Some(i);
        }

    }
    return None
}

fn load_input(filename: String) -> Vec<Vec<u32>>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);


    let mut vec : Vec<Vec<u32>> = vec![];

    for line in reader.lines(){
        let input = line.unwrap().to_string();
        let lexer = Lexer::build_lexer(&input);
        let row: Vec<u32> = lexer.collect();
        vec.push(row);
    }

    vec
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_lexer_input(){
        let input = "11 3 2 11 30 1".to_string();
        let mut lexer = Lexer::build_lexer(&input);

        assert_eq!(lexer.next(), Some(11));
        assert_eq!(lexer.next(), Some(3));
        assert_eq!(lexer.next(), Some(2));
        assert_eq!(lexer.next(), Some(11));
        assert_eq!(lexer.next(), Some(30));
        assert_eq!(lexer.next(), Some(1));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn test_is_safe(){
        assert_eq!(unsafe_index(&vec![11, 13, 14, 15, 17]), None);
        assert_eq!(unsafe_index(&vec![17, 14, 13, 11, 9]), None);
        assert_eq!(unsafe_index(&vec![9, 13, 14, 15, 17]), Some(1));
        assert_eq!(unsafe_index(&vec![12, 14, 14, 15, 17]), Some(2));
        assert_eq!(unsafe_index(&vec![12, 13, 14, 13, 12]), Some(3));
        assert_eq!(unsafe_index(&vec![16, 15, 14, 15, 16]), Some(3));
    }

}



