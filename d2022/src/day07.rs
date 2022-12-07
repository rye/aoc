use {
	core::convert::Infallible,
	std::{collections::BTreeMap, path::PathBuf},
};

enum Line<'a> {
	CommandLine(&'a str),
	DirLine(&'a str),
	FileLine(usize, &'a str),
}

impl<'a> TryFrom<&'a str> for Line<'a> {
	type Error = Infallible;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		// If the line is a command line (e.g. $ ls, $ cd <dir>)
		if &value[0..2] == "$ " {
			Ok(Self::CommandLine(&value[2..]))
		} else if &value[0..4] == "dir " {
			Ok(Self::DirLine(&value[4..]))
		} else {
			let mut split = value.split(' ');

			match (split.next().map(str::parse), split.next()) {
				(Some(Ok(sz)), Some(name)) => Ok(Self::FileLine(sz, name)),
				_ => unreachable!(),
			}
		}
	}
}

struct DirectoryTree(BTreeMap<PathBuf, usize>);

pub type Intermediate = ();
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let lines: Vec<Line> = input
		.lines()
		.map(TryFrom::try_from)
		.collect::<Result<Vec<Line>, _>>()?;

	Ok(())
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
