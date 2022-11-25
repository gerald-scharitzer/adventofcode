pub mod day1;

pub struct Puzzle<'a> {
    year: i32,
    day: i32,
    name: &'a str
}

impl<'a> Puzzle<'a> {
    pub fn new(year: i32, day: i32, name: &'a str) -> Puzzle<'a> {
        Puzzle { year, day, name }
    }

    pub fn get_year(&self) -> i32 { self.year }
    pub fn get_day(&self) -> i32 { self.day }
    pub fn get_name(&self) -> &str { self.name }
}
