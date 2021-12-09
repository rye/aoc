use std::collections::HashSet;

type Intermediate<'a> = Vec<([&'a str; 10], [&'a str; 4])>;

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.map(|line| line.split(" | "))
		.map(|mut split| (split.next().unwrap(), split.next().unwrap()))
		.map(|(signals, outputs)| {
			(
				signals.split(' ').collect::<Vec<_>>().try_into().unwrap(),
				outputs.split(' ').collect::<Vec<_>>().try_into().unwrap(),
			)
		})
		.collect()
}

type Solution = usize;

pub fn part_one(parts: &Intermediate) -> Option<Solution> {
	Some(
		parts
			.iter()
			.map(|s| s.1.iter())
			.flatten()
			.filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
			.count(),
	)
}

fn all_possibilities() -> HashSet<char> {
	['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect()
}

const SEGMENT_TOP: usize = 0;
const SEGMENT_TOP_RIGHT: usize = 1;
const SEGMENT_BOTTOM_RIGHT: usize = 2;
const SEGMENT_BOTTOM: usize = 3;
const SEGMENT_BOTTOM_LEFT: usize = 4;
const SEGMENT_TOP_LEFT: usize = 5;
const SEGMENT_CENTER: usize = 6;

fn solve_segments(signals: &[&str; 10]) -> [HashSet<char>; 10] {
	let mut possibilities: [HashSet<char>; 7] = [
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
	];

	let signals: Vec<HashSet<char>> = signals.iter().map(|s| s.chars().collect()).collect();

	let one_signal = signals.iter().find(|&s| s.len() == 2).unwrap();
	possibilities[SEGMENT_TOP_RIGHT] = one_signal.clone();
	possibilities[SEGMENT_BOTTOM_RIGHT] = one_signal.clone();

	let four_signal = signals.iter().find(|&s| s.len() == 4).unwrap();
	possibilities[SEGMENT_CENTER] = four_signal - one_signal;
	possibilities[SEGMENT_TOP_LEFT] = four_signal - one_signal;

	let seven_signal = signals.iter().find(|&s| s.len() == 3).unwrap();
	possibilities[SEGMENT_TOP] = seven_signal - one_signal;

	let eight_signal = signals.iter().find(|&s| s.len() == 7).unwrap();
	possibilities[SEGMENT_BOTTOM_LEFT] = eight_signal.clone();
	possibilities[SEGMENT_BOTTOM] = eight_signal.clone();

	let nine_signal = signals
		.iter()
		.find(|&s| s.len() == 6 && s.is_superset(four_signal))
		.unwrap();

	possibilities[SEGMENT_BOTTOM_LEFT] = &all_possibilities() - nine_signal;

	let six_signal = signals
		.iter()
		.find(|&s| s.len() == 6 && !s.is_superset(one_signal))
		.unwrap();

	possibilities[SEGMENT_TOP_RIGHT] = &all_possibilities() - six_signal;

	possibilities[SEGMENT_BOTTOM_RIGHT] =
		&possibilities[SEGMENT_BOTTOM_RIGHT] - &possibilities[SEGMENT_TOP_RIGHT];

	let zero_signal = signals
		.iter()
		.find(|&s| {
			s.len() == 6
				&& s.is_superset(&(&possibilities[SEGMENT_TOP_RIGHT] | &possibilities[SEGMENT_BOTTOM_LEFT]))
		})
		.unwrap();

	possibilities[SEGMENT_CENTER] = &all_possibilities() - zero_signal;

	possibilities[SEGMENT_TOP_LEFT] =
		&possibilities[SEGMENT_TOP_LEFT] - &possibilities[SEGMENT_CENTER];

	possibilities[SEGMENT_BOTTOM] = &(nine_signal - four_signal) - &possibilities[SEGMENT_TOP];

	let digit_map: [HashSet<char>; 10] = [
		// 0:
		&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_TOP_LEFT],
		// 1:
		&possibilities[SEGMENT_TOP_RIGHT] | &possibilities[SEGMENT_BOTTOM_RIGHT],
		// 2
		&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 3
		&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_CENTER],
		// 4
		&(&(&possibilities[SEGMENT_TOP_LEFT] | &possibilities[SEGMENT_CENTER])
			| &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT],
		// 5
		&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 6
		&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_BOTTOM_LEFT])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
		// 7
		&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT],
		// 8
		all_possibilities(),
		// 9
		&(&(&(&(&possibilities[SEGMENT_TOP] | &possibilities[SEGMENT_TOP_RIGHT])
			| &possibilities[SEGMENT_BOTTOM_RIGHT])
			| &possibilities[SEGMENT_BOTTOM])
			| &possibilities[SEGMENT_TOP_LEFT])
			| &possibilities[SEGMENT_CENTER],
	];

	digit_map
}

#[test]
fn solve_segments_works() {
	use itertools::Itertools;

	const SIGNALS: [&str; 10] = [
		"acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb", "cagedb", "ab",
	];

	let signal_digits: Vec<Vec<char>> = solve_segments(&SIGNALS)
		.into_iter()
		.map(|c| c.into_iter().sorted().collect::<Vec<_>>())
		.collect();

	// Check the digits we can easily solve for.
	assert_eq!(signal_digits[1], vec!['a', 'b']);
	assert_eq!(signal_digits[4], vec!['a', 'b', 'e', 'f']);
	assert_eq!(signal_digits[7], vec!['a', 'b', 'd']);
	assert_eq!(signal_digits[8], vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);

	// Check the 6-segment digits...
	assert_eq!(signal_digits[9], vec!['a', 'b', 'c', 'd', 'e', 'f']);
	assert_eq!(signal_digits[6], vec!['b', 'c', 'd', 'e', 'f', 'g']);
	assert_eq!(signal_digits[0], vec!['a', 'b', 'c', 'd', 'e', 'g']);

	// Then check the 5-segment digits.
	assert_eq!(signal_digits[2], vec!['a', 'c', 'd', 'f', 'g']);
	assert_eq!(signal_digits[3], vec!['a', 'b', 'c', 'd', 'f']);
	assert_eq!(signal_digits[5], vec!['b', 'c', 'd', 'e', 'f']);
}

fn apply_segments(solved_digits: &[HashSet<char>; 10], outputs: &[&str; 4]) -> usize {
	let digits: Vec<usize> = outputs
		.iter()
		.map(|&output| {
			let output_chars: HashSet<char> = output.chars().collect();
			solved_digits
				.iter()
				.enumerate()
				.find(|(_idx, digit)| digit == &&output_chars)
				.map(|(idx, _parts)| idx)
				.unwrap()
		})
		.collect();

	digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3]
}

pub fn part_two(parts: &Intermediate) -> Option<Solution> {
	Some(
		parts
			.iter()
			.map(|(signals, outputs)| apply_segments(&solve_segments(signals), outputs))
			.sum(),
	)
}
