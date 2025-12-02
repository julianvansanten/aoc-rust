use crate::aoc_lib::get_cookie_from_env;

mod aoc_lib;

fn main() {
    match get_cookie_from_env() {
        Ok(res) => println!("Cookie: {}", res),
        Err(err) => eprintln!("{}", err),
    }
}
