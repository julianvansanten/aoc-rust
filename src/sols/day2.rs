use itertools::Itertools;

fn get_numbers_in_range(start: i64, end: i64) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for n in start..end + 1 {
        result.push(n.to_string());
    }
    result
}

fn is_invalid_id_middle(number: &String) -> bool {
    let middle = number.len() / 2;
    &number[0..middle] == &number[middle..number.len()]
}

fn is_invalid_id_repeat(number: &String) -> bool {
    for slice in 1..(number.len() / 2 + 1) {
        if number.chars()
                .collect::<Vec<_>>()
                .chunks(slice).all_equal() {
            return true;
        }
    }
    false
}

fn solver(input: &str, invalid_check: fn(&String) -> bool) -> i64 {
    input
        .trim()
        .split(",")
        .map(|s| {
            s.split("-")
                .map(|subs| subs.trim().parse::<i64>().expect("Unable to parse number!"))
                .collect()
        })
        .map(|ranges: Vec<i64>| {
            get_numbers_in_range(ranges[0], ranges[1])
                .iter()
                .filter(|string| invalid_check(string))
                .map(|string| {
                    string
                        .parse::<i64>()
                        .expect("Unable to parse numbers in range!")
                })
                .sum::<i64>()
        })
        .sum::<i64>()
}

pub fn solve(input: &str) -> (i64, i64) {
    (solver(input, is_invalid_id_middle), solver(input, is_invalid_id_repeat))
}

#[cfg(test)]
mod tests {
    use crate::aoc_lib;

    use super::*;

    #[test]
    fn test_solve1() {
        let input = match aoc_lib::get_test_and_store(2025, 2) {
            Ok(text) => text,
            Err(_) => panic!("Unable to get test input!"),
        };

        assert_eq!(solver(input.as_str(), is_invalid_id_middle), 1227775554);
    }
    
    #[test]
    fn test_is_invalid_id_repeat() {
        assert!(is_invalid_id_repeat(&String::from("11")));
        assert!(is_invalid_id_repeat(&String::from("22")));
        assert!(is_invalid_id_repeat(&String::from("99")));
        assert!(is_invalid_id_repeat(&String::from("111")));
        assert!(is_invalid_id_repeat(&String::from("999")));
        assert!(is_invalid_id_repeat(&String::from("1010")));
        assert!(is_invalid_id_repeat(&String::from("1188511885")));
        assert!(is_invalid_id_repeat(&String::from("222222")));
        assert!(is_invalid_id_repeat(&String::from("446446")));
        assert!(is_invalid_id_repeat(&String::from("38593859")));
        assert!(is_invalid_id_repeat(&String::from("565656")));
        assert!(is_invalid_id_repeat(&String::from("824824824")));
        assert!(is_invalid_id_repeat(&String::from("2121212121")));
        assert!(!is_invalid_id_repeat(&String::from("2121222121")));
    }

    #[test]
    fn test_solve2() {
        let input = match aoc_lib::get_test_and_store(2025, 2) {
            Ok(text) => text,
            Err(_) => panic!("Unable to get test input!"),
        };
        
        assert_eq!(solver(input.as_str(), is_invalid_id_repeat), 4174379265);
    }
}
