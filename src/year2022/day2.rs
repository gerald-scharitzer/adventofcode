use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day2 {}

impl<'a> Puzzle<'a> for Day2 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 2 }
	fn get_name(&self) -> &'a str { "Rock Paper Scissors" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("shape", "outcome") }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day2.in").expect("open input failed");
		let reader = BufReader::new(file);
		let mut shape_score = 0;
		let mut outcome_score = 0;

		const ELF_ROCK: char = 'A';
		const YOU_ROCK: char = 'X';
		const ELF_PAPER: char = 'B';
		const YOU_PAPER: char = 'Y';
		const ELF_SCISSORS: char = 'C';
		const YOU_SCISSORS: char = 'Z';
		const ROCK: i32 = 1;
		const PAPER: i32 = 2;
		const SCISSORS: i32 = 3;
		const DRAW: i32 = 3;
		const WIN: i32 = 6;
		
		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			let mut elf_rock = false;
			let mut elf_paper = false;
			let mut elf_scissors = false;
		
			// elf
			let mut chars = line.chars();
			let mut c = chars.next().unwrap();
			match c {
				ELF_ROCK => elf_rock = true,
				ELF_PAPER => elf_paper = true,
				ELF_SCISSORS => elf_scissors = true,
				_ => return Err(format!("character must be [ABC] but was {c}"))
			}

			// blank
			c = chars.next().unwrap();
			if c != ' ' {
				return Err(format!("expected ' ' but got '{c}'"))
			}

			// you
			c = chars.next().unwrap();
			match c {
				YOU_ROCK => { // part 2 you lose
					shape_score += ROCK;
					if elf_rock {
						shape_score += DRAW;
						// part 2 scissors
						outcome_score += SCISSORS;
					} else if elf_paper {
						// part 1 you lose and do not score
						// part 2 rock
						outcome_score += ROCK;
					} else {
						shape_score += WIN;
						// part 2 paper
						outcome_score += PAPER;
					}
				},
				YOU_PAPER => { // part 2 draw
					shape_score += PAPER;
					outcome_score += DRAW;
					if elf_rock {
						shape_score += WIN;
						// part 2 rock
						outcome_score += ROCK;
					} else if elf_paper {
						shape_score += DRAW;
						// part 2 paper
						outcome_score += PAPER;
					} else {
						// part 1 you lose and do not score
						// part 2 scissors
						outcome_score += SCISSORS;
					}
				},
				YOU_SCISSORS => { // part 2 you win
					shape_score += SCISSORS;
					outcome_score += WIN;
					if elf_rock {
						// part 1 you lose and do not score
						// part 2 paper
						outcome_score += PAPER;
					} else if elf_paper {
						shape_score += WIN;
						// part 2 scissors
						outcome_score += SCISSORS;
					} else if elf_scissors {
						shape_score += DRAW;
						// part 2 rock
						outcome_score += ROCK;
					}
				},
				_ => return Err(format!("character must be [XYZ] but was {c}"))
			}
		}
		Ok((shape_score, outcome_score))
	}
}
