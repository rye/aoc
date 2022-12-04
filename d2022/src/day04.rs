use core::{convert::Infallible, num::ParseIntError, ops::RangeInclusive, str::FromStr};

pub type Intermediate = Vec<(Assignment, Assignment)>;
pub type Output = u32;

#[derive(Debug)]
pub struct Assignment(RangeInclusive<u32>);

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
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
