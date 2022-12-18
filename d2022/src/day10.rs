use core::{num::ParseIntError, str::FromStr};

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

fn values_during_and_after(instructions: &[Instruction]) -> (Vec<i32>, Vec<i32>) {
	let mut reg_x = 1_i32;

	let mut values_during = vec![reg_x];
	let mut values_after = vec![reg_x];

	for instruction in instructions {
		match instruction {
			Instruction::Noop => {
				// Noop takes one cycle and has no effect.
				values_during.push(reg_x);
				values_after.push(reg_x);
			}
			Instruction::Addx(v) => {
				// Addx takes two cycles to complete.

				// First cycle: push current values.
				values_during.push(reg_x);
				values_after.push(reg_x);

				// Second cycle: reg value is updated after the during collection completes.
				values_during.push(reg_x);
				reg_x += v;
				values_after.push(reg_x);
			}
		}
	}

	(values_during, values_after)
}

#[test]
fn values_during_and_after_short_example() {
	use Instruction::*;
	let instructions = vec![Noop, Addx(3), Addx(-5)];

	let (values_during, values_after) = values_during_and_after(&instructions);

	assert_eq!(values_during, vec![1, 1, 1, 1, 4, 4]);
	assert_eq!(values_after, vec![1, 1, 1, 4, 4, -1]);

	assert_eq!(values_during[1], 1);
	assert_eq!(values_after[1], 1);

	assert_eq!(values_during[2], 1);
	assert_eq!(values_after[2], 1);

	assert_eq!(values_during[3], 1);
	assert_eq!(values_after[3], 4);

	assert_eq!(values_during[4], 4);
	assert_eq!(values_after[4], 4);

	assert_eq!(values_during[5], 4);
	assert_eq!(values_after[5], -1);
}

#[must_use]
pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	let (values_during, _values_after) = values_during_and_after(instructions);

	Some(
		[20, 60, 100, 140, 180, 220]
			.map(|cycle| values_during[cycle] * (cycle as i32))
			.iter()
			.sum::<i32>() as u32,
	)
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
