use adventofcode::{day1part1, day1part2};

fn main() {
    println!("Advent of Code");
	match day1part1() {
		Err(message) => println!("Error: {message}"),
		_ => {}
	}
	match day1part2() {
		Err(message) => println!("Error: {message}"),
		_ => {}
	}
}
