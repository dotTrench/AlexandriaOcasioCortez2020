pub fn read_input_vec(path: &str) -> Vec<u32> {
    std::fs::read_to_string(path)
        .expect("Could not open file")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect()
}
