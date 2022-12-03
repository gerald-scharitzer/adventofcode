use adventofcode::Puzzle;
use adventofcode::year2022::{Day1, Day2, Day3};

fn main() {
    println!("Advent of Code");
	// TODO print answer names
	// TODO let puzzles: Vec<dyn Puzzle> = vec![Day1::new(), Day2::new()];
	// TODO for puzzle in puzzles {
	let day1 = Day1::new();
	let mut names = day1.get_answer_names();
	let mut result = day1.solve();
	print_answers(names, result);

	let day2 = Day2::new();
	names = day2.get_answer_names();
	result = day2.solve();
	print_answers(names, result);

	let day3 = Day3::new();
	names = day3.get_answer_names();
	result = day3.solve();
	print_answers(names, result);
	//}
}

fn print_answers(names: (&str, &str), result: Result<(i32, i32), String>) {
	match result {
		Ok((value1, value2)) => {
			let (name1, name2) = names;
			println!("{name1} {value1}");
			println!("{name2} {value2}");
		}
		Err(message) => println!("Error: {message}"),
	}
}