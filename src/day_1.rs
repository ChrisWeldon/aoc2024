use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;

pub fn problem_1(filename: String) -> i32{
    let (mut left, mut right) = load_input(filename);
    calc_total_difference(&mut left, &mut right)
}

pub fn problem_2(filename: String) -> i32{
    let (mut left, mut right) = load_input(filename);
    calc_similarity_score(&mut left, &mut right)
}

pub fn calc_total_difference(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32{
    let mut tot = 0;
    left.sort(); // nlogn
    right.sort();
    for i in 0..left.len(){ //n
        tot += i32::abs(left[i]-right[i]);
    }
    tot
}

pub fn calc_similarity_score(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32{
    let mut tot = 0;
    let mut mem: HashMap<i32, i32> = HashMap::new();
    for l in 0..left.len(){
        let mut ct = 0;
        
        if let Some(i) = mem.get(&left[l]){
            tot+=i;
            continue;
        }

        for r in 0..right.len(){
            if left[l] == right[r]{
                ct += 1;
            }
        }


        mem.insert(left[l], ct*left[l]);
        tot += ct*left[l];
    }

    tot
}

pub fn load_input(filename: String) -> (Vec<i32>, Vec<i32>){
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

    (left, right)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_basic_arrays(){
        let mut l: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut r: Vec<i32> = vec![1, 2, 3, 4, 5];

        assert_eq!(calc_total_difference(&mut l, &mut r), 0);
    }

    #[test]
    fn test_basic_arrays_jumbled(){
        let mut l: Vec<i32> = vec![1, 2, 3, 4, 5];
        let mut r: Vec<i32> = vec![1, 5, 4, 3, 2];

        assert_eq!(calc_total_difference(&mut l, &mut r), 0);
    }

    #[test]
    fn test_simple_real_case(){
        let mut l: Vec<i32> = vec![1, 5, 4, 3];
        let mut r: Vec<i32> = vec![1, 2, 7, 6];

        assert_eq!(calc_total_difference(&mut l, &mut r), 5);
    }

    // Problem 2

    #[test]
    fn test_example_given(){
        let mut l: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let mut r: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(calc_similarity_score(&mut l, &mut r), 31);
       
    }
}

