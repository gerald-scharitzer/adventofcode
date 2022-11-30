pub mod day1;
pub mod day2;

pub trait Puzzle<'a> {
    fn new() -> Self;
    fn get_year(&self) -> i32;
    fn get_day(&self) -> i32;
    fn get_name(&self) -> &'a str;
    fn solve(&self) -> Result<(i32, i32), String>;
}
