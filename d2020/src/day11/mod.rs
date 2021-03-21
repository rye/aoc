use std::collections::HashMap;

pub type Intermediate = Layout;
pub type Solution = usize;

pub fn parse(input: &str) -> Intermediate {
	Layout::parse(input)
}

pub fn part_one(intermediate: &Intermediate) -> Option<Solution> {
	let mut layout = intermediate.clone();

	let mut prev_layout = None;

	while prev_layout.is_none() || layout != prev_layout.unwrap() {
		prev_layout = Some(layout.clone());
		layout = layout.advance(transition);
	}

	Some(
		layout
			.cells
			.values()
			.filter(|cell| **cell == Occupied)
			.count(),
	)
}

pub fn part_two(intermediate: &Intermediate) -> Option<Solution> {
	let mut layout = intermediate.clone();

	let mut prev_layout = None;

	while prev_layout.is_none() || layout != prev_layout.unwrap() {
		prev_layout = Some(layout.clone());
		layout = layout.advance(transition2);
	}

	Some(
		layout
			.cells
			.values()
			.filter(|cell| **cell == Occupied)
			.count(),
	)
}

fn transition(layout: &Layout, coords: Coords, cell: &Cell) -> Cell {
	let occupied_neigbhors = layout.occupied_neighbors(coords);

	match cell {
		Floor => Floor,
		Empty => {
			if occupied_neigbhors == 0 {
				Occupied
			} else {
				Empty
			}
		}
		Occupied => {
			if occupied_neigbhors >= 4 {
				Empty
			} else {
				Occupied
			}
		}
	}
}

fn transition2(layout: &Layout, coords: Coords, cell: &Cell) -> Cell {
	let occupied_neigbhors = layout.visible_occupied_neighbors(coords);

	match cell {
		Floor => Floor,
		Empty => {
			if occupied_neigbhors == 0 {
				Occupied
			} else {
				Empty
			}
		}
		Occupied => {
			if occupied_neigbhors >= 5 {
				Empty
			} else {
				Occupied
			}
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
	Floor,
	Empty,
	Occupied,
}

type Coords = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Layout {
	cells: HashMap<Coords, Cell>,
	width: usize,
	height: usize,
}

use Cell::*;

const DELTAS: [(i32, i32); 8] = [
	(-1, -1),
	(-1, 0),
	(-1, 1),
	(0, -1),
	(0, 1),
	(1, -1),
	(1, 0),
	(1, 1),
];

impl Layout {
	fn parse(input: &str) -> Layout {
		let mut cells = HashMap::new();
		let mut width = 0;
		let mut height = 0;

		for (row_idx, line) in input.trim_end().split('\n').enumerate() {
			for (col_idx, ch) in line.chars().enumerate() {
				let cell = match ch {
					'L' => Empty,
					'.' => Floor,
					'#' => Occupied,
					_ => panic!("Unexpected input {}", ch),
				};

				cells.insert((row_idx, col_idx), cell);
				width = width.max(col_idx + 1);
			}
			height = height.max(row_idx + 1);
		}

		Layout {
			cells,
			width,
			height,
		}
	}

	fn new(width: usize, height: usize) -> Layout {
		Layout {
			width,
			height,
			cells: HashMap::new(),
		}
	}

	fn advance<F>(&self, transition: F) -> Layout
	where
		F: Fn(&Layout, Coords, &Cell) -> Cell,
	{
		let mut new_layout = Layout::new(self.width, self.height);

		for row_idx in 0..self.height {
			for col_idx in 0..self.width {
				let coords = (row_idx, col_idx);
				let cell = self.cells.get(&coords).unwrap();

				let new_cell = transition(self, coords, cell);
				new_layout.cells.insert(coords, new_cell);
			}
		}

		new_layout
	}

	fn occupied_neighbors(&self, (row_idx, col_idx): Coords) -> usize {
		DELTAS
			.iter()
			.map(|(row_delta, col_delta)| (row_idx as i32 + row_delta, col_idx as i32 + col_delta))
			.filter_map(|(row_idx, col_idx)| {
				if row_idx >= 0 && col_idx >= 0 {
					self.cells.get(&(row_idx as usize, col_idx as usize))
				} else {
					None
				}
			})
			.filter(|cell| **cell == Occupied)
			.count()
	}

	fn visible_occupied_neighbors(&self, coords: Coords) -> usize {
		DELTAS.iter().fold(0, |count, delta| {
			if let Some(true) = (1..).find_map(|n| {
				if let Some(visible_coords) = self.delta_n(coords, delta, n) {
					match self.cells.get(&visible_coords) {
						Some(Occupied) => Some(true),
						Some(Empty) => Some(false),
						None => Some(false),
						_ => None,
					}
				} else {
					Some(false)
				}
			}) {
				count + 1
			} else {
				count
			}
		})
	}

	fn delta_n(
		&self,
		(row_idx, col_idx): Coords,
		(row_delta, col_delta): &(i32, i32),
		n: i32,
	) -> Option<Coords> {
		let (row_idx, col_idx) = (
			row_idx as i32 + row_delta * n,
			col_idx as i32 + col_delta * n,
		);

		if row_idx >= 0 && col_idx >= 0 && row_idx < self.height as i32 && col_idx < self.width as i32 {
			Some((row_idx as usize, col_idx as usize))
		} else {
			None
		}
	}
}
