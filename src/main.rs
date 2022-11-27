use adventofcode::{day1, day2};

fn main() {
    println!("Advent of Code");
	let puzzles = [
		day1::solve, day2::solve
	];
	for puzzle in puzzles {
		match puzzle() {
			Err(message) => println!("Error: {message}"),
			_ => {}
		}
	}
}
