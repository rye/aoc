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

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let seats: Vec<Vec<Cell>> = data
		.lines()
		.map(|line| line.chars().map(|c| Cell::from(c)).collect())
		.collect();

	// dbg!(seats);

	{
		let mut seats = seats.clone();
		let mut day = 0;

		let height = seats.len();
		let width = seats[0].len();

		loop {
			let positions: Vec<(usize, usize)> = (0..seats.len())
				.map(|y| (0..seats[y].len()).map(move |x| (x, y)))
				.flatten()
				.collect();

			// Each empty seat with no adjacent, occupied seats becomes occupied
			let become_occupied: Vec<StateChange> = positions
				.iter()
				.filter(|pos| -> bool { seats[pos.1][pos.0].0 == CellState::Empty })
				.filter(|pos| -> bool {
					let top_left: (usize, usize) = (pos.0.saturating_sub(1), pos.1.saturating_sub(1));
					let bottom_right: (usize, usize) = (pos.0.saturating_add(1), pos.1.saturating_add(1));

					let positions_to_check = (top_left.1..=bottom_right.1)
						.map(|y| (top_left.0..=bottom_right.0).map(move |x| (x, y)))
						.flatten()
						.filter(|pos| pos.1 < seats.len() && pos.0 < seats[pos.1].len())
						.filter(|pos_i| &pos_i != pos);

					let occupied_adjacents: Vec<(usize, usize)> = positions_to_check
						.filter(|pos| seats[pos.1][pos.0].0 == CellState::Occupied)
						.collect();

					occupied_adjacents.len() == 0
				})
				.map(|pos| -> StateChange {
					let position: (usize, usize) = *pos;
					let old_state: CellState = seats[position.1][position.0].0;
					assert_eq!(old_state, CellState::Empty);
					let new_state: CellState = CellState::Occupied;

					StateChange {
						position,
						old_state,
						new_state,
					}
				})
				.collect();

			println!(
				"Day {}, Phase 1: {} seats will become occupied",
				day,
				become_occupied.len()
			);

			// Each occupied seat with four or more adjacent seats becomes empty
			let become_emptied: Vec<StateChange> = positions
				.iter()
				.filter(|pos| -> bool { seats[pos.1][pos.0].0 == CellState::Occupied })
				.filter(|pos| -> bool {
					let top_left: (usize, usize) = (pos.0.saturating_sub(1), pos.1.saturating_sub(1));
					let bottom_right: (usize, usize) = (pos.0.saturating_add(1), pos.1.saturating_add(1));

					let positions_to_check = (top_left.1..=bottom_right.1)
						.map(|y| (top_left.0..=bottom_right.0).map(move |x| (x, y)))
						.flatten()
						.filter(|pos| pos.1 < seats.len() && pos.0 < seats[pos.1].len())
						.filter(|pos_i| &pos_i != pos);

					let occupied_adjacents: Vec<(usize, usize)> = positions_to_check
						.filter(|pos| seats[pos.1][pos.0].0 == CellState::Occupied)
						.collect();

					occupied_adjacents.len() >= 4
				})
				.map(|pos| -> StateChange {
					let position: (usize, usize) = *pos;
					let old_state: CellState = seats[position.1][position.0].0;
					assert_eq!(old_state, CellState::Occupied);
					let new_state: CellState = CellState::Empty;

					StateChange {
						position,
						old_state,
						new_state,
					}
				})
				.collect();

			println!(
				"Day {}, Phase 2: {} seats will become emptied",
				day,
				become_emptied.len()
			);

			if become_occupied.is_empty() && become_emptied.is_empty() {
				break;
			} else {
				for change in become_occupied {
					let pos = change.position;
					assert_eq!(seats[pos.1][pos.0].0, change.old_state);
					seats[pos.1][pos.0].0 = change.new_state;
				}

				for change in become_emptied {
					let pos = change.position;
					assert_eq!(seats[pos.1][pos.0].0, change.old_state);
					seats[pos.1][pos.0].0 = change.new_state;
				}

				day += 1;
			}
		}

		let occupied_count: usize = seats
			.iter()
			.map(|row| {
				row
					.iter()
					.filter(|seat| seat.0 == CellState::Occupied)
					.count()
			})
			.sum();

		println!("Part One: {:?}", occupied_count);
	}

	{
		println!("Part Two: {:?}", ());
	}
}
