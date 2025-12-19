use std::process::exit;
use crate::aoc_lib::helpers::transpose;

fn build_str_matrix_transpose(input: &str) -> Vec<Vec<&str>> {
    let matrix: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.trim().split_ascii_whitespace().collect())
        .collect();
    transpose(matrix)
}

fn solve1(input: &str) -> i64 {
    let matrix = build_str_matrix_transpose(input);
    let mut result = 0;
    for entry in matrix {
        match entry[entry.len() - 1] {
            "+" => {
                result += entry[0..entry.len() - 1].iter().map(|s| s.parse::<i64>().unwrap_or(0)).sum::<i64>();
            },
            "*" => {
                result += entry[0..entry.len() - 1].iter().map(|s| s.parse::<i64>().unwrap_or(1)).product::<i64>();
            },
            unknown => {
                eprintln!("Unable to compute calculation for {}", unknown);
                exit(1)
            }
        }
    };
    result
}

fn solve2(input: &str) -> i64 {
    let matrix = build_str_matrix_transpose(input);
    let mut result = 0;
    for entry in matrix {
        match entry[entry.len() - 1] {
            "+" => {
                todo!("Add for loop that builds the cephalopod numbers")
            },
            "*" => {
                todo!("Add for loop that builds the cephalopod numbers")
            },
            unknown => {
                eprintln!("Unable to compute calculation for {}", unknown)
            }
        }
    }
    result
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
        let input = match get_test_and_store(2025, 6) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve1(&input), 4277556);
    }
    
    #[test]
    fn test_solve2() {
        let input = match get_test_and_store(2025, 6) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve2(&input), 3263827);
    }
}
