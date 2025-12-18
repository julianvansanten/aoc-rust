fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

fn solve1(input: &str) -> i64 {
    let matrix: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.trim().split_ascii_whitespace().collect())
        .collect();
    let matrix = transpose(matrix);
    0
}

pub fn solve(input: &str) -> (i64, i64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_lib::get_test_and_store;

    #[test]
    fn test_solve1() {
        let input = match get_test_and_store(2025, 6) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve1(&input), 4277556);
    }
}
