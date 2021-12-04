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

struct Turn<'a, 'b: 'a> {
	call: Number,
	winners: Vec<&'a Board>,
	calls_so_far: &'b [Number],
}

fn record_all_rounds<'a, 'b>(calls: &'b [Number], boards: &'b [Board]) -> Vec<Turn<'a, 'b>> {
	let mut seen_calls: BTreeSet<Number> = BTreeSet::new();

	let mut remaining_boards: HashSet<&Board> = boards.iter().collect();

	calls
		.iter()
		.enumerate()
		.map(|(idx, &call)| {
			seen_calls.insert(call);

			let mut winners: Vec<&'b Board> = vec![];

			for &board in &remaining_boards {
				if board.numbers().intersection(&seen_calls).count() < 5 {
					continue;
				} else if let Some(_winning_move) = board.find_winning_move(&seen_calls) {
					winners.push(board);
				}
			}

			for &board in winners.iter() {
				remaining_boards.remove(board);
			}

			let calls_so_far = &calls[0..=idx];

			Turn {
				call,
				winners,
				calls_so_far,
			}
		})
		.collect()
}

pub fn part_two((calls, boards): &Intermediate) -> Option<Solution> {
	let all_turns = record_all_rounds(calls, boards);

	let (&last_call, winning_board, seen_calls) = all_turns
		.iter()
		.rfind(|Turn { winners, .. }| !winners.is_empty())
		.map(|turn| {
			let Turn {
				call,
				winners,
				calls_so_far,
				..
			} = turn;

			(call, winners[0], calls_so_far.iter().copied().collect())
		})
		.expect("no last winner?!");

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}
