use adventofcode::{day1, day2};

fn main() {
    println!("Advent of Code");
	let parts = [
		day1::part1, day1::part2,
		day2::solve
	];
	for part in parts {
		match part() {
			Err(message) => println!("Error: {message}"),
			_ => {}
		}
	}
}
