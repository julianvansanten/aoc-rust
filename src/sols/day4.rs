fn solve1(input: &str) -> i64 {
    let chars: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let max_x = chars.len();
    let max_y = chars[0].len();
    for x in 0..max_x {
        for y in 0..max_y {
            
        }
    }
    0
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve1(input), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_lib::get_test_and_store;
    
    #[test]
    fn test_solve1() {
        let input = match get_test_and_store(2025, 4) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve1(&input), 13);
    }
}