use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use super::super::Puzzle;

pub struct Day7 {}

enum Entry {
	Directory {
		parent: usize,
		entries: Vec<usize>
	},
	File { size: usize }
}

impl<'a> Puzzle<'a> for Day7 {
	fn new() -> Self { Self {} }
	fn get_year(&self) -> i32 { 2022 }
	fn get_day(&self) -> i32 { 7 }
	fn get_name(&self) -> &'a str { "No Space Left On Device" }
	fn get_answer_names(&self) -> (&'a str, &'a str) { ("FIXME size", "") } // FIXME
	fn solve(&self) -> Result<(i32, i32), String> {
		let year = self.get_year();
		let day = self.get_day();
		let name = self.get_name();
		println!("Year {year} Day {day}: {name}");
		let file = File::open("2022/day7test.in").expect("open input failed");
		let reader = BufReader::new(file);
		let mut file_system: Vec<Entry> = Vec::with_capacity(16);
		let mut root = Entry::Directory {
			parent: 0,
			entries: Vec::with_capacity(16)
		};
		file_system.push(root);
		let mut workdir: usize = 0;
		let mut total_size: usize = 0;
		let mut position = 0;
		for line_result in reader.lines() {
			let line;
			match line_result {
				Ok(string) => line = string,
				Err(error) => return Err(format!("read line failed: {error}"))
			}

			let mut words = line.split(' ');
			let word = words.next().unwrap();
			match word {
				"$" => {
					let command =  words.next().unwrap();
					match command {
						"cd" => {
							let dir = words.next().unwrap();
							match dir {
								"/" => {
									workdir = 0;
								},
								".." => {
									match &file_system[workdir] {
										Entry::Directory { parent, entries } => workdir = *parent,
										_ => return Err(format!("cd .. requires directoy"))
									}
								},
								_ => {
									let entry = Entry::Directory {
										parent: workdir,
										entries: Vec::with_capacity(16)
									};
									workdir = file_system.len();
									file_system.push(entry);
								}
							}
						},
						"ls" => {},
						_ => return Err(format!("invalid command: {command}"))
					}
				},
				"dir" => {
					let dir = words.next().unwrap();
				},
				_ => {
					let filesize;
					let parse_result = word.parse::<usize>();
					match parse_result {
						Ok(integer) => filesize = integer,
						_ => return Err(format!("size must be an integer but was {word}"))
					}
					let file = Entry::File { size: filesize };
					let node = file_system.len();
					let dir = &mut file_system[workdir];
					match dir {
						Entry::Directory { parent, entries } => {
							entries.push(node); // link file to directory
							file_system.push(file); // add file to sytem
						},
						_ => return Err(format!("file requires directoy"))
					}
				}
			}
		}

		get_size(&file_system, 0, &mut total_size);

		let total_size = total_size.try_into().unwrap();
		Ok((total_size, position))
	}
}

fn get_size(file_system: &Vec<Entry>, node: usize, total_size: &mut usize) -> usize {
	let entry = &file_system[node];
	match entry {
		Entry::Directory { parent, entries } => {
			let mut dirsize = 0;
			for child_node in entries {
				dirsize += get_size(file_system, *child_node, total_size);
			}
			if dirsize <= 100000 {
				*total_size += dirsize;
			}
			dirsize
		},
		Entry::File { size } => *size
	}
}
