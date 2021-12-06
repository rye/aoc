use std::{
	collections::{btree_map::Entry, BTreeMap},
	convert::Infallible,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimerValue(u8);

impl core::str::FromStr for TimerValue {
	type Err = Infallible;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let timer_value: u8 = value.parse().expect("failed to parse fish value");
		Ok(Self(timer_value))
	}
}

type Intermediate = BTreeMap<TimerValue, usize>;

pub fn parse(input: &str) -> Intermediate {
	let fish_values: Vec<TimerValue> = input
		.trim()
		.split(',')
		.map(str::parse)
		.collect::<Result<Vec<_>, _>>()
		.expect("failed to parse input");

	let mut school: BTreeMap<TimerValue, usize> = BTreeMap::new();

	for fish_value in fish_values {
		match school.entry(fish_value) {
			Entry::Occupied(mut e) => e.insert(e.get() + 1),
			Entry::Vacant(e) => *e.insert(1),
		};
	}

	school
}

type Solution = usize;

fn school_replacement(school: &BTreeMap<TimerValue, usize>) -> BTreeMap<TimerValue, usize> {
	let new_cell_counts: Vec<(TimerValue, usize)> = school
		.iter()
		.map(|(timer_value, &count)| match (timer_value, count) {
			(TimerValue(0), count) => vec![(TimerValue(8), count), (TimerValue(6), count)],
			(TimerValue(v), count) => vec![(TimerValue(v - 1), count)],
		})
		.flatten()
		.collect();

	let mut new_map: BTreeMap<TimerValue, usize> = BTreeMap::new();

	for (timer_value, count) in new_cell_counts {
		match new_map.entry(timer_value) {
			Entry::Occupied(mut e) => e.insert(e.get() + count),
			Entry::Vacant(e) => *e.insert(count),
		};
	}

	new_map
}

fn school_size(school: &BTreeMap<TimerValue, usize>) -> usize {
	school.values().sum()
}

fn simulate(school: &mut BTreeMap<TimerValue, usize>, cycles: usize) {
	for cycle in 0..cycles {
		let new_counts = school_replacement(school);
		*school = new_counts;
	}
}

pub fn part_one(school: &Intermediate) -> Option<Solution> {
	let mut school: BTreeMap<TimerValue, usize> = school.clone();

	simulate(&mut school, 80);

	Some(school_size(&school))
}

pub fn part_two(school: &Intermediate) -> Option<Solution> {
	let mut school: BTreeMap<TimerValue, usize> = school.to_owned();

	simulate(&mut school, 256);

	Some(school_size(&school))
}
