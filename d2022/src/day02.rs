use core::{convert::Infallible, str::FromStr};

pub enum Move {
	Rock,
	Paper,
	Scissors,
}

impl Move {
	fn shape_score(&self) -> u32 {
		match self {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissors => 3,
		}
	}
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

enum Outcome {
	Win,
	Draw,
	Loss,
}

impl Outcome {
	fn outcome_score(&self) -> u32 {
		match self {
			Outcome::Win => 6,
			Outcome::Draw => 3,
			Outcome::Loss => 0,
		}
	}
}

impl StrategyPart {
	fn self_score(&self) -> u32 {
		let self_move = &self.1;
		let opponent_move = &self.0;

		let outcome = match (self_move, opponent_move) {
			(Move::Rock, Move::Rock) => Outcome::Draw,
			(Move::Rock, Move::Paper) => Outcome::Loss,
			(Move::Rock, Move::Scissors) => Outcome::Win,
			(Move::Paper, Move::Rock) => Outcome::Win,
			(Move::Paper, Move::Paper) => Outcome::Draw,
			(Move::Paper, Move::Scissors) => Outcome::Loss,
			(Move::Scissors, Move::Rock) => Outcome::Loss,
			(Move::Scissors, Move::Paper) => Outcome::Win,
			(Move::Scissors, Move::Scissors) => Outcome::Draw,
		};

		self_move.shape_score() + outcome.outcome_score()
	}
}

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
pub fn part_one(guide: &Intermediate) -> Option<Output> {
	Some(guide.iter().map(|guide_move| guide_move.self_score()).sum())
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
