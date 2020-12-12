#![allow(unused_imports)]

use std::io::{stdin, Read};
use std::{collections::*, str::FromStr};

use regex::Regex;

use d2020::day11::*;

#[derive(Debug, Copy, Clone, PartialEq)]
enum CellState {
	Floor,
	Empty,
	Occupied,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cell(CellState);

impl From<char> for Cell {
	fn from(c: char) -> Self {
		match c {
			'.' => Cell(CellState::Floor),
			'L' => Cell(CellState::Empty),
			'#' => Cell(CellState::Occupied),
			_ => todo!(),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct StateChange {
	position: (usize, usize),
	old_state: CellState,
	new_state: CellState,
}

#[derive(Clone)]
struct Seating {
	width: usize,
	height: usize,
	seats: Vec<Vec<Cell>>,
}

impl<'x> core::iter::FromIterator<&'x str> for Seating {
	fn from_iter<T: IntoIterator<Item = &'x str>>(iter: T) -> Self {
		let lines: Vec<&str> = iter.into_iter().collect();

		let height = lines.len();
		let width = lines[0].len();

		let seats: Vec<Vec<Cell>> = lines
			.iter()
			.map(|line| line.chars().map(|c| Cell::from(c)).collect())
			.collect();

		Self {
			width,
			height,
			seats,
		}
	}
}

impl Seating {
	fn has_seat(&self, x: usize, y: usize) -> bool {
		(0..self.width).contains(&x) && (0..self.height).contains(&y)
	}

	fn seat_state(&self, x: usize, y: usize) -> Option<&CellState> {
		self
			.seats
			.get(y)
			.map(|row| row.get(x).map(|cell| &cell.0))
			.flatten()
	}

	fn seat_positions<'x>(&'x self) -> impl Iterator<Item = (usize, usize)> + 'x {
		(0..self.height)
			.map(move |y| (0..self.width).map(move |x| (x, y)))
			.flatten()
	}

	fn change_seat_state(&mut self, x: usize, y: usize, new_state: CellState) {
		assert!(self.has_seat(x, y));
		self.seats[y][x].0 = new_state;
	}

	fn seats_with_state<'x>(
		&'x self,
		state: &'x CellState,
	) -> impl Iterator<Item = (usize, usize)> + 'x {
		self
			.seat_positions()
			.filter(move |pos| self.seat_state(pos.0, pos.1) == Some(state))
	}
}

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let seats: Seating = data.lines().collect();

	{
		let mut seats = seats.clone();
		let mut round = 0;

		loop {
			let t0 = std::time::Instant::now();

			// Each empty seat with no adjacent, occupied seats becomes occupied
			let become_occupied: Vec<StateChange> = seats
				.seats_with_state(&CellState::Empty)
				.filter(|pos| -> bool {
					let top_left: (usize, usize) = (pos.0.saturating_sub(1), pos.1.saturating_sub(1));
					let bottom_right: (usize, usize) = (pos.0.saturating_add(1), pos.1.saturating_add(1));

					let positions_to_check = (top_left.1..=bottom_right.1)
						.map(|y| (top_left.0..=bottom_right.0).map(move |x| (x, y)))
						.flatten()
						.filter(|pos| seats.has_seat(pos.0, pos.1))
						.filter(|pos_i| pos_i != pos);

					let occupied_adjacents: Vec<(usize, usize)> = positions_to_check
						.filter(|pos| seats.seat_state(pos.0, pos.1) == Some(&CellState::Occupied))
						.collect();

					occupied_adjacents.len() == 0
				})
				.map(|pos| -> StateChange {
					let position: (usize, usize) = pos;
					let old_state: &CellState = seats.seat_state(pos.0, pos.1).unwrap();
					assert_eq!(old_state, &CellState::Empty);
					let new_state: CellState = CellState::Occupied;

					StateChange {
						position,
						old_state: *old_state,
						new_state,
					}
				})
				.collect();

			println!(
				"Round {}, Phase 1: {} seats will become occupied",
				round,
				become_occupied.len()
			);

			let t1 = std::time::Instant::now();

			// Each occupied seat with four or more adjacent seats becomes empty
			let become_emptied: Vec<StateChange> = seats
				.seats_with_state(&CellState::Occupied)
				.filter(|pos| -> bool {
					let top_left: (usize, usize) = (pos.0.saturating_sub(1), pos.1.saturating_sub(1));
					let bottom_right: (usize, usize) = (pos.0.saturating_add(1), pos.1.saturating_add(1));

					let positions_to_check = (top_left.1..=bottom_right.1)
						.map(|y| (top_left.0..=bottom_right.0).map(move |x| (x, y)))
						.flatten()
						.filter(|pos| seats.has_seat(pos.0, pos.1))
						.filter(|pos_i| pos_i != pos);

					let occupied_adjacents: Vec<(usize, usize)> = positions_to_check
						.filter(|pos| seats.seat_state(pos.0, pos.1) == Some(&CellState::Occupied))
						.collect();

					occupied_adjacents.len() >= 4
				})
				.map(|pos| -> StateChange {
					let position: (usize, usize) = pos;
					let old_state: &CellState = seats.seat_state(pos.0, pos.1).unwrap();
					assert_eq!(old_state, &CellState::Occupied);
					let new_state: CellState = CellState::Empty;

					StateChange {
						position,
						old_state: *old_state,
						new_state,
					}
				})
				.collect();

			println!(
				"Round {}, Phase 2: {} seats will become emptied",
				round,
				become_emptied.len()
			);

			let t2 = std::time::Instant::now();

			println!(
				"Completed planning in {}ns ({}ns, {}ns)",
				t2.duration_since(t0).as_nanos(),
				t1.duration_since(t0).as_nanos(),
				t2.duration_since(t1).as_nanos(),
			);

			if become_occupied.is_empty() && become_emptied.is_empty() {
				break;
			} else {
				for change in become_occupied {
					let pos = change.position;
					seats.change_seat_state(pos.0, pos.1, change.new_state);
				}

				for change in become_emptied {
					let pos = change.position;
					seats.change_seat_state(pos.0, pos.1, change.new_state);
				}

				println!(
					"Completed changes in {}ns",
					std::time::Instant::now().duration_since(t2).as_nanos()
				);

				round += 1;
			}
		}

		let occupied_count: usize = seats.seats_with_state(&CellState::Occupied).count();

		println!("Part One: {:?}", occupied_count);
	}

	{
		println!("Part Two: {:?}", ());
	}
}
