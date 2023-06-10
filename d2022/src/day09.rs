use core::{
	convert::Infallible,
	fmt::{self, Display, Formatter},
	iter::repeat,
	str::FromStr,
};

use std::collections::BTreeMap;

pub type Intermediate = Vec<Move>;
pub type Output = usize;

pub struct State {
	start: (i32, i32),
	rope: Rope,
	tail_pos: BTreeMap<(i32, i32), usize>,
}

#[derive(Default)]
struct StateDisplay {
	min: (i32, i32),
	max: (i32, i32),
	symbols: BTreeMap<(i32, i32), String>,
}

impl StateDisplay {
	fn add_symbol(&mut self, position: (i32, i32), symbol: String) {
		if position.0 - 1 < self.min.0 {
			self.min.0 = position.0 - 1;
		}
		if position.1 - 1 < self.min.1 {
			self.min.1 = position.1 - 1;
		}
		if position.0 + 1 > self.max.0 {
			self.max.0 = position.0 + 1;
		}
		if position.1 + 1 > self.max.1 {
			self.max.1 = position.1 + 1;
		}

		self.symbols.insert(position, symbol);
	}
}

impl Display for StateDisplay {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		debug_assert!(self.min.0 < self.max.0);
		debug_assert!(self.min.1 < self.max.1);

		for y in self.min.1..=self.max.1 {
			for x in self.min.0..=self.max.0 {
				let sym = self.symbols.get(&(x, y));
				match sym {
					Some(s) => write!(f, "{}", s),
					None => write!(f, " "),
				}?;
			}
			writeln!(f, "")?;
		}
		Ok(())
	}
}

impl Display for State {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut display = StateDisplay::default();

		// seen tail positions are always on the bottom
		for pos in self.rope.tail_history.keys() {
			display.add_symbol(*pos, "*".to_string());
		}

		// start is next
		display.add_symbol(self.start, "s".to_string());

		// rope positions are next...
		for (idx, pos) in self.rope.positions.iter().enumerate() {
			display.add_symbol(*pos, (idx % 10).to_string());
		}

		// tail after that
		display.add_symbol(*self.rope.tail(), "T".to_string());

		// head is always last, therefore on top.
		display.add_symbol(*self.rope.head(), "H".to_string());

		writeln!(f, "{}", display)

		// todo!()
	}
}

struct Rope {
	positions: Vec<(i32, i32)>,
	tail_history: BTreeMap<(i32, i32), usize>,
}

impl Rope {
	fn with_length(at_pos: (i32, i32), length: usize) -> Self {
		let positions = repeat(at_pos).take(length).collect();

		let tail_history = BTreeMap::default();

		Self {
			positions,
			tail_history,
		}
	}

	fn head(&self) -> &(i32, i32) {
		self.positions.first().expect("missing first position")
	}

	fn tail(&self) -> &(i32, i32) {
		self.positions.last().expect("missing first position")
	}

	fn apply_move(&mut self, mv: &Move) {
		use core::cell::Cell;

		let step_vec: (i32, i32) = (&mv.direction).into();

		let position_cells: &[Cell<(i32, i32)>] =
			Cell::from_mut(&mut self.positions[..]).as_slice_of_cells();

		for _n in 0..mv.distance {
			// Step 1: Move the head.
			if let Some(head) = position_cells.first() {
				let new_head = (head.get().0 + step_vec.0, head.get().1 + step_vec.1);
				Cell::set(head, new_head);
			}

			// Step 2: Cascade all changes down.
			for window in position_cells.windows(2) {
				if let Some(nudge) = Rope::pair_nudge(&window[0].get(), &window[1].get()) {
					let mut new_pos = window[1].get();
					new_pos.0 += nudge.0;
					new_pos.1 += nudge.1;
					Cell::set(&window[1], new_pos);
				}
			}

			// Step 3: Record tail history.
			if let Some(tail) = position_cells.last() {
				*self.tail_history.entry(tail.get()).or_insert(1) += 1;
			}
		}
	}

	fn positions(&self) -> impl Iterator<Item = &(i32, i32)> {
		self.positions.iter()
	}

	fn pair_nudge(head: &(i32, i32), tail: &(i32, i32)) -> Option<(i32, i32)> {
		match (head.0 - tail.0, head.1 - tail.1) {
			// A total of 9 possibilities require no nudge.
			(-1, -1) => None,
			(0, -1) => None,
			(1, -1) => None,
			(-1, 0) => None,
			(0, 0) => None,
			(1, 0) => None,
			(-1, 1) => None,
			(0, 1) => None,
			(1, 1) => None,

			// Cardinal direction overextensions always nudge in the same direction
			// to close the gap.
			(0, -2) => Some((0, -1)),
			(0, 2) => Some((0, 1)),
			(-2, 0) => Some((-1, 0)),
			(2, 0) => Some((1, 0)),

			//
			(-1, -2) => Some((-1, -1)),
			(-2, -1) => Some((-1, -1)),
			(1, -2) => Some((1, -1)),
			(2, -1) => Some((1, -1)),
			(-2, 1) => Some((-1, 1)),
			(-1, 2) => Some((-1, 1)),
			(2, 1) => Some((1, 1)),
			(1, 2) => Some((1, 1)),

			(-2, -2) => Some((-1, -1)),
			(-2, 2) => Some((-1, 1)),
			(2, -2) => Some((1, -1)),
			(2, 2) => Some((1, 1)),

			(x, y) => todo!("{x} {y} has no nudge"),
		}
	}
}

impl State {
	fn apply_move(&mut self, mv: &Move) {
		self.rope.apply_move(mv);
		self
			.tail_pos
			.entry(self.rope.tail().to_owned())
			.or_insert(1);
	}
}

pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl From<&Direction> for (i32, i32) {
	fn from(value: &Direction) -> Self {
		match value {
			Direction::Up => (0, -1),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}
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
		tail_pos: BTreeMap::default(),
	};

	for r#move in moves {
		// println!("== {move} ==");
		state.apply_move(r#move);
	}

	Some(state.rope.tail_history.len())
}

#[must_use]
pub fn part_two(moves: &Intermediate) -> Option<Output> {
	let mut state = State {
		start: (0, 0),
		rope: Rope::with_length((0, 0), 10),
		tail_pos: BTreeMap::default(),
	};

	// println!("== Initial State ==");

	// print!("{}", state);

	for r#move in moves {
		// println!("== {move} ==");
		state.apply_move(r#move);
		// print!("{}", state);
	}

	Some(state.rope.tail_history.len())
}

daocutil::test_example!(
	part_two_simple,
	parse,
	part_two,
	"R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2",
	Some(1)
);

daocutil::test_example!(
	part_two_larger,
	parse,
	part_two,
	"R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20",
	Some(36)
);
