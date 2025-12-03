/// A solver has two solvers and a parser
/// The parser parses an input String to a generic type defined in each instant
pub trait Solver<T, E> {

    /// Parse the input string
    fn parse(&self, input: &String) -> Result<T, E>;

    /// Solve the first exercise
    fn solve_1(&self, input: T) -> Result<i64, E>;
    
    /// Solve the second exercise
    fn solve_2(&self, input: T) -> Result<i64, E>;
}
