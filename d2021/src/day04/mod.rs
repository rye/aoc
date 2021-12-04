use std::{
	collections::{BTreeSet, HashSet},
	num::ParseIntError,
};

type Number = u8;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
	#[error("failed to parse a number")]
	IntegerParse(#[from] ParseIntError),

	#[error("failed to parse board")]
	BoardParse,
}

mod board;
pub use board::*;

type Intermediate = (Vec<Number>, Vec<Board>);

pub fn parse(input: &str) -> Intermediate {
	let mut results = input.split("\n\n");
	let calls = results.next();
	let boards = results;

	let calls: Vec<Number> = calls
		.unwrap_or("")
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
				if board.numbers().intersection(&seen_calls).count() < 5 {
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

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}

pub fn part_two((calls, boards): &Intermediate) -> Option<Solution> {
	let mut seen_calls = BTreeSet::new();

	let mut remaining_boards: HashSet<&Board> = boards.iter().collect();

	let winning_boards_per_call: Vec<(usize, Number, Vec<&Board>)> = calls
		.iter()
		.enumerate()
		.map(|(idx, &call)| {
			seen_calls.insert(call);

			let mut winners_this_round: Vec<&Board> = vec![];

			for &board in &remaining_boards {
				if board.numbers().intersection(&seen_calls).count() < 5 {
					continue;
				} else if let Some(_winning_move) = board.find_winning_move(&seen_calls) {
					winners_this_round.push(board);
				}
			}

			for &board in winners_this_round.iter() {
				remaining_boards.remove(board);
			}

			(idx, call, winners_this_round)
		})
		.collect();

	let (&last_call, winning_board, seen_calls) = winning_boards_per_call
		.iter()
		.rfind(|(_idx, _call, winners)| !winners.is_empty())
		.map(|(call_idx, last_call, winners)| {
			let calls_to_this_point = &calls[0..=*call_idx];
			let calls_to_this_point: BTreeSet<Number> = calls_to_this_point.iter().copied().collect();

			assert_eq!(
				winners.len(),
				1,
				"should only have one winner on the last day"
			);

			let winning_board = winners[0];

			(last_call, winning_board, calls_to_this_point)
		})
		.expect("no last winner?!");

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}
