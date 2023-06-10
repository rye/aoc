use core::{convert::Infallible, str::FromStr};

pub type Intermediate = (State, Vec<Move>);
pub type Output = u32;

pub struct State {}

impl Default for State {
	fn default() -> Self {
		Self {}
	}
}

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl FromStr for Direction {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"U" => Ok(Self::Up),
			"D" => Ok(Self::Down),
			"L" => Ok(Self::Left),
			"R" => Ok(Self::Right),
			_ => unreachable!(),
		}
	}
}

mod r#move;
pub use r#move::*;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let moves = input
		.lines()
		.map(str::parse)
		.collect::<Result<Vec<Move>, _>>()?;

	Ok((State::default(), moves))
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
