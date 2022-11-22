use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Advent of Code");
	day1();
}

fn day1() {
	println!("Day 1: Not Quite Lisp");
	let file = File::open("day1.in").expect("open input failed");
	let mut reader = BufReader::new(file);
	let mut line = String::new();
	reader.read_line(&mut line).expect("read line failed");
	println!("{line}");
}