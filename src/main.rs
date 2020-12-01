mod day1;
mod util;

fn main() {
    match day1::t1(util::read_input_vec("inputs/day1.txt")) {
        Some(answer) => println!("D1T1: {}", answer),
        None => println!("D1T1: No answer found")
    };

    match day1::t2(util::read_input_vec("inputs/day1.txt")) {
        Some(answer) => println!("D1T2: {}", answer),
        None => println!("D1T2: No answer found")
    };
}
