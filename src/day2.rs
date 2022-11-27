use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Puzzle;

pub fn solve() -> Result<i32, String> {
	let puzzle = Puzzle::new(2015, 2, "I Was Told There Would Be No Math");
	let year = puzzle.get_year();
	let day = puzzle.get_day();
	let name = puzzle.get_name();
	println!("Year {year} Day {day}: {name}");
	let file = File::open("day2.in").expect("open input failed"); // TODO derive file name from module name
	let reader = BufReader::new(file);
	let mut length = 0;
	let mut width = 0;
	let mut height = 0;
	let mut paper = 0;
	let mut ribbon = 0;
	
	for line_result in reader.lines() {
		let line;
		match line_result {
			Ok(string) => line = string,
			Err(error) => return Err("read line failed".into())
		}

		let dimensions = line.split('x');
		let mut x = 0;

		for dimension in dimensions {
			x += 1;
			if x > 3 {
				return Err("number of dimensions must be 3".into());
			}

			let dim;
			let parse_result = dimension.parse::<i32>();
			match parse_result {
				Ok(value) => dim = value,
				_ => return Err(format!("dimension must be an integer but was {dimension}"))
			}
			
			match x {
				1 => length = dim,
				2 => width = dim,
				3 => height = dim,
				_ => {}
			}
		}

		if x != 3 {
			return Err(format!("number of dimensions must be 3 but was {x}"));
		}

		// part 1
		let bottom = length * width;
		let front = length * height;
		let side = width * height;
		let surface = 2 * (bottom + front + side);
		let mut slack = min(bottom, front);
		slack = min(slack, side);
		paper += surface + slack;

		// part 2
		let side1;
		let side2;
		if length < width {
			side1 = length;
			if width < height {
				side2 = width;
			} else {
				side2 = height;
			}
		} else {
			side1 = width;
			if length < height {
				side2 = length;
			} else {
				side2 = height;
			}
		}
		let volume = length * width * height;
		ribbon += 2 * (side1 + side2) + volume;
	}

	println!("paper {paper}");
	println!("ribbon {ribbon}");
	Ok(paper)
}
