use std::collections::{BTreeSet, VecDeque};

pub type Intermediate = ();
type Solution = usize;

fn perform_move_cycle(cups: &mut VecDeque<(u8, bool)>) {
	let mut cup_values: BTreeSet<u8> = cups.iter().map(|c| c.0).collect();

	let current_idx = cups
		.iter()
		.position(|c| (c.1))
		.expect("expected current cup");

	cups.rotate_left(current_idx);

	debug_assert!(cups.front().is_some_and(|c| c.1), "first cup is current");

	let current_cup = cups.pop_front().expect("expected to have a cup to pop");
	debug_assert!(current_cup.1);

	cup_values.remove(&current_cup.0);

	let mut picked_up: Vec<(u8, bool)> = vec![];

	for _ in 0..3 {
		let picked_up_cup = cups.pop_front().expect("expected to be able to pop a cup");
		cup_values.remove(&picked_up_cup.0);
		picked_up.push(picked_up_cup);
	}

	let destination_idx = cups
		.iter()
		.position(|c| c.0 == *cup_values.last().expect("no remaining cups"));
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
}

pub fn parse(_: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(())
}

pub fn part_one(_: &Intermediate) -> Option<Solution> {
	None
}
pub fn part_two(_: &Intermediate) -> Option<Solution> {
	None
}
