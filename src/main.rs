use std::{collections::HashMap, env, process::exit};

mod aoc_lib;
mod sols;

use crate::sols::day1;
use crate::sols::day2;

const YEAR: i32 = 2025;

fn get_input_and_solve(day: u32, solve: &fn(&str) -> (i64, i64)) {
    let input: String = match aoc_lib::get_input_and_store(YEAR, day) {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Unable to get input for y{}d{}", YEAR, day);
            exit(1);
        }
    };
    let res = solve(input.as_str());
    println!("Part 1: {}\nPart 2: {}", res.0, res.1);
}

fn get_test_and_solve(day: u32, solve: &fn(&str) -> (i64, i64)) {
    let test: String = match aoc_lib::get_test_and_store(YEAR, day) {
        Ok(test) => test,
        Err(_) => {
            eprintln!("Unable to get test input for y{}d{}", YEAR, day);
            exit(1);
        }
    };

    let res = solve(test.as_str());
    println!("Part 1: {}\nPart 2: {}", res.0, res.1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day_number> [-t]", args[0]);
        exit(1);
    }
    let day_to_run = &args[1];

    let solvers: HashMap<u32, fn(&str) -> (i64, i64)> = {
        let mut m: HashMap<u32, fn(&str) -> (i64, i64)> = HashMap::new();
        m.insert(1, day1::solve);
        m.insert(2, day2::solve);
        m
    };

    let day_to_run = match day_to_run.parse::<u32>() {
        Ok(day) => day,
        Err(_) => {
            eprintln!("{} is not a number!", day_to_run);
            exit(1);
        }
    };

    let solver = match solvers.get(&day_to_run) {
        Some(solver) => solver,
        None => {
            eprintln!("Solver for day {} is not implemented!", day_to_run);
            exit(1);
        }
    };
    
    if args.len() > 2 {
        match args[2].as_str() {
            "-t" => get_test_and_solve(day_to_run, solver),
            f => {
                eprintln!("Unknown flag {}", f);
                exit(1);
            }
        }
    } else {
        get_input_and_solve(day_to_run, solver);
    }
}
