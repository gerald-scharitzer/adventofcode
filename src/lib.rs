use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn day1part1() -> Result<i32, String> {
	let puzzle = Puzzle::new(2015, 1, "Not Quite Lisp");
	let year = puzzle.get_year();
	let day = puzzle.get_day();
	let name = puzzle.get_name();
	println!("Year {year} Day {day}: {name}");
	let file = File::open("day1.in").expect("open input failed");
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	let mut floor: i32 = 0;
	reader.read_line(&mut line).expect("read line failed");
	let chars = line.chars();
	let mut result = Ok(0);

	for char in chars {
		match char {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => { result = Err(format!("invalid character")); break }
		}
	}

	println!("floor {floor}");
    match result {
        Ok(_) => Ok(floor),
        _ => result
    }
}

pub fn day1part2() -> Result<i32, String> {
	println!("Part Two");
	let file = File::open("day1.in").expect("open input failed");
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	let mut floor: i32 = 0;
	reader.read_line(&mut line).expect("read line failed");
	let chars = line.chars();
	let mut result = Ok(0);
	let mut basement = false;
	let mut position: i32 = 0;
	for char in chars {
		if !basement {
			position += 1;
		}
		match char {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => { result = Err(format!("invalid character")); break }
		}
		if floor == -1 {
			basement = true;
		}
	}
	println!("position {position}");
    match result {
        Ok(_) => Ok(position),
        _ => result
    }
}
