use regex::Regex;
use std::env;

mod aoc_lib;

use aoc_lib::validate_date;

use crate::aoc_lib::get_input_and_store;

struct Selector {
    year: i32,
    day: u32,
    puzzle2: bool,
}

fn build_exercise_selector(args: Vec<String>) -> Selector {
    let mut selector = Selector {
        year: 0,
        day: 0,
        puzzle2: false,
    };
    let reg = Regex::new("").unwrap();
    while !validate_date(selector.year, selector.day) {}

    selector
}

fn main() {
    // Which year?
    // Which day?
    // Cookie?
    let res = get_input_and_store(2025, 1);
    match res {
        Ok(res) => println!("Success! {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}
