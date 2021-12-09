use std::collections::HashSet;

type Intermediate<'a> = Vec<([&'a str; 10], [&'a str; 4])>;

pub fn parse<'a>(input: &'a str) -> Intermediate<'a> {
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

enum Segment {
	Definite(char),
	Possibilities(HashSet<char>),
}

fn all_possibilities() -> HashSet<char> {
	['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect()
}

fn solve_segments(signals: &[&str; 10]) -> [HashSet<char>; 10] {
	let result: [HashSet<char>; 10] = [
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
		all_possibilities(),
	];

	result
}

fn apply_segments(solved_digits: &[HashSet<char>; 10], outputs: &[&str; 4]) -> usize {
	0
}

pub fn part_two(parts: &Intermediate) -> Option<Solution> {
	Some(
		parts
			.iter()
			.map(|(signals, outputs)| apply_segments(&solve_segments(&signals), outputs))
			.sum(),
	)
}
