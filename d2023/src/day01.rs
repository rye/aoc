pub type Intermediate<'i> = Vec<&'i str>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.lines().collect())
}

fn get_calibration_value(str: &str, consider_spelling: bool) -> u32 {
	let mut input = str.to_string();
	if consider_spelling {
		// do somethign
	}

	format!(
		"{}{}",
		input.chars().find(|c| c.is_ascii_digit()).unwrap(),
		input.chars().rfind(|c| c.is_ascii_digit()).unwrap()
	)
	.parse()
	.unwrap()
}

#[must_use]
pub fn part_one(terminal_digits: &Intermediate) -> Option<Output> {
	Some(
		terminal_digits
			.iter()
			.map(|str| get_calibration_value(str, false))
			.sum(),
	)
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day01-part1"),
	Some(142)
);

#[must_use]
pub fn part_two(terminal_digits: &Intermediate) -> Option<Output> {
	Some(
		terminal_digits
			.iter()
			.map(|str| get_calibration_value(str, true))
			.sum(),
	)
}

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day01-part2"),
	Some(281)
);
