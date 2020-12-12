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
	state: CellState,
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

	fn adjacents<'x>(&'x self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'x {
		(y.saturating_sub(1)..core::cmp::min(y.saturating_add(2), self.height))
			.map(move |iy| {
				(x.saturating_sub(1)..core::cmp::min(x.saturating_add(2), self.width))
					.map(move |ix| (ix, iy))
			})
			.flatten()
			.filter(move |pos| pos != &(x, y))
	}

	fn adjacents_with_state<'x>(
		&'x self,
		x: usize,
		y: usize,
		state: &'x CellState,
	) -> impl Iterator<Item = (usize, usize)> + 'x {
		self
			.adjacents(x, y)
			.filter(move |adjacent| self.seat_state(adjacent.0, adjacent.1) == Some(state))
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
					seats
						.adjacents_with_state(pos.0, pos.1, &CellState::Occupied)
						.count() == 0
				})
				.map(|pos| -> StateChange {
					let position: (usize, usize) = pos;
					let state: CellState = CellState::Occupied;

					StateChange { position, state }
				})
				.collect();

			let t1 = std::time::Instant::now();

			// Each occupied seat with four or more adjacent seats becomes empty
			let become_emptied: Vec<StateChange> = seats
				.seats_with_state(&CellState::Occupied)
				.filter(|pos| -> bool {
					seats
						.adjacents_with_state(pos.0, pos.1, &CellState::Occupied)
						.count() >= 4
				})
				.map(|pos| -> StateChange {
					let position: (usize, usize) = pos;
					let state: CellState = CellState::Empty;

					StateChange { position, state }
				})
				.collect();

			let t2 = std::time::Instant::now();

			println!(
				"Round {}: {} will become Occupied, {} will become Empty",
				round,
				become_occupied.len(),
				become_emptied.len(),
			);

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
					seats.change_seat_state(pos.0, pos.1, change.state);
				}

				for change in become_emptied {
					let pos = change.position;
					seats.change_seat_state(pos.0, pos.1, change.state);
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
