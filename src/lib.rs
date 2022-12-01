pub mod year2015;
pub mod year2022;

pub trait Puzzle<'a> {
    fn new() -> Self;
    fn get_year(&self) -> i32;
    fn get_day(&self) -> i32;
    fn get_name(&self) -> &'a str;
    fn solve(&self) -> Result<(i32, i32), String>;
}
