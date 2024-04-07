pub type Intermediate = ();
pub type Output = String;

/// # Errors
pub fn parse(_data: &str) -> anyhow::Result<Intermediate> {
	Ok(())
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
