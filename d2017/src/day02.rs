use itertools::Itertools;

pub type Intermediate = Vec<Vec<u32>>;
pub type Solution = u32;

pub fn parse(spreadsheet: &str) -> anyhow::Result<Intermediate> {
	Ok(
		spreadsheet
			.lines()
			.map(|line| {
				line
					.split_whitespace()
					.map(|number| u32::from_str_radix(number, 10))
					.collect::<Result<Vec<u32>, _>>()
			})
			.collect::<Result<Vec<Vec<u32>>, _>>()?,
	)
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day02-1"),
	Some(18)
);

pub fn part_one(spreadsheet: &Intermediate) -> Option<Solution> {
	Some(
		spreadsheet
			.iter()
			.map(|line| (line.iter().min(), line.iter().max()))
			.map(|(min, max)| max.unwrap_or(&0) - min.unwrap_or(&0))
			.sum(),
	)
}

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day02-2"),
	Some(9)
);

pub fn part_two(spreadsheet: &Intermediate) -> Option<Solution> {
	Some(
		spreadsheet
			.iter()
			.flat_map(|line| {
				line
					.iter()
					.permutations(2)
					.find(|pair| pair[0] % pair[1] == 0)
					.map(|pair| pair[0] / pair[1])
			})
			.sum(),
	)
}
