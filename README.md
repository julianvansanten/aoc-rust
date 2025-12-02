# Rust Advent of Code framework

This is my Advent of Code framework in Rust.
It is my first time learning Rust!

## Using `aoc_lib` to get puzzle input

You can use the function `get_files::get_input_and_store` to get the input for a given year and date from the AoC website.
It will first locally check if you already have the input in the folder `puzzle_input/{year}/{day}.txt` and if so, it will use that as input instead.
Use function `get_files::validate_date` to check if a given year and day are valid puzzle days.


If the file cannot be found, it will be requested from the website.
For this, you need to add an **environment variable** `COOKIE`, which can be placed in a `.env` file, or entered via the terminal.
