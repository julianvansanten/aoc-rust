use std::env;

use dotenv::dotenv;
use reqwest::{self as req, header::COOKIE};
use chrono::{self, Datelike, Utc};

fn validate_date(year: i32, day: u32) -> bool {
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
    return day >= 1 && day <= current_day;
}

/// Pull the puzzle input from advent of code with a given string
pub async fn get_input_from_aoc(
    cookie: &String,
    year: &String,
    day: &String,
) -> Result<String, &'static str> {
    if cookie.len() != 128 {
        return Err("Cookie is invalid!");
    }

    let year_int: i32 = year.parse().unwrap_or(0);
    let day_int: u32 = year.parse().unwrap_or(0);

    if !validate_date(year_int, day_int) {
        return Err("Year or day invalid!");
    }

    let client = req::Client::new();

    client
        .get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header(COOKIE, cookie)
        .send()
        .await
        .map_err(|_| "Unable to execute request!")?
        .text()
        .await
        .map_err(|_| "Unable to unpack result string!")
}

pub fn get_cookie_from_env() -> Result<String, env::VarError> {
    let _ = dotenv();

    env::var("COOKIE")
}

#[cfg(test)]
mod test {
    use super::*;

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