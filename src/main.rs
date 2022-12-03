use adventofcode::Puzzle;
use adventofcode::year2022::{Day1, Day2, Day3};

fn main() {
    println!("Advent of Code");
	// TODO let puzzles: Vec<dyn Puzzle> = vec![Day1::new(), Day2::new()];
	// TODO for puzzle in puzzles {
		match Day1::new().solve() {
			Err(message) => println!("Error: {message}"),
			_ => {}
		}
		match Day2::new().solve() {
			Err(message) => println!("Error: {message}"),
			_ => {}
		}
		match Day3::new().solve() {
			Err(message) => println!("Error: {message}"),
			_ => {}
		}
	//}
}
