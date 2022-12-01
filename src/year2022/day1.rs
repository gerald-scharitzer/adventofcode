use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day1 {}

impl<'a> Puzzle<'a> for Day1 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 1 }
	fn get_name(&self) -> &'a str { "Calorie Counting" }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day1.in").expect("open input failed");
		let reader = BufReader::new(file);
		let mut food = 0;
		let mut elf = 0;

		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			if line.is_empty() {
				food = max(food, elf);
				elf = 0;
			} else {
				let parse_result = line.parse::<i32>();
				match parse_result {
					Ok(energy) => elf += energy,
					_ => return Err(format!("energy must be an integer but was {line}"))
				}
			}
		}
		
		println!("food {food}");
		Ok((food, 0))
	}
}
