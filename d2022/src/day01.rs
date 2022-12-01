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
		let snacks = str
			.lines()
			.map(|line| line.parse())
			.collect::<Result<Vec<u32>, _>>()?;

		Ok(Self { snacks })
	}
}

pub type Intermediate = Vec<Elf>;
pub type Output = u32;

pub fn parse(str: &str) -> Result<Intermediate, impl std::error::Error> {
	str.split("\n\n").map(str::parse).collect()
}

pub fn part_one(elves: &Intermediate) -> Option<Output> {
	let elf_carrying_totals: BTreeSet<u32> = elves.iter().map(|elf| elf.calorie_total()).collect();
	elf_carrying_totals.last().copied()
}

pub fn part_two(elves: &Intermediate) -> Option<Output> {
	let elf_carrying_totals: BTreeSet<u32> = elves.iter().map(|elf| elf.calorie_total()).collect();
	Some(elf_carrying_totals.iter().rev().take(3).sum())
}
