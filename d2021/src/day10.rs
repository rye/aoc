pub struct Subsystem();

pub type Intermediate = Subsystem;

pub fn parse(_input: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(Subsystem())
}

type Solution = u32;

pub fn part_one(_subsystem: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_subsystem: &Intermediate) -> Option<Solution> {
	None
}
