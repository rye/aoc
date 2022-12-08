pub type Intermediate = Vec<Vec<u8>>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(|line| {
				line
					.chars()
					.filter_map(|char| char.to_digit(10))
					.filter_map(|u32| u8::try_from(u32).ok())
					.collect()
			})
			.collect(),
	)
}

#[must_use]
pub fn part_one(lines: &Intermediate) -> Option<Output> {
	None
}

#[test]
fn part_one_example() {
	daocutil::test_example!(
		"30373\n25512\n65332\n33549\n35390",
		part_one,
		parse,
		Some(21)
	);
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
