type Intermediate<'a> = Vec<(&'a str, &'a str)>;

pub fn parse<'a>(input: &'a str) -> Intermediate<'a> {
	input
		.lines()
		.map(|line| line.split(" | "))
		.map(|mut split| (split.next().unwrap(), split.next().unwrap()))
		.collect()
}

type Solution = usize;

pub fn part_one(parts: &Intermediate) -> Option<Solution> {
	Some(
		parts
			.iter()
			.map(|n| n.1.split(' '))
			.flatten()
			.filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
			.count(),
	)
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
