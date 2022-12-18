use core::{convert::Infallible, num::ParseIntError, str::FromStr};

pub enum Instruction {
	Noop,
	Addx(i32),
}

impl FromStr for Instruction {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match &s[0..4] {
			"noop" => Ok(Self::Noop),
			"addx" => s[5..].parse().map(Self::Addx),
			_ => unreachable!(),
		}
	}
}

pub type Intermediate = Vec<Instruction>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(str::parse)
			.collect::<Result<Vec<_>, _>>()?,
	)
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day10-longer"),
	Some(13140)
);

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
