use core::{convert::Infallible, str::FromStr};

#[derive(Default)]
pub struct State {
	start: (i32, i32),
	head_pos: (i32, i32),
	tail_pos: (i32, i32),
}

fn calculate_tail_nudge(head_pos: &(i32, i32), tail_pos: &(i32, i32)) -> Option<(i32, i32)> {
	// This helper should be called for every new head_pos, and its return value, if Some, should be
	// added componentwise to the existing tail_pos to get the new tail_pos.
	//
	// As indicated by the below illustration, there are a grand total of 5^2 - 4 = 21 cases we have
	// a defined movement for, which break down to be:
	//
	// - 9 cases (indicated by the lowercase letters) where no movement is necessary as the new
	//   head_pos and tail_pos are already connected.
	//
	// - 12 cases (indicated by uppercase letters) where the tail does need to move into one of the
	//   `o` positions.
	//
	// .BAB.
	// BoooB
	// AotoA
	// BoooB
	// .BAB.
	//
	// There is an implicit assumption here that the head can only move in the 4 cardinal directions,
	// which is important because the outer corners (the final 4 positions in the 5^2 grid) are not
	// mapped to anything.

	match (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1) {
		// If the head is in one of the `o` positions, the tail is connected so does not need to move.
		(-1..=1, -1..=1) => Some((0, 0)),

		// The head has moved and the tail just needs to move in the same row or column. (`A` cases)
		(2, 0) => Some((1, 0)),
		(-2, 0) => Some((-1, 0)),
		(0, 2) => Some((0, 1)),
		(0, -2) => Some((0, -1)),

		// The head has moved and the tail needs to move to one of the corners. (`B` cases)
		(2, 1) | (1, 2) => Some((1, 1)),
		(2, -1) | (1, -2) => Some((1, -1)),
		(-2, -1) | (-1, -2) => Some((-1, -1)),
		(-2, 1) | (-1, 2) => Some((-1, 1)),

		// As a fallback, we have no idea what to do, so return no nudge and let the caller report
		// that as they wish.
		_ => None,
	}
}

#[cfg(test)]
mod calculate_tail_nudge {
	use super::calculate_tail_nudge;

	#[test]
	fn overlapping() {
		assert_eq!(calculate_tail_nudge(&(0, 0), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_right() {
		assert_eq!(calculate_tail_nudge(&(1, 0), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_left() {
		assert_eq!(calculate_tail_nudge(&(-1, 0), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_above() {
		assert_eq!(calculate_tail_nudge(&(0, 1), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_below() {
		assert_eq!(calculate_tail_nudge(&(0, -1), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_below_left() {
		assert_eq!(calculate_tail_nudge(&(-1, -1), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_below_right() {
		assert_eq!(calculate_tail_nudge(&(1, -1), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_above_left() {
		assert_eq!(calculate_tail_nudge(&(-1, 1), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn connected_above_right() {
		assert_eq!(calculate_tail_nudge(&(1, 1), &(0, 0)), Some((0, 0)));
	}

	#[test]
	fn disconnected_above() {
		assert_eq!(calculate_tail_nudge(&(0, 2), &(0, 0)), Some((0, 1)));
	}

	#[test]
	fn disconnected_below() {
		assert_eq!(calculate_tail_nudge(&(0, -2), &(0, 0)), Some((0, -1)));
	}

	#[test]
	fn disconnected_left() {
		assert_eq!(calculate_tail_nudge(&(-2, 0), &(0, 0)), Some((-1, 0)));
	}

	#[test]
	fn disconnected_right() {
		assert_eq!(calculate_tail_nudge(&(2, 0), &(0, 0)), Some((1, 0)));
	}

	#[test]
	fn disconnected_above_right() {
		assert_eq!(calculate_tail_nudge(&(2, 1), &(0, 0)), Some((1, 1)));
		assert_eq!(calculate_tail_nudge(&(1, 2), &(0, 0)), Some((1, 1)));
	}

	#[test]
	fn disconnected_below_right() {
		assert_eq!(calculate_tail_nudge(&(2, -1), &(0, 0)), Some((1, -1)));
		assert_eq!(calculate_tail_nudge(&(1, -2), &(0, 0)), Some((1, -1)));
	}

	#[test]
	fn disconnected_below_left() {
		assert_eq!(calculate_tail_nudge(&(-2, -1), &(0, 0)), Some((-1, -1)));
		assert_eq!(calculate_tail_nudge(&(-1, -2), &(0, 0)), Some((-1, -1)));
	}

	#[test]
	fn disconnected_above_left() {
		assert_eq!(calculate_tail_nudge(&(-2, 1), &(0, 0)), Some((-1, 1)));
		assert_eq!(calculate_tail_nudge(&(-1, 2), &(0, 0)), Some((-1, 1)));
	}

	#[test]
	fn disconnected_no_mapping() {
		assert_eq!(calculate_tail_nudge(&(42, 1), &(0, 0)), None);
	}
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
