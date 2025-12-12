use std::cmp::min;


fn count_neighbours(chars: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut res = 0;
    let min_x = if x > 0 {x - 1} else {0};
    let min_y = if y > 0 {y - 1} else {0};
    let max_x = chars.len();
    let max_y = chars[0].len();
    for check_x in min_x..(min(x + 2, max_x)) {
        for check_y in min_y..(min(y + 2, max_y)) {
            if check_x == x && check_y == y {
                continue
            }
            if chars[check_x][check_y] == '@' {
                res += 1;
            }
        }
    }
    res
}

fn remove_rolls(rolls: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let mut result: Vec<Vec<char>> = rolls.clone();
    let mut removed = 0;
    let max_x = rolls.len();
    let max_y = rolls[0].len();
    for x in 0..max_x {
        for y in 0..max_y {
            if rolls[x][y] != '@' {
                continue
            }
            if count_neighbours(&rolls, x, y) < 4 {
                removed += 1;
                result[x][y] = '.';
            }
        }
    }
        
    (result, removed)
}

fn solve1(input: &str) -> i64 {
    let chars: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let (_, result) = remove_rolls(&chars);
    result as i64
}

fn solve2(input: &str) -> i64 {
    let mut chars: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut result = 0;
    loop {
        let (remaining, removed) = remove_rolls(&chars);
        if removed == 0 {
            break
        }
        result += removed;
        chars = remaining;
    }
    result as i64
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
        let input = match get_test_and_store(2025, 4) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve1(&input), 13);
    }
    
    #[test]
    fn test_solve2() {
        let input = match get_test_and_store(2025, 4) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve2(&input), 43);
    }
}