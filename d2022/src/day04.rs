use {
	core::{
		convert::Infallible,
		num::ParseIntError,
		ops::{Deref, RangeInclusive},
		str::FromStr,
	},
	std::collections::HashSet,
};

pub type Intermediate = Vec<(Assignment, Assignment)>;
pub type Output = u32;

#[derive(Debug)]
pub struct Assignment(RangeInclusive<u32>);

impl Deref for Assignment {
	type Target = RangeInclusive<u32>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl FromStr for Assignment {
	type Err = ParseIntError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let split: Vec<&str> = str.split('-').collect();

		Ok(Self(split[0].parse()?..=split[1].parse()?))
	}
}

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	let intermediate = str
		.lines()
		.map(|line| {
			let parts: Vec<&str> = line.split(',').collect();

			(parts[0].parse().unwrap(), parts[1].parse().unwrap())
		})
		.collect();

	Ok(intermediate)
}

#[must_use]
pub fn part_one(assignments: &Intermediate) -> Option<Output> {
	assignments
		.iter()
		.filter(|(left, right)| {
			let left_contents: HashSet<u32> = (left.0).clone().collect();
			let right_contents: HashSet<u32> = (right.0).clone().collect();
			left_contents.is_superset(&right_contents) || right_contents.is_superset(&left_contents)
		})
		.count()
		.try_into()
		.ok()
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
