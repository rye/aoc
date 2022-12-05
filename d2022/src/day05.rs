use {
	core::ops::Range,
	std::collections::{hash_map::Entry, HashMap, VecDeque},
};

#[derive(Debug)]
struct Crate(char);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Stack(u8);

pub struct State {
	held: VecDeque<Crate>,
	stacks: HashMap<Stack, Vec<Crate>>,
}

pub struct Move {
	amount: u8,
	from: Stack,
	to: Stack,
}

pub type Intermediate = (State, Vec<Move>);
pub type Output = String;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let regions: Vec<&str> = input.split("\n\n").collect();

	assert_eq!(regions.len(), 2);

	let header_region = regions[0];
	let moves_region = regions[1];

	let state: State = {
		let lines: Vec<&str> = header_region.lines().collect();

		let last_line = lines.last().expect("no lines?");

		// TODO: Support multi-character labels and add more robust filtering.
		let labels: Vec<(Stack, Range<usize>)> = last_line
			.char_indices()
			.filter_map(|(idx, char)| match char {
				' ' => None,
				c if c.is_digit(10) => Some((
					Stack(
						c.to_string()
							.parse()
							.expect("failed to parse digit as string"),
					),
					idx..idx + 1,
				)),
				_ => unreachable!(),
			})
			.collect();

		let held: VecDeque<Crate> = VecDeque::new();

		let mut stacks = HashMap::new();

		for (stack, range) in labels {
			for idx in (0..(lines.len() - 1)).rev() {
				let line = lines[idx];

				let value: &str = &line[range.clone()];

				assert_eq!(value.len(), 1);

				let value = value
					.chars()
					.next()
					.expect("expected exactly one character");

				match value {
					' ' => break,
					c if c.is_ascii_uppercase() => {
						let stack = stack;
						let krate = Crate(c);

						// Insert onto stack
						stacks.entry(stack).or_insert(Vec::new()).push(krate);
					}
					_ => unreachable!(),
				};
			}
		}

		dbg!(&stacks);

		State { held, stacks }
	};

	let moves: Vec<Move> = { todo!() };

	Ok((state, moves))
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
