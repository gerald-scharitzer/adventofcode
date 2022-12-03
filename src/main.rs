use adventofcode::Puzzle;
use adventofcode::year2022::{Day1, Day2, Day3};

fn main() {
    println!("Advent of Code");
	// TODO print answer names
	// TODO let puzzles: Vec<dyn Puzzle> = vec![Day1::new(), Day2::new()];
	// TODO for puzzle in puzzles {
		match Day1::new().solve() {
			Ok((value1, value2)) => {
				println!("value1 {value1}");
				println!("value2 {value2}");
			}
			Err(message) => println!("Error: {message}"),
		}
		match Day2::new().solve() {
			Ok((value1, value2)) => {
				println!("value1 {value1}");
				println!("value2 {value2}");
			}
			Err(message) => println!("Error: {message}"),
		}
		match Day3::new().solve() {
			Ok((value1, value2)) => {
				println!("value1 {value1}");
				println!("value2 {value2}");
			}
			Err(message) => println!("Error: {message}"),
		}
	//}
}
