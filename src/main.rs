pub mod day_1;

fn main() {
    let sol_1 = day_1::problem_1("src/day_1_input.txt".to_string());
    let sol_2 = day_1::problem_2("src/day_1_input.txt".to_string());

    println!("{:?}", sol_1);
    println!("{:?}", sol_2);
}

