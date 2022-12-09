pub type Intermediate = ();
pub type Output = usize;

/// # Errors
pub fn parse(_data: &str) -> anyhow::Result<Intermediate> {
	Ok(())
}

#[must_use]
pub fn part_one(_intermediate: &Intermediate) -> Option<Output> {
	None
}

#[test]
fn part_one_example() {
	let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
	daocutil::test_example!(input, part_one, parse, Some(13));
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
