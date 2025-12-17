use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
    process::exit,
};

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::{complete::i64, streaming::newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
};

#[derive(Debug, Clone, Hash, Eq)]
struct Range {
    min: i64,
    max: i64,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

impl Range {
    fn new(min: i64, max: i64) -> Range {
        assert!(min <= max);
        Range { min: min, max: max }
    }

    fn can_combine(&self, other: &Range) -> bool {
        (self.min <= other.min && other.min <= (self.max + 1))
            || (self.min <= (other.max + 1) && other.max <= self.max)
    }

    fn in_range(&self, number: i64) -> bool {
        self.min <= number && number <= self.max
    }

    fn count(&self) -> i64 {
        self.max - self.min + 1
    }

    fn numbers(&self) -> Vec<i64> {
        let mut res: Vec<i64> = Vec::new();
        for x in self.min..self.max + 1 {
            res.push(x);
        }
        res
    }
}

fn combine(first: &Range, second: &Range) -> Option<Range> {
    if !first.can_combine(&second) {
        return None;
    };
    let mut min = first.min;
    if second.min <= first.min && first.min <= (second.max + 1) {
        min = second.min;
    }
    let mut max = first.max;
    if second.min <= (first.max + 1) && first.max <= second.max {
        max = second.max;
    }
    Some(Range::new(min, max))
}

fn count_numbers(ranges: Vec<Range>) -> i64 {
    let mut numbers: HashSet<i64> = HashSet::new();
    for range in ranges {
        numbers.extend(range.numbers());
    }
    numbers.len() as i64
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let parser = separated_pair(i64, tag("-"), i64);
    map(parser, |(min, max)| Range::new(min, max)).parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(newline, parse_range).parse(input)
}

fn parse_input(input: &str) -> Result<(Vec<Range>, Vec<i64>), &'static str> {
    let (remainder, ranges) = match parse_ranges(input) {
        Ok((remainder, ranges)) => (remainder, ranges),
        Err(_) => return Err("Unable to read any ranges!"),
    };
    let numbers: Vec<i64> = remainder
        .trim()
        .lines()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    Ok((ranges, numbers))
}

fn solve1(input: &str) -> i64 {
    let (ranges, numbers) = match parse_input(input) {
        Ok((r, n)) => (r, n),
        Err(_) => {
            eprintln!("Unable to parse!");
            exit(1)
        }
    };
    let mut result = 0;
    for number in numbers {
        for range in &ranges {
            if range.in_range(number) {
                result += 1;
                break;
            }
        }
    }
    result
}

fn solve2(input: &str) -> i64 {
    let ranges = match parse_input(input) {
        Ok((r, _)) => r,
        Err(_) => {
            eprintln!("Unable to parse!");
            exit(1);
        }
    };
    count_numbers(ranges)
}

pub fn solve(input: &str) -> (i64, i64) {
    (solve1(input), solve2(input))
}

#[cfg(test)]
mod tests {
    use std::panic;

    use crate::aoc_lib::get_test_and_store;

    use super::*;

    #[test]
    fn test_solve1() {
        let input = match get_test_and_store(2025, 5) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve1(&input), 3);
    }

    #[test]
    fn test_solve2() {
        let input = match get_test_and_store(2025, 5) {
            Ok(i) => i,
            Err(_) => panic!("Unable to get input!"),
        };
        assert_eq!(solve2(&input), 14);
    }

    #[test]
    fn test_count_ranges() {
        let mut ranges: Vec<Range> = Vec::new();
        ranges.push(Range::new(3, 6));
        ranges.push(Range::new(5, 8));
        assert_eq!(count_numbers(ranges), 6);
    }

    #[test]
    fn test_in_range() {
        let range = Range::new(1, 3);
        assert!(!range.in_range(0));
        assert!(range.in_range(1));
        assert!(range.in_range(2));
        assert!(range.in_range(3));
        assert!(!range.in_range(4));
    }

    #[test]
    fn test_ranges_parser() {
        let input = "1-2\n3-4\n700-800\n\n";
        match parse_ranges(input) {
            Ok((remainder, ranges)) => {
                assert_eq!(ranges.len(), 3);
                assert_eq!(remainder.len(), 2);
            }
            Err(_) => panic!("Unable to parse!"),
        }
    }

    #[test]
    fn test_parse_input() {
        let input1 = "";
        let input2 = "1-2\n3-4\n\n1\n2\n3\n4";
        match parse_input(input1) {
            Ok(_) => panic!("Invalid input should not succeed!"),
            Err(_) => (),
        }
        match parse_input(input2) {
            Ok((ranges, numbers)) => {
                assert_eq!(ranges.len(), 2);
                assert_eq!(numbers.len(), 4);
            }
            Err(_) => panic!("Valid input not correctly parsed!"),
        }
    }

    #[test]
    fn test_range_count() {
        let r1 = Range::new(5, 5);
        let r2 = Range::new(6, 7);
        let r3 = Range::new(1, 100);

        assert_eq!(r1.count(), 1);
        assert_eq!(r2.count(), 2);
        assert_eq!(r3.count(), 100);
    }
}
