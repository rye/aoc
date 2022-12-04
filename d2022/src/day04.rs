use {
	core::{
		num::ParseIntError,
		ops::{Deref, RangeInclusive},
		str::FromStr,
	},
	std::collections::HashSet,
};

pub type Intermediate = Vec<(Assignment, Assignment)>;
pub type Output = u32;

#[derive(Debug)]
pub struct Assignment(HashSet<u32>);

impl Deref for Assignment {
	type Target = HashSet<u32>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl FromStr for Assignment {
	type Err = ParseIntError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let split: Vec<&str> = str.split('-').collect();
		let contents: HashSet<u32> = ((split[0].parse()?)..=(split[1].parse()?)).collect();
		Ok(Self(contents))
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
		.filter(|(left, right)| left.is_superset(&right) || right.is_superset(&left))
		.count()
		.try_into()
		.ok()
}

#[must_use]
pub fn part_two(assignments: &Intermediate) -> Option<Output> {
	assignments
		.iter()
		.filter(|(left, right)| left.intersection(&right).count() > 0)
		.count()
		.try_into()
		.ok()
}
