pub struct Subsystem();

impl core::str::FromStr for Subsystem {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		todo!()
	}
}

pub type Intermediate = Subsystem;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.parse()
}

type Solution = u32;

pub fn part_one(_subsystem: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_subsystem: &Intermediate) -> Option<Solution> {
	None
}
