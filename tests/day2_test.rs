use adventofcode::Puzzle;
use adventofcode::day2::Day2;

#[test]
fn test() {
	let day = Day2::new();
	assert_eq!(Ok((1606483, 3842356)), day.solve());
}
