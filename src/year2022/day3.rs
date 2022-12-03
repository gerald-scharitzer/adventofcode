use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day3 {}

impl<'a> Puzzle<'a> for Day3 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 3 }
	fn get_name(&self) -> &'a str { "Rucksack Reorganization" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("item", "badge") }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day3.in").expect("open input failed");
		let reader = BufReader::new(file);
		let priority_map: HashMap<char, i32> = HashMap::from([
			('a', 1),
			('b', 2),
			('c', 3),
			('d', 4),
			('e', 5),
			('f', 6),
			('g', 7),
			('h', 8),
			('i', 9),
			('j', 10),
			('k', 11),
			('l', 12),
			('m', 13),
			('n', 14),
			('o', 15),
			('p', 16),
			('q', 17),
			('r', 18),
			('s', 19),
			('t', 20),
			('u', 21),
			('v', 22),
			('w', 23),
			('x', 24),
			('y', 25),
			('z', 26),
			('A', 27),
			('B', 28),
			('C', 29),
			('D', 30),
			('E', 31),
			('F', 32),
			('G', 33),
			('H', 34),
			('I', 35),
			('J', 36),
			('K', 37),
			('L', 38),
			('M', 39),
			('N', 40),
			('O', 41),
			('P', 42),
			('Q', 43),
			('R', 44),
			('S', 45),
			('T', 46),
			('U', 47),
			('V', 48),
			('W', 49),
			('X', 50),
			('Y', 51),
			('Z', 52)
		]);
		let mut items0: HashSet<char> = HashSet::with_capacity(16);
		let mut items1: HashSet<char> = HashSet::with_capacity(16);
		let mut elf = 0;
		let mut priority_sum = 0;
		let mut badge_priority = 0;

		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			let compartment_size = line.len() / 2;
			let mut compartment: HashSet<char> = HashSet::with_capacity(compartment_size);
			let mut count = 0;
			let mut found2 = false;
			let mut found3 = false;

			let chars = line.chars();
			for c in chars {
				match c {
					'a'..='z' | 'A'..='Z' => {
						// part 1
						if count < compartment_size { // compartment 1
							count += 1;
							compartment.insert(c);
						} else if !found2 { // compartment 2
							if compartment.contains(&c) {
								found2 = true;
								priority_sum += priority_map.get(&c).unwrap();
							}
						}

						// part 2
						match elf {
							0 => {
								items0.insert(c);
							},
							1 => {
								if items0.contains(&c) {
									items1.insert(c);
								}
							},
							2 => {
								if !found3 {
									if items1.contains(&c) {
										found3 = true;
										badge_priority += priority_map.get(&c).unwrap();
									}
								}
							},
							_ => return Err(format!("expected [012] but got {elf}"))
						}
					},
					_ => return Err(format!("expected [a-zA-Z] but got '{c}'"))
				}
			}

			// part 2
			match elf {
				0 => {
					elf = 1;
				},
				1 => {
					items0.clear();
					elf = 2;
				},
				2 => {
					items1.clear();
					elf = 0;
				},
				_ => return Err(format!("expected [012] but got {elf}"))
			}
		}
		Ok((priority_sum, badge_priority))
	}
}
