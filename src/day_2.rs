use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

fn load_input(filename: String){
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines(){
        let str = line.unwrap();
        let caps = re.captures(&str).unwrap();
        let l = caps.get(1).unwrap().as_str();
        let r = caps.get(2).unwrap().as_str();

        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
        
    }

}

fn parse_input(input: String){

}
