/// A solver has two functions: a solve function for the first part and for the second
pub trait Solver {
    fn solve_1(&self, input: &String) -> Result<String, &'static str> {
        Err("Solve 1 not implemented!")
    }
    fn solve_2(&self, input: &String) -> Result<String, &'static str> {
        Err("Solve 2 not implemented!")
    }
}
