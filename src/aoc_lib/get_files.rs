use std::{env, io::{self, Read}};

use chrono::{self, Datelike, Utc};
use dotenv::dotenv;
use reqwest::{blocking as req, header::COOKIE};
use std::{fs, path::Path};

const FOLDER_NAME: &str = "puzzle_input";
const PUZZLE_INPUT: &str = "input";
const PUZZLE_TEST: &str = "test";

/// Validate a year and a day for the puzzle
pub fn validate_date(year: i32, day: u32) -> bool {
    let current_year: i32 = Utc::now().year();
    let current_day: u32 = Utc::now().day();
    if year < 2015 || year > current_year {
        return false;
    }
    if year < 2025 {
        return 1 <= day && day <= 25;
    }
    if year < current_year {
        return 1 <= day && day <= 12;
    }
    return day >= 1 && day <= current_day && day <= 12;
}

/// Pull the puzzle input from advent of code with a given string
fn get_input_from_aoc(cookie: &String, year: i32, day: u32) -> Result<String, &'static str> {
    if cookie.len() != 128 {
        return Err("Cookie is invalid!");
    }

    if !validate_date(year, day) {
        return Err("Year or day invalid!");
    }

    let client = req::Client::new();

    client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(COOKIE, format!("session={cookie}"))
        .send()
        .map_err(|_| "Unable to execute request!")?
        .text()
        .map_err(|_| "Unable to unpack result string!")
}

/// Try to get the cookie from the environment variables
fn get_cookie_from_env() -> Result<String, env::VarError> {
    let _ = dotenv();
    env::var("COOKIE")
}

/// Try to read input from an existing file
fn get_input_from_file(year: i32, day: u32, is_test: bool) -> Result<String, &'static str> {
    if !validate_date(year, day) {
        return Err("Year or day invalid!");
    }
    let content = fs::read_to_string(format!(
        "./{FOLDER_NAME}/{year}/{}/{day}.txt",
        if is_test { PUZZLE_TEST } else { PUZZLE_INPUT }
    ));
    content.map_err(|_| "Unable to read puzzle input for year {year} and day {day}!")
}

fn write_to_file(
    year: i32,
    day: u32,
    input: &String,
    is_test: bool,
) -> Result<String, &'static str> {
    let path = format!(
        "./{FOLDER_NAME}/{year}/{}/{day}.txt",
        if is_test { PUZZLE_TEST } else { PUZZLE_INPUT }
    );
    let exists: bool;
    match fs::exists(&path) {
        Ok(res) => exists = res,
        Err(_) => exists = false,
    }
    if exists {
        return get_input_from_file(year, day, is_test);
    }
    // Recursively create all parent directories
    if let Some(parent) = Path::new(&path).parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => {}
            Err(_) => return Err("Could not create extra folders!"),
        }
    }
    match fs::write(&path, input) {
        Ok(_) => Ok(input.to_string()),
        Err(_) => Err("Could not write the input to file!"),
    }
}

/// Try to get the input from a file, otherwise pull it from AdventOfCode.com
/// Store the result locally
pub fn get_input_and_store(year: i32, day: u32) -> Result<String, &'static str> {
    if !validate_date(year, day) {
        return Err("Invalid puzzle year or day!");
    }
    let input: String = match get_input_from_file(year, day, false) {
        Ok(res) => res,
        Err(_) => {
            let cookie = match get_cookie_from_env() {
                Ok(res) => res,
                Err(_) => return Err("Cannot get cookie!"),
            };
            match get_input_from_aoc(&cookie, year, day) {
                Ok(res) => res,
                Err(_) => return Err("Unable to make request"),
            }
        }
    };
    write_to_file(year, day, &input, false)
}

/// Try to get the test data from a file, otherwise ask the user to paste the input
pub fn get_test_and_store(year: i32, day: u32) -> Result<String, &'static str> {
    match get_input_from_file(year, day, true) {
        Ok(res) => Ok(res),
        Err(_) => {
            println!("Paste test input (Ctrl+D to finish):");
            let mut input = String::new();
            io::stdin()
                .read_to_string(&mut input)
                .map_err(|_| "Failed to read from stdin")?;
            input = String::from(input.as_str().trim());
            match write_to_file(year, day, &input, true) {
                Ok(_) => (),
                Err(_) => eprintln!("Failed to write test to file! Continuing..."),
            }
            Ok(input)
        }
    }
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn test_write_to_file() {
        let input = String::from("Test\ninput");
        let result = write_to_file(2015, 1, &input, false);
        match result {
            Ok(text) => assert_eq!(input, text),
            Err(err) => panic!("{}", err),
        }
        let second_result = write_to_file(2015, 1, &input, false);
        match second_result {
            Ok(text) => assert_eq!(input, text),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn test_validate_year() {
        let current_year: i32 = Utc::now().year();
        let current_day: u32 = Utc::now().day();

        assert!(validate_date(2015, 1));
        assert!(validate_date(2015, 25));
        assert!(!validate_date(2014, 1));
        assert!(!validate_date(2015, 0));
        assert!(!validate_date(2015, 26));

        assert!(validate_date(current_year, current_day));
        assert!(!validate_date(current_year + 1, 1));
        assert!(!validate_date(current_year, current_day + 1));
    }
}
