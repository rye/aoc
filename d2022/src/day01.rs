use {
	core::{num::ParseIntError, str::FromStr},
	std::collections::BTreeSet,
};

pub struct Elf {
	snacks: Vec<u32>,
}

impl Elf {
	fn calorie_total(&self) -> u32 {
		self.snacks.iter().sum()
	}
}

impl FromStr for Elf {
	type Err = ParseIntError;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		let snacks: Vec<u32> = str
			.lines()
			.map(str::parse)
			.collect::<Result<Vec<u32>, _>>()?;

		Ok(Self { snacks })
	}
}

pub type Intermediate = Vec<Elf>;
pub type Output = u32;

/// Parse the input to the [`Intermediate`] type.
///
/// # Errors
///
/// Will return `Err` if parsing any of the lines to a `u32` should fail.
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(
		str
			.split("\n\n")
			.map(str::parse)
			.collect::<Result<Vec<Elf>, ParseIntError>>()?,
	)
}

pub fn part_one(elves: &Intermediate) -> Option<Output> {
	let elf_carrying_totals: BTreeSet<u32> = elves.iter().map(Elf::calorie_total).collect();
	elf_carrying_totals.last().copied()
}

pub fn part_two(elves: &Intermediate) -> Option<Output> {
	let elf_carrying_totals: BTreeSet<u32> = elves.iter().map(Elf::calorie_total).collect();
	Some(elf_carrying_totals.iter().rev().take(3).sum())
}
