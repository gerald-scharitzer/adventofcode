use adventofcode::day1::{part1, part2};

fn main() {
    println!("Advent of Code");
	match part1() {
		Err(message) => println!("Error: {message}"),
		_ => {}
	}
	match part2() {
		Err(message) => println!("Error: {message}"),
		_ => {}
	}
}
