type Intermediate<'input> = (&'input str, [[&'input str; 2]; 100]);

pub fn parse<'input>(input: &'input str) -> Intermediate<'input> {
	let mut split = input.split("\n\n");
	let first_line: &'input str = split.next().expect("missing template");
	let insertion_rules: &'input str = split.next().expect("missing rules");

	let insertion_rules: [[&'input str; 2]; 100] = insertion_rules
		.lines()
		.map(|line| {
			line
				.split(" -> ")
				.collect::<Vec<_>>()
				.try_into()
				.expect("failed to collect both parts of a rule")
		})
		.collect::<Vec<[&'input str; 2]>>()
		.try_into()
		.expect("failed to collect 100 rules");

	(first_line, insertion_rules)
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
