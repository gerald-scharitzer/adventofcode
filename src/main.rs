use adventofcode::Puzzle;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Advent of Code");
	day1();
}

fn day1() {
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
	for char in chars {
		match char {
			'(' => floor += 1,
			')' => floor -= 1,
			' ' => break,
			_ => break
		}
	}
	println!("floor {floor}");
}