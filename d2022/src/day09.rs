use core::{convert::Infallible, iter::repeat, str::FromStr};
use std::collections::HashSet;

pub type Intermediate = Vec<Move>;
pub type Output = u32;

pub struct State {
	start: (i32, i32),
	rope: Rope,
	tail_pos: HashSet<(i32, i32)>,
}

struct Rope {
	positions: Vec<(i32, i32)>,
}

impl Rope {
	fn with_length(at_pos: (i32, i32), length: usize) -> Self {
		let positions = repeat(at_pos).take(length).collect();
		Self { positions }
	}

	fn head(&self) -> &(i32, i32) {
		self.positions.first().expect("missing first position")
	}

	fn tail(&self) -> &(i32, i32) {
		self.positions.last().expect("missing first position")
	}

	fn apply_move(&mut self, mv: &Move) {}
}

impl State {
	fn apply_move(&mut self, mv: &Move) {
		self.rope.apply_move(mv);
		self.tail_pos.insert(self.rope.tail().to_owned());
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

	Ok(moves)
}

#[must_use]
pub fn part_one(moves: &Intermediate) -> Option<Output> {
	let mut state = State {
		start: (0, 0),
		rope: Rope::with_length((0, 0), 2),
		tail_pos: HashSet::default(),
	};

	for r#move in moves {
		state.apply_move(r#move)
	}

	Some(state.tail_pos.len() as u32)
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
