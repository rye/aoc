use {
	core::cmp::Reverse,
	std::collections::{BTreeMap, BTreeSet, BinaryHeap},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrabPosition(i32);

type Intermediate = Vec<CrabPosition>;

pub fn parse(input: &str) -> Intermediate {
	input
		.trim()
		.split(',')
		.map(str::parse)
		.map(Result::unwrap)
		.map(CrabPosition)
		.collect::<Vec<CrabPosition>>()
}

type Solution = i32;

#[cfg(test)]
fn example_positions() -> impl Iterator<Item = CrabPosition> {
	[16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
		.into_iter()
		.map(CrabPosition)
}

fn cost_to_align_all(positions: &[CrabPosition], chosen_position: CrabPosition) -> i32 {
	positions
		.iter()
		.map(|position| (chosen_position.0 - position.0).abs())
		.sum()
}

#[test]
fn example_cost_to_align_all_2_min() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(cost_to_align_all(&positions, CrabPosition(2)), 37);
}

#[test]
fn example_cost_to_align_all_1() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(cost_to_align_all(&positions, CrabPosition(1)), 41);
}

#[test]
fn example_cost_to_align_all_3() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(cost_to_align_all(&positions, CrabPosition(3)), 39);
}

#[test]
fn example_cost_to_align_all_10() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(cost_to_align_all(&positions, CrabPosition(10)), 71);
}

fn min_fuel(positions: &[CrabPosition]) -> i32 {
	let (min, max) = positions.iter().fold((None, None), |(min, max), position| {
		(
			match min {
				Some(cur_min) if position < cur_min => Some(position),
				Some(cur_min) => Some(cur_min),
				None => Some(position),
			},
			match max {
				Some(cur_max) if position > cur_max => Some(position),
				Some(cur_max) => Some(cur_max),
				None => Some(position),
			},
		)
	});

	let min = min.unwrap();
	let max = max.unwrap();

	let position_costs: BinaryHeap<Reverse<i32>> = (min.0..=max.0)
		.map(|pos| Reverse(cost_to_align_all(positions, CrabPosition(pos))))
		.collect();

	position_costs.peek().unwrap().0
}

#[test]
fn example_min_fuel() {
	let positions = example_positions().collect::<Vec<_>>();
	assert_eq!(min_fuel(&positions), 37);
}

pub fn part_one(crabs: &Intermediate) -> Option<Solution> {
	Some(min_fuel(crabs))
}

pub fn part_two(_crabs: &Intermediate) -> Option<Solution> {
	None
}
