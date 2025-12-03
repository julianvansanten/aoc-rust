use crate::aoc_lib::Solver;

struct Day1;

fn get_num(input: String) -> i32 {
    if input.contains("L") {
        let num: i32 = input.trim_start_matches(|s| s == 'L').parse().unwrap();
        -num
    } else if input.contains("R") {
        input.trim_start_matches(|s| s == 'R').parse().unwrap()
    } else {
        0
    }
}

impl Solver<Vec<i32>, &'static str> for Day1 {

    fn parse(&self, input: &String) -> Result<Vec<i32>, &'static str> {
        Ok(input.trim().lines().map(String::from).map(get_num).collect())
    }

    fn solve_1(&self, input: Vec<i32>) -> Result<i64, &'static str> {
        todo!()
    }

    fn solve_2(&self, input: Vec<i32>) -> Result<i64, &'static str> {
        todo!()
    }
}