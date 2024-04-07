use anyhow::anyhow;

pub type Intermediate = Vec<Vec<Instruction>>;
pub type Output = String;

pub enum Instruction {
	Up,
	Down,
	Left,
	Right,
}

impl Instruction {
	fn from_char(char: char) -> Option<Self> {
		Some(match char {
			'U' => Self::Up,
			'D' => Self::Down,
			'L' => Self::Left,
			'R' => Self::Right,
			_ => return None,
		})
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input
		.lines()
		.map(|line| {
			line
				.chars()
				.map(Instruction::from_char)
				.collect::<Option<Vec<Instruction>>>()
				.ok_or(anyhow!("invalid input"))
		})
		.collect::<anyhow::Result<Vec<Vec<_>>>>()
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day02"),
	Some("1985".to_string())
);

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
