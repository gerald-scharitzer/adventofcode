use adventofcode::Puzzle;
use adventofcode::year2022::{Day1, Day2, Day3, Day4, Day5, Day6, Day7};

fn main() {
    println!("Advent of Code");
	// TODO print answer names
	// TODO let puzzles: Vec<dyn Puzzle> = vec![Day1::new(), Day2::new()];
	// TODO for puzzle in puzzles {
	let day = Day1::new();
	let mut names = day.get_answer_names();
	let mut result = day.solve();
	print_answers(names, result);

	let day = Day2::new();
	names = day.get_answer_names();
	result = day.solve();
	print_answers(names, result);

	let day = Day3::new();
	names = day.get_answer_names();
	result = day.solve();
	print_answers(names, result);

	let day = Day4::new();
	names = day.get_answer_names();
	result = day.solve();
	print_answers(names, result);

	let day = Day5::new();
	names = day.get_answer_names();
	let string_result = day.solve();
	print_answer_strings(names, string_result);

	let day = Day6::new();
	names = day.get_answer_names();
	result = day.solve();
	print_answers(names, result);

	let day = Day7::new();
	names = day.get_answer_names();
	result = day.solve();
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

fn print_answer_strings(names: (&str, &str), result: Result<(String, String), String>) {
	match result {
		Ok((value1, value2)) => {
			let (name1, name2) = names;
			println!("{name1} {value1}");
			println!("{name2} {value2}");
		}
		Err(message) => println!("Error: {message}"),
	}
}
