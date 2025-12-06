/// Return the number for a given input string
fn get_num(input: String) -> i32 {
    if input.contains("L") {
        let num: i32 = input.trim_start_matches(|s| s == 'L').parse().unwrap_or(0);
        -num
    } else if input.contains("R") {
        input.trim_start_matches(|s| s == 'R').parse().unwrap_or(0)
    } else {
        0
    }
}

pub fn solve(input: &str) -> (i64, i64) {
    (0,0)
}