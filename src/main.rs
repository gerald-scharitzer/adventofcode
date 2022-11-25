use adventofcode::day1::{part1, part2};

fn main() {
    println!("Advent of Code");
	for part in [part1, part2] {
		match part() {
			Err(message) => println!("Error: {message}"),
			_ => {}
		}
	}
}
