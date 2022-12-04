use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day4 {}

impl<'a> Puzzle<'a> for Day4 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 4 }
	fn get_name(&self) -> &'a str { "Camp Cleanup" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("pairs", "") }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day4.in").expect("open input failed"); // TODO derive file name from module name
		let reader = BufReader::new(file);
		let mut pairs = 0;
		let mut ribbon = 0;
		
		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			let mut ranges = line.split(',');
			let mut range = ranges.next().unwrap();
			let mut sections = range.split('-');

			let mut section = sections.next().unwrap();
			let lower1;
			let mut parse_result = section.parse::<i32>();
			match parse_result {
				Ok(value) => lower1 = value,
				_ => return Err(format!("lower section must be an integer but was {section}"))
			}

			section = sections.next().unwrap();
			let upper1;
			parse_result = section.parse::<i32>();
			match parse_result {
				Ok(value) => upper1 = value,
				_ => return Err(format!("upper section must be an integer but was {section}"))
			}

			range = ranges.next().unwrap();
			sections = range.split('-');

			section = sections.next().unwrap();
			let lower2;
			parse_result = section.parse::<i32>();
			match parse_result {
				Ok(value) => lower2 = value,
				_ => return Err(format!("lower section must be an integer but was {section}"))
			}

			section = sections.next().unwrap();
			let upper2;
			parse_result = section.parse::<i32>();
			match parse_result {
				Ok(value) => upper2 = value,
				_ => return Err(format!("upper section must be an integer but was {section}"))
			}

			if lower1 < lower2 {
				if upper1 >= upper2 {
					pairs += 1;
				}
			} else if lower1 > lower2 {
				if upper1 <= upper2 {
					pairs += 1;
				}
			} else { // equal
				pairs += 1;
			}
		}
		Ok((pairs, ribbon))
	}
}
