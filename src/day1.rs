pub fn run(input: Vec<u32>) -> Option<u32> {
    for i in 0..input.len() {
        for j in i..input.len() {
            if input[j] + input[i] == 2020 {
                return Some(input[j] * input[i])
            }
        }
    }
    None
}

#[test]
fn test() {
    assert_eq!(Some(514579), run(vec![1721, 979, 366, 299, 675, 1456]))
}