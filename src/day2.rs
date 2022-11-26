use std::fs::File;
use std::io::{BufRead, BufReader};

use super::Puzzle;

pub fn part1() -> Result<i32, String> {
	let puzzle = Puzzle::new(2015, 2, "I Was Told There Would Be No Math");
	let year = puzzle.get_year();
	let day = puzzle.get_day();
	let name = puzzle.get_name();
	println!("Year {year} Day {day}: {name}");
	let file = File::open("day2.in").expect("open input failed"); // TODO derive file name from module name
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	let mut length = 0;
	let mut width = 0;
	let mut height = 0;
	
	// FIXME iterate over lines
	reader.read_line(&mut line).expect("read line failed");
	if line.ends_with('\n') {
		line = line.trim_end().to_string();
	}

	let dimensions = line.split('x');
	let mut x = 0;
	let mut slack = 0;

	for dimension in dimensions {
		x += 1;
		if x > 3 {
			return Err("number of dimensions must be 3".into());
		}
		let result = dimension.parse::<i32>();
		let dim;
		
		match result {
			Ok(value) => dim = value,
			_ => return Err(format!("dimension must be an integer but was {dimension}"))
		}
		
		match x {
			1 => length = dim,
			2 => width = dim,
			3 => height = dim,
			_ => {}
		}

		if x == 1 {
			slack = dim;
		} else {
			if dim < slack {
				slack = dim;
			}
		}
	}

	if x != 3 {
		return Err(format!("number of dimensions must be 3 but was {x}"));
	}

	let surface = 2 * (length*width + width*height + length*height);
	let area = surface + slack;
	println!("area {area}");
	Ok(area)
}

pub fn part2() -> Result<i32, String> {
	part1()
}
