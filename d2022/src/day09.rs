use core::{convert::Infallible, str::FromStr};

#[derive(Default)]
pub struct State {
	start: (i32, i32),
	head_pos: (i32, i32),
	tail_pos: (i32, i32),
}

enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl FromStr for Move {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let direction = match &s[0..1] {
			"D" => Direction::Down,
			"U" => Direction::Up,
			"L" => Direction::Left,
			"R" => Direction::Right,
			_ => unreachable!(),
		};

		let size: u16 = *&s[2..].parse().expect("failure");

		Ok(Move { direction, size })
	}
}

pub struct Move {
	direction: Direction,
	size: u16,
}

pub type Intermediate = (State, Vec<Move>);
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let state = State::default();

	let moves: Vec<Move> = input
		.lines()
		.map(str::parse)
		.collect::<Result<Vec<Move>, _>>()?;

	Ok((state, moves))
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[test]
fn part_one_example() {
	let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
	daocutil::test_example!(input, part_one, parse, Some(13));
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
