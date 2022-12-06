use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use super::super::Puzzle;

pub struct Day5 {}

impl<'a> /*Puzzle<'a> for*/ Day5 {
	pub fn new() -> Self { Self {} }
	pub fn get_year(&self) -> i32 { 2022 }
	pub fn get_day(&self) -> i32 { 5 }
	pub fn get_name(&self) -> &'a str { "Supply Stacks" }
	pub fn get_answer_names(&self) -> (&'a str, &'a str) { ("crates", "") }
	pub fn solve(&self) -> Result<(String, String), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day5.in").expect("open input failed"); // TODO derive file name from module name
		let reader = BufReader::new(file);
		let mut stacks: Vec<Vec<char>> = Vec::with_capacity(10);
		let mut stack_count = 0;
		let mut crates1 =  "".to_string();
		let mut crates2 =  "".to_string();
		let mut moves = false;
		let mut is_crate = false;
		
		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			if !moves { // stacks
				if line.is_empty() { // reverse stacks
					moves = true;
					for crates in &mut stacks {
						crates.reverse();
					}
					stack_count = stacks.len();
				} else { // get crates
					let mut x = 0;
					let mut column = 0;
					for c in line.chars() {
						match x {
							0 => {
								match c {
									'[' => is_crate = true,
									' ' => {},
									_ => return Err(format!("expected left bracket or blank but got {c}"))
								}
								x = 1;
							},
							1 => {
								if is_crate {
									match c {
										'A'..='Z' => {
											while stacks.len() <= column {
												let crates: Vec<char> = Vec::new();
												stacks.push(crates);
											}
											stacks[column].push(c);
										},
										_ => return Err(format!("expected capital letter but got {c}"))
									}
								} else {
									match c {
										' ' | '0'..='9' => {},
										_ => return Err(format!("expected blank or digit but got {c}"))
									}
								}
								x = 2;
							},
							2 => {
								if is_crate {
									if c != ']' {
										return Err(format!("expected right bracket but got {c}"));
									}
									is_crate = false;
								} else {
									if c != ' ' {
										return Err(format!("expected blank but got {c}"));
									}
								}
								x = 3;
							},
							3 => {
								if c != ' ' {
									return Err(format!("expected blank but got {c}"));
								}
								column += 1;
								x = 0;
							},
							_ => return Err(format!("expected 0..=3 but got {x}"))
						}
					}
	
				}
			} else { // moves
				let mut words = line.split(' ');
				let mut word = words.next().unwrap();
				if word != "move" {
					return Err(format!("expected move but got {word}"));
				}

				word = words.next().unwrap();
				let mut parse_result = word.parse::<usize>();
				let mut count;
				match parse_result {
					Ok(value) => count = value,
					_ => return Err(format!("count must be an integer but was {word}"))
				}

				word = words.next().unwrap();
				if word != "from" {
					return Err(format!("expected from but got {word}"));
				}

				word = words.next().unwrap();
				parse_result = word.parse::<usize>();
				let mut source;
				match parse_result {
					Ok(value) => source = value,
					_ => return Err(format!("source must be an integer but was {word}"))
				}
				if source < 1 || source > stack_count {
					return Err(format!("source must be within 1..={stack_count} but was {source}"));
				}

				word = words.next().unwrap();
				if word != "to" {
					return Err(format!("expected to but got {word}"));
				}
				
				word = words.next().unwrap();
				parse_result = word.parse::<usize>();
				let mut target;
				match parse_result {
					Ok(value) => target = value,
					_ => return Err(format!("target must be an integer but was {word}"))
				}
				if target < 1 || target > stack_count {
					return Err(format!("target must be within 1..={stack_count} but was {source}"));
				}

				source -= 1;
				target -= 1;
				while count > 0 {
					let top = stacks[source].pop();
					match top {
						Some(c) => stacks[target].push(c),
						_ => return Err(format!("pop stack {source} failed"))
					}
					count -= 1;
				}
			}
		}

		for mut stack in stacks {
			let top = stack.pop();
			match top {
				Some(c) => crates1.push(c),
				_ => {}
			}
		}
		Ok((crates1, crates2))
	}
}
