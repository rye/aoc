pub type Intermediate = Vec<(u8, u8)>;
pub type Output = u32;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(
		input
			.lines()
			.map(|line| line.chars().filter(|c| c.is_ascii_digit()))
			.map(|c| {
				let digits: Vec<u8> = c
					.map(|char| {
						char
							.to_digit(10)
							.expect("expected ascii digit to be convertible to base-10 value")
							.try_into()
							.expect("expected ascii digit to be convertible to u8")
					})
					.collect();

				dbg!(&digits);

				(
					digits
						.first()
						.expect("expected at least one digit")
						.to_owned(),
					digits
						.last()
						.expect("expected at least one digit")
						.to_owned(),
				)
			})
			.collect(),
	)
}

#[must_use]
pub fn part_one(terminal_digits: &Intermediate) -> Option<Output> {
	Some(
		terminal_digits
			.iter()
			.map(|&(tens, ones)| (u32::from(tens) * 10 + u32::from(ones)))
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
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
