use std::env;
pub mod lexer;

pub mod day_1;
pub mod day_2;


fn main(){
    let args: Vec<String> = env::args().collect();
    
    let day = &args[1].to_string();
    let day_num = day.parse::<i32>().unwrap_or_else(|error| {
        panic!("Invalid arg[1] type problem={error:?}. Expected integer")
    });

    let problem = &args[2].to_string();
    let problem_num = problem.parse::<i32>().unwrap_or_else(|error| {
        panic!("Invalid arg[2] type problem={error:?}. Expected integer")
    });

    let input_path = &args[3];


    let res: i32 = match day_num {
        1 => day_1::run(problem_num, input_path.to_string()),
        2 => day_2::run(problem_num, input_path.to_string()),
        _ => panic!("Sorry...day not implemented yet!!")
    };

    println!("result: {}", res);
}

