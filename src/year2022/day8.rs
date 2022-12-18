use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::super::Puzzle;

pub struct Day8 {}

impl<'a> Puzzle<'a> for Day8 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 8 }
	fn get_name(&self) -> &'a str { "Treetop Tree House" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("visible", "") }
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open(format!("{year}/day{day}.in")).expect("open input failed");
		let reader = BufReader::new(file);
		const MAX_HEIGHT: u8 = 9;
		let mut highest = 0;
		let mut rows: Vec<Vec<u8>> = Vec::with_capacity(16);
		let mut visibility: Vec<Vec<bool>> = Vec::with_capacity(16);
		let mut visible = 0;
		let mut position: i32 = 0;

		let lines = reader.lines();
		for line_result in lines {
			let line;
			match line_result {
				Ok(string) => line = string,
				_ => return Err(format!("read line failed"))
			}
			let mut row: Vec<u8> = Vec::with_capacity(16);
			let mut row_visible: Vec<bool> = Vec::with_capacity(16);

			let chars = line.chars();
			for c in chars {
				let tree;
				match c {
					'0'..='9' => tree = u8::try_from(c).unwrap(),
					_ => return Err(format!("invalid digit: {c}"))
				}
				let is_visible;
				if tree > highest { // is visible
					is_visible = true;
					highest = tree;
				} else {
					is_visible = false;
				}
				if rows.len() > 0 && row.len() > 0 { // first row and first column are visible
					row_visible.push(is_visible);
				}
				row.push(tree);
			}

			if row.len() > 0 {
				if rows.len() > 1 { // entire first row is visible
					highest = 0;
					let mut x = row.len();
					let east_to_west = row.iter().rev();
					for &tree in east_to_west {
						x -= 1;
						if tree > highest { // is visible
							highest = tree;
							if x > 0 && x + 1 < row.len() {
								row_visible[x-1] = true;
							}
						}
						if tree == MAX_HEIGHT { // no more trees visible from the east
							break;
						}
					}
				}
				rows.push(row);
				if rows.len() > 1 {
					row_visible.pop(); // last column is visible
					visibility.push(row_visible);
				}
			}
		}

		visibility.pop(); // last row is visible
		let mut x = 0;

		// vertical visibility
		for column in &rows[0][1..(rows[0].len()-2)] {
			x += 1;
			// northern visibility
			highest = 0;
			let mut y = 0;
			for row in &rows {
				let tree = row[x];
				if tree > highest { // is visible
					highest = tree;
					if y > 0 && y + 1 < rows.len() {
						visibility[y-1][x-1] = true;
					}
				}
				if tree == MAX_HEIGHT { // no more trees visible from the north
					break;
				}
				y += 1;
			}

			// southern visibility
			highest = 0;
			y = rows.len();
			let south_to_north = rows.iter().rev();
			for row in south_to_north {
				y -= 1;
				let tree = row[x];
				if tree > highest { // is visible
					highest = tree;
					if y > 0 && y + 1 < rows.len() {
						visibility[y-1][x-1] = true;
					}
				}
				if tree == MAX_HEIGHT { // no more trees visible from the south
					break;
				}
			}
		}

		for row_visibility in visibility {
			for is_visible in row_visibility {
				if is_visible {
					visible += 1;
				}
			}
		}

		visible += 2 * rows.len(); // first and last column are visible
		visible += 2 * rows[0].len(); // first and last row are visible
		let visible = (visible - 4) as i32; // compensate double-count of corners FIXME 1450 is too low

		Ok((visible, position))
	}
}
