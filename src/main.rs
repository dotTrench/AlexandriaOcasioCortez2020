mod day1;

fn main() {
    match day1::run(read_input_vec("inputs/day1.txt")) {
        Some(answer) => println!("Day1: {}", answer),
        None => println!("Day1: No answer found")
    };
}

fn read_input_vec(path: &str) -> Vec<u32> {
    std::fs::read_to_string(path)
        .expect("Could not open file")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}
