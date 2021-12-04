use std::{collections::BTreeSet, num::ParseIntError};

type Number = u8;

#[derive(Debug)]
pub struct Board {
	//contents: [[Number; 5]; 5],
	winning_moves: Vec<BTreeSet<Number>>,
	all_contents: BTreeSet<Number>,
}

impl Board {
	fn generate_winning_moves(contents: &[[Number; 5]; 5]) -> Vec<BTreeSet<Number>> {
		let mut winning_moves = Vec::new();

		for &row in contents {
			let rank_set: BTreeSet<Number> = BTreeSet::from(row);
			winning_moves.push(rank_set);
		}

		for column in 0..5 {
			let mut file_set: BTreeSet<Number> = BTreeSet::new();

			for &row in contents.iter() {
				file_set.insert(row[column]);
			}

			winning_moves.push(file_set);
		}

		winning_moves
	}

	fn find_winning_move(&self, seen_calls: &BTreeSet<Number>) -> Option<&BTreeSet<Number>> {
		self
			.winning_moves
			.iter()
			.find(|winning_move| seen_calls.is_superset(winning_move))
	}

	fn from_contents(contents: [[Number; 5]; 5]) -> Self {
		let winning_moves = Self::generate_winning_moves(&contents);

		let all_contents: BTreeSet<Number> = contents
			.iter()
			.flat_map(|row| row.iter().copied())
			.collect();

		Self {
			//contents,
			winning_moves,
			all_contents,
		}
	}
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
	#[error("failed to parse a number")]
	IntegerParse(#[from] ParseIntError),

	#[error("failed to parse board")]
	BoardParse,
}

fn parse_board_line(line: &str) -> Result<[Number; 5], Error> {
	let result: Result<Vec<Number>, Error> = line
		.split_ascii_whitespace()
		.map(str::parse)
		.collect::<Result<Vec<Number>, _>>()
		.map_err(Error::from);

	match result {
		Ok(vec) => <[Number; 5]>::try_from(vec).or_else(|_| Err(Error::BoardParse)),
		Err(e) => Err(e),
	}
}

fn parse_board(board: &str) -> Result<[[Number; 5]; 5], Error> {
	let result: Result<Vec<[Number; 5]>, Error> = board
		.split('\n')
		.map(parse_board_line)
		.collect::<Result<Vec<[Number; 5]>, Error>>();

	match result {
		Ok(vec) => <[[Number; 5]; 5]>::try_from(vec).or_else(|_| Err(Error::BoardParse)),
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod parse_board_line {
	use super::{parse_board_line, Error};

	#[test]
	fn line_62_5_77_94_75() {
		assert_eq!(parse_board_line("62  5 77 94 75"), Ok([62, 5, 77, 94, 75]));
	}

	#[test]
	fn line_62_5_77_94() {
		assert_eq!(parse_board_line("62  5 77 94"), Err(Error::BoardParse));
	}

	#[test]
	fn line_62_5_77_94_asdf() {
		assert!(parse_board_line("62  5 77 94 asdf").is_err())
	}
}

impl core::str::FromStr for Board {
	type Err = Error;

	fn from_str(board: &str) -> Result<Self, Self::Err> {
		parse_board(board).map(Board::from_contents)
	}
}

type Intermediate = (Vec<Number>, Vec<Board>);

pub fn parse(input: &str) -> Intermediate {
	let mut results = input.split("\n\n");
	let calls = results.next().unwrap();
	let boards = results;

	let calls: Vec<Number> = calls
		.split(',')
		.map(str::parse)
		.filter_map(Result::ok)
		.collect();

	let boards: Vec<Board> = boards.map(str::parse).filter_map(Result::ok).collect();

	(calls, boards)
}

type Solution = usize;

pub fn part_one((calls, boards): &Intermediate) -> Option<Solution> {
	let mut seen_calls = BTreeSet::new();

	let mut calls = calls.iter();

	let winning_board: Option<(Number, &Board)> = 'main: loop {
		let call = calls.next();

		if let Some(&call) = call {
			seen_calls.insert(call);

			for board in boards {
				if board.all_contents.intersection(&seen_calls).count() < 5 {
					continue;
				} else if let Some(_winning_move) = board.find_winning_move(&seen_calls) {
					break 'main Some((call, board));
				}
			}
		} else {
			break None;
		}
	};

	let (last_call, winning_board) = winning_board.expect("failed to find a winning board");

	let score: u32 = winning_board
		.all_contents
		.difference(&seen_calls)
		.map(|n| *n as u32)
		.sum::<u32>()
		* last_call as u32;

	Some(score as usize)
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
