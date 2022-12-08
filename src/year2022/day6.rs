use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day6 {}

impl<'a> Puzzle<'a> for Day6 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 6 }
	fn get_name(&self) -> &'a str { "Tuning Trouble" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("packet", "message") }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day6.in").expect("open input failed");
		let mut reader = BufReader::new(file);
		let mut line = String::new();
		const MARKER_LENGTH: usize = 14; // TODO different packet and message marker lengths
		let mut length: usize = 0;
		let mut position: i32 = 0;
		reader.read_line(&mut line).expect("read line failed");
		let chars = line.chars();
		let mut deque: VecDeque<char> = VecDeque::with_capacity(MARKER_LENGTH-1); // TODO hashmap or int array instead

		for c in chars {
			if c == '\n' {
				break;
			}
			length += 1;
			if length >= MARKER_LENGTH {
				if !deque.contains(&c) {
					let mut found = false;
					let mut x: usize = 0;
					'marker_loop: for &m in &deque {
						x += 1;
						let range = x..MARKER_LENGTH-1;
						for y in range {
							if m == deque[y] {
								found = true;
								break 'marker_loop;
							}
						}
					}
					if !found { // all different
						break;
					}
				}
				deque.pop_front();
			}
			deque.push_back(c);
		}

		let length = length.try_into().unwrap();
		Ok((length, position))
	}
}
