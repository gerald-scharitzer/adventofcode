use adventofcode::Puzzle;
use adventofcode::day1::Day1;

#[test]
fn test() {
    let day = Day1::new();
    assert_eq!(Ok((138, 1771)), day.solve());
}
