type Intermediate = LookAndSay;

pub struct LookAndSay(String);

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.nth(0)
		.map(str::to_string)
		.map(LookAndSay)
		.unwrap()
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
