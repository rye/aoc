use std::collections::{BTreeSet, VecDeque};

pub type Intermediate = VecDeque<(u8, bool)>;
type Solution = String;

fn maxish(set: &BTreeSet<u8>, cur: u8) -> u8 {
	let max = set.last().to_owned();

	let max_below = set.range(0..cur).last().to_owned();

	*max_below.or(max).expect("oops")
}

fn perform_move_cycle(cups: &mut VecDeque<(u8, bool)>) {
	let mut cup_values: BTreeSet<u8> = cups.iter().map(|c| c.0).collect();

	let current_idx = cups
		.iter()
		.position(|c| (c.1))
		.expect("expected current cup");

	cups.rotate_left(current_idx);

	debug_assert!(cups.front().is_some_and(|c| c.1), "first cup is current");

	let mut current_cup = cups.pop_front().expect("expected to have a cup to pop");
	debug_assert!(current_cup.1);

	cup_values.remove(&current_cup.0);

	let mut picked_up: Vec<(u8, bool)> = vec![];

	for _ in 0..3 {
		let picked_up_cup = cups.pop_front().expect("expected to be able to pop a cup");
		cup_values.remove(&picked_up_cup.0);
		picked_up.push(picked_up_cup);
	}

	let destination_value = maxish(&cup_values, current_cup.0);

	let destination_idx = cups
		.iter()
		.position(|c| c.0 == destination_value)
		.expect("could not find destination despite having index");

	cups.rotate_left(destination_idx);

	let destination_cup = cups.pop_front().expect("expected to pop destination");
	debug_assert_eq!(destination_cup.0, destination_value);

	while let Some(cup) = picked_up.pop() {
		cups.push_front(cup);
	}

	cups.push_front(destination_cup);

	cups.rotate_right(destination_idx);

	current_cup.1 = false;

	cups.push_front(current_cup);
	cups.rotate_right(current_idx);

	// Select new cup.

	if let Some(next) = cups.get_mut(current_idx + 1) {
		next.1 = true;
	} else if let Some(next) = cups.get_mut(0) {
		next.1 = true;
	}
}

#[test]
fn perform_move_example() {
	let mut cups = vec![
		(3, true),
		(8, false),
		(9, false),
		(1, false),
		(2, false),
		(5, false),
		(4, false),
		(6, false),
		(7, false),
	]
	.into();

	perform_move_cycle(&mut cups);

	assert_eq!(
		VecDeque::<(u8, bool)>::from(vec![
			(3, false),
			(2, true),
			(8, false),
			(9, false),
			(1, false),
			(5, false),
			(4, false),
			(6, false),
			(7, false),
		]),
		cups
	);

	perform_move_cycle(&mut cups);

	assert_eq!(
		VecDeque::<(u8, bool)>::from(vec![
			(3, false),
			(2, false),
			(5, true),
			(4, false),
			(6, false),
			(7, false),
			(8, false),
			(9, false),
			(1, false),
		]),
		cups
	);

	perform_move_cycle(&mut cups);

	assert_eq!(
		VecDeque::<(u8, bool)>::from(vec![
			(7, false),
			(2, false),
			(5, false),
			(8, true),
			(9, false),
			(1, false),
			(3, false),
			(4, false),
			(6, false),
		]),
		cups
	);
}

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	let first_line = input
		.lines()
		.next()
		.expect("expected to have at least one line");

	let mut digits: Vec<(u8, bool)> = first_line
		.chars()
		.map(|c| u8::from_str_radix(&c.to_string(), 10).expect("failed to convert to string"))
		.map(|n| (n, false))
		.collect();

	digits[0].1 = true;

	Ok(digits.into())
}

fn fixed_report(digits: &VecDeque<(u8, bool)>) -> String {
	let mut digits = digits.to_owned();
	let one_idx = digits
		.iter()
		.position(|c| c.0 == 1_u8)
		.expect("expected to find appropriate position");

	digits.rotate_left(one_idx);
	debug_assert_eq!(digits[0].0, 1_u8);

	let _ = digits.pop_front();

	let mut output: String = String::new();
	for s in digits.drain(..) {
		output = format!("{output}{}", s.0);
	}

	output
}

pub fn part_one(set: &Intermediate) -> Option<Solution> {
	let mut set = set.to_owned();
	for _ in 0..100 {
		perform_move_cycle(&mut set);
	}

	Some(fixed_report(&set))
}
pub fn part_two(_: &Intermediate) -> Option<Solution> {
	None
}
