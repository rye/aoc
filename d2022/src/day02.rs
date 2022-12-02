use core::{convert::Infallible, str::FromStr};

pub enum Move {
	Rock,
	Paper,
	Scissors,
}

impl FromStr for Move {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"X" => Ok(Self::Rock),
			"Y" => Ok(Self::Paper),
			"Z" => Ok(Self::Scissors),
			"A" => Ok(Self::Rock),
			"B" => Ok(Self::Paper),
			"C" => Ok(Self::Scissors),
			_ => unreachable!(),
		}
	}
}

pub struct StrategyPart(Move, Move);

impl FromStr for StrategyPart {
	type Err = Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut s = s.split(' ');

		let opponent_play = s.next().expect("no opponent play?");
		let my_play = s.next().expect("no self play?");

		let opponent_play: Move = opponent_play.parse().expect("unrecognized?");
		let my_play: Move = my_play.parse().expect("unrecognized?");

		Ok(Self(opponent_play, my_play))
	}
}

pub type Intermediate = Vec<StrategyPart>;
pub type Output = u32;

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(
		str
			.lines()
			.map(str::parse)
			.collect::<Result<Intermediate, _>>()?,
	)
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
