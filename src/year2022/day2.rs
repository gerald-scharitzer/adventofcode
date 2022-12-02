use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day2 {}

impl<'a> Puzzle<'a> for Day2 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 2 }
	fn get_name(&self) -> &'a str { "Rock Paper Scissors" }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day2.in").expect("open input failed");
		let reader = BufReader::new(file);
		let mut score = 0;

		const ELF_ROCK: char = 'A';
		const YOU_ROCK: char = 'X';
		const ELF_PAPER: char = 'B';
		const YOU_PAPER: char = 'Y';
		const ELF_SCISSORS: char = 'C';
		const YOU_SCISSORS: char = 'Z';
		const DRAW: i32 = 3;
		const WIN: i32 = 6;
		
		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			let mut elf_rock = false;
			let mut you_rock = false;
			let mut elf_paper = false;
			let mut you_paper = false;
			let mut elf_scissors = false;
			let mut you_scissors = false;
		
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
				YOU_ROCK => {
					you_rock = true;
					score += 1;
					if elf_rock {
						score += DRAW;
					} else if elf_scissors {
						score += WIN;
					} // else you lose and do not score
				},
				YOU_PAPER => {
					you_paper = true;
					score += 2;
					if elf_rock {
						score += WIN;
					} else if elf_paper {
						score += DRAW;
					} // else you lose and do not score
				},
				YOU_SCISSORS => {
					you_scissors = true;
					score += 3;
					if elf_paper {
						score += WIN;
					} else if elf_scissors {
						score += DRAW;
					} // else you lose and do not score
				},
				_ => return Err(format!("character must be [XYZ] but was {c}"))
			}

		}
		
		println!("score {score}");
		Ok((score, 0))
	}
}
