use regex::Regex;
use std::env;

mod aoc_lib;
mod sols;

use aoc_lib::validate_date;

use crate::aoc_lib::get_input_and_store;

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
