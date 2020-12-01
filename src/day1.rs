pub fn t1(input: Vec<u32>) -> Option<u32> {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[j] + input[i] == 2020 {
                return Some(input[j] * input[i]);
            }
        }
    }
    None
}

pub fn t2(input: Vec<u32>) -> Option<u32> {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for k in (j + 1)..input.len() {
                if input[k] + input[j] + input[i] == 2020 {
                    return Some(input[j] * input[i] * input[k]);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::*;

    #[test]
    fn test_t1() {
        assert_eq!(Some(514579), t1(vec![1721, 979, 366, 299, 675, 1456]))
    }

    #[test]
    fn test_t1_answer() {
        assert_eq!(Some(157059), t1(read_input_vec("inputs/day1.txt")))
    }

    #[test]
    fn test_t2() {
        assert_eq!(Some(241861950), t2(vec![1721, 979, 366, 299, 675, 1456]))
    }

    #[test]
    fn test_t2_answer() {
        assert_eq!(Some(165080960), t2(read_input_vec("inputs/day1.txt")))
    }
}
