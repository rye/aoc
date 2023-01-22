pub type Intermediate = ();
pub type Output = u32;

/// # Errors
pub fn parse(_data: &str) -> anyhow::Result<Intermediate> {
	Ok(())
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day12"),
	Some(31)
);

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
