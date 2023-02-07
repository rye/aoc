use itertools::Itertools;

pub type Intermediate = Vec<char>;
pub type Solution = usize;

pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	match input.lines().next() {
		Some(line) => Ok(line.chars().collect()),
		_ => panic!("no input given"),
	}
}

pub fn part_one(chars: &Intermediate) -> Option<Solution> {
	Some(
		chars
			.iter()
			.circular_tuple_windows()
			.filter(|(a, b)| a == b)
			.map(|(a, _b)| a)
			.filter_map(|c| c.to_digit(10))
			.sum::<u32>() as usize,
	)
}

daocutil::test_example!(example_1122, parse, part_one, "1122", Some(3));
daocutil::test_example!(example_1111, parse, part_one, "1111", Some(4));
daocutil::test_example!(example_1234, parse, part_one, "1234", Some(0));
daocutil::test_example!(example_91212129, parse, part_one, "91212129", Some(9));

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
