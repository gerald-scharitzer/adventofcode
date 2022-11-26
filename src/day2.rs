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
	// let length: i32;
	// let width: i32;
	// let height: i32;
	// let surface = 2 * (length*width + width*height + length*height);
	// let slack;
	// let area = surface + slack;
	let area: i32 = 0;
	reader.read_line(&mut line).expect("read line failed");
	let chars = line.chars();
	let mut result = Ok(0);

	for char in chars {
		match char {
			'0'..='9' => {},
			'x' => {},
			'\n' => {},
			_ => { result = Err(format!("invalid character")); break }
		}
	}

	println!("area {area}");
    match result {
        Ok(_) => Ok(area),
        _ => result
    }
}

pub fn part2() -> Result<i32, String> {
	part1()
}
