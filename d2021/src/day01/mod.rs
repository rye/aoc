pub type Intermediate = Vec<u16>;

pub fn parse(input: &str) -> Result<Intermediate, std::num::ParseIntError> {
	input.lines().map(str::parse).collect()
}

type Solution = usize;

pub fn part_one(readings: &Intermediate) -> Option<Solution> {
	Some(
		readings
			.windows(2)
			.map(<[u16; 2]>::try_from)
			.filter_map(Result::ok)
			.filter(|[a, b]| a < b)
			.count(),
	)
}

pub fn part_two(readings: &Intermediate) -> Option<Solution> {
	Some(
		readings
			.windows(3)
			.map(|window| window.iter().sum())
			.collect::<Box<[u16]>>()
			.windows(2)
			.map(<[u16; 2]>::try_from)
			.filter_map(Result::ok)
			.filter(|[a, b]| a < b)
			.count(),
	)
}
