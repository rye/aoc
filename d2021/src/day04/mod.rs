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

struct Turn<'a> {
	call: Number,
	winners: Vec<&'a Board>,
	calls_so_far: &'a [Number],
}

fn record_all_rounds<'a>(
	calls: &'a [Number],
	boards: &'a [Board],
) -> impl Iterator<Item = Turn<'a>> {
	calls.iter().enumerate().scan(
		(boards.iter().collect(), BTreeSet::new()),
		|(remaining_boards, seen_calls): &mut (HashSet<&Board>, BTreeSet<Number>), (idx, &call)| {
			seen_calls.insert(call);

			let mut winners: Vec<&'a Board> = vec![];

			for &board in remaining_boards.iter() {
				if board.numbers().intersection(&seen_calls).count() < 5 {
					continue;
				} else if let Some(_winning_move) = board.find_winning_move(&seen_calls) {
					winners.push(board);
				}
			}

			for &board in winners.iter() {
				remaining_boards.remove(board);
			}

			let calls_so_far: &'a [Number] = &calls[0..=idx];

			Some(Turn {
				call,
				winners,
				calls_so_far,
			})
		},
	)
}

type Solution = usize;

pub fn part_one((calls, boards): &Intermediate) -> Option<Solution> {
	let Turn {
		call,
		winners,
		calls_so_far,
	} = record_all_rounds(calls, boards)
		.find(|turn| !turn.winners.is_empty())
		.unwrap();

	let last_call = call;
	let winning_board = winners[0];
	let seen_calls = calls_so_far.iter().copied().collect();

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}

pub fn part_two((calls, boards): &Intermediate) -> Option<Solution> {
	let all_turns: Vec<Turn> = record_all_rounds(calls, boards).collect();

	let last_turn = all_turns
		.iter()
		.rfind(|Turn { winners, .. }| !winners.is_empty())
		.unwrap();

	let Turn {
		call,
		winners,
		calls_so_far,
	} = last_turn;

	let &last_call = call;
	let winning_board = winners[0];
	let seen_calls = calls_so_far.iter().copied().collect();

	let score: u32 = winning_board.score(last_call, &seen_calls);

	Some(score as usize)
}
