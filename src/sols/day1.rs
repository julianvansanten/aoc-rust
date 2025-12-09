/// Return the number for a given input string
fn get_num(input: &str) -> i64 {
    if input.contains("L") {
        let num: i64 = input.trim_start_matches(|s| s == 'L').parse().unwrap_or(0);
        -num
    } else if input.contains("R") {
        input.trim_start_matches(|s| s == 'R').parse().unwrap_or(0)
    } else {
        0
    }
}

fn solve1(input: &str) -> i64 {
    input
        .lines()
        .map(get_num)
        .scan(50, |acc, x| {
            *acc = (*acc + x).rem_euclid(100);
            Some(*acc)
        })
        .filter(|x| *x == 0)
        .count() as i64
}

fn solve2(input: &str) -> i64 {
    // match input
    //     .lines()
    //     .map(get_num)
    //     .scan((50, 0, 0), |(acc, zeros, click), x| {
    //         *click += (*acc + x).abs() / 100;
    //         *acc = (*acc + x).rem_euclid(100);
    //         if (*acc) == 0 {
    //             *click -= 1;
    //             *zeros += 1;
    //         }
    //         Some((*acc, *zeros, *click))
    //     })
    //     .last()
    // {
    //     Some((_, zeros, clicks)) => zeros + clicks,
    //     None => 0,
    // }

    match input
        .lines()
        .map(get_num)
        .scan((50, 0, 0), |(acc, zeros, click), x| {
            *acc = (*acc + x).rem_euclid(100);
            *click += (*acc + x.abs()) / 100;
            if (*acc) == 0 {
                *click -= 1;
                *zeros += 1;
            }
            Some((*acc, *zeros, *click))
        }).last() {
            Some((_, z, c)) => z + c,
            None => 0,
        }
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate:: aoc_lib;

    #[test]
    fn test_get_num() {
        assert_eq!(get_num("L10"), -10);
        assert_eq!(get_num("R10"), 10);
        assert_eq!(get_num("R500"), 500);
        assert_eq!(get_num("L500"), -500);
        assert_eq!(get_num("AAAA"), 0);
    }

    #[test]
    fn test_solve1() {
        let input = match aoc_lib::get_test_and_store(2025, 1) {
            Ok(i) => i,
            Err(_) => panic!("Cannot read input!"),
        };
        assert_eq!(solve1(input.as_str()), 3)
    }

    #[test]
    fn test_solve2() {
        let input = match aoc_lib::get_test_and_store(2025, 1) {
            Ok(i) => i,
            Err(_) => panic!("Cannot read input!"),
        };
        println!("Testing input:\n{}", input);
        assert_eq!(solve2(input.as_str()), 6)
    }
}
