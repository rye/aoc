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

fn update_school(school: &mut BTreeMap<TimerValue, usize>) {
	// Calculate the new cell counts from the current contents of the school.
	let new_cell_counts: Vec<(TimerValue, usize)> = school
		.iter()
		// Entries look like a "timer value" => "count" map. There is, by design, exactly one
		// entry per "timer value", so this space is very small.
		.flat_map(|(timer_value, &count)| match (timer_value, count) {
			// Fish with a current timer value of 0 reproduce (producing fish with timer values of 8) and reset their timer values to 6.
			(TimerValue(0), count) => vec![(TimerValue(8), count), (TimerValue(6), count)],
			// Otherwise, the timer value ticks down by 1.
			(TimerValue(v), count) => vec![(TimerValue(v - 1), count)],
		})
		// Since we use a Vec, flatten into a big stream of individual components.
		.collect();

	// Clear out the contents of the school.
	// TODO: Is there a way to drain the school rather than iterating-then-clearing?
	school.clear();

	// Rebuild the school from the counts we just computed.
	for (timer_value, count) in new_cell_counts {
		match school.entry(timer_value) {
			Entry::Occupied(mut e) => e.insert(e.get() + count),
			Entry::Vacant(e) => *e.insert(count),
		};
	}
}

fn school_size(school: &BTreeMap<TimerValue, usize>) -> usize {
	school.values().sum()
}

fn simulate(school: &mut BTreeMap<TimerValue, usize>, cycles: usize) {
	for cycle in 0..cycles {
		update_school(school);
	}
}

pub fn part_one(school: &Intermediate) -> Option<Solution> {
	let mut school: BTreeMap<TimerValue, usize> = school.clone();

	simulate(&mut school, 80);

	Some(school_size(&school))
}

pub fn part_two(school: &Intermediate) -> Option<Solution> {
	let mut school: BTreeMap<TimerValue, usize> = school.clone();

	simulate(&mut school, 256);

	Some(school_size(&school))
}
