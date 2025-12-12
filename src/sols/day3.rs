fn solve1(input: &str) -> i64 {
    input.lines().map(|s| find_max_joltage(2, s)).sum()
}

fn solve2(input: &str) -> i64 {
    input.lines().map(|s| find_max_joltage(12, s)).sum()
}

fn find_max_joltage(length: usize, input: &str) -> i64 {
    if length == 0 {
        return 0
    }
    let mut max: (usize, u32) = (0, 0);
    let chars: Vec<char> = input.chars().collect();
    
    for n in 0..(input.len() - length + 1) {
        let dig = chars[n].to_digit(10).expect("Unable to convert to digit!");
        if dig > max.1 {
            max = (n, dig);
        }
    }
    (max.1 as i64) * (10i64.pow(length as u32 - 1)) + find_max_joltage(length - 1, &input[max.0+1..])
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_lib::get_test_and_store;
    
    #[test]
    fn test_solve1() {
        let input = match get_test_and_store(2025, 3) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get puzzle input!"),
        };
        assert_eq!(solve1(&input), 357);
    }
    
    #[test]
    fn test_solve2() {
        let input = match get_test_and_store(2025, 3) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get puzzle input!"),
        };
        assert_eq!(solve2(&input), 3121910778619);
    }
}
