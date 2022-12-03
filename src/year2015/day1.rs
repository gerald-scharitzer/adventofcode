use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day1 {}

impl<'a> Puzzle<'a> for Day1 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2015 }
	fn get_day(&self) -> i32 { 1 }
	fn get_name(&self) -> &'a str { "Not Quite Lisp" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("floor", "position") }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("day1.in").expect("open input failed");
		let mut reader = BufReader::new(file);
		let mut line = String::new();
		let mut floor: i32 = 0;
		let mut basement = false;
		let mut position: i32 = 0;
		reader.read_line(&mut line).expect("read line failed");
		let chars = line.chars();

		for c in chars {
			if !basement {
				position += 1;
			}
			match c {
				'(' => floor += 1,
				')' => floor -= 1,
				_ => return Err(format!("invalid character: {c}"))
			}
			if floor == -1 {
				basement = true;
			}
		}
		Ok((floor, position))
	}
}
