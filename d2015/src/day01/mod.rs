pub enum Direction {
	Up,
	Down,
}

impl From<&Direction> for i32 {
	fn from(direction: &Direction) -> Self {
		match direction {
			Direction::Down => -1,
			Direction::Up => 1,
		}
	}
}

type Intermediate = Vec<Direction>;

pub fn parse(input: &str) -> Intermediate {
	input
		.chars()
		.filter_map(|c| match c {
			'(' => Some(Direction::Up),
			')' => Some(Direction::Down),
			c if c.is_whitespace() => None,
			_ => panic!("unexpected character {}", c),
		})
		.collect()
}

type Solution = i32;

pub fn part_one(directions: &Intermediate) -> Option<Solution> {
	Some(directions.iter().map(i32::from).sum())
}

pub fn part_two(directions: &Intermediate) -> Option<Solution> {
	None
}
