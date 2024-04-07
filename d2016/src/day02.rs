use anyhow::anyhow;
use phf::phf_map;

pub type Intermediate = Vec<Vec<Instruction>>;
pub type Output = String;

#[derive(Debug)]
pub enum Instruction {
	Up,
	Down,
	Left,
	Right,
}

impl Instruction {
	fn from_char(char: char) -> Option<Self> {
		Some(match char {
			'U' => Self::Up,
			'D' => Self::Down,
			'L' => Self::Left,
			'R' => Self::Right,
			_ => return None,
		})
	}

	fn to_key(&self, loc: char) -> [u8; 2] {
		[
			match self {
				Self::Up => 0,
				Self::Down => 1,
				Self::Left => 2,
				Self::Right => 3,
			},
			loc.to_digit(16).expect("expected a digit") as u8,
		]
	}
}

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	input
		.lines()
		.map(|line| {
			line
				.chars()
				.map(Instruction::from_char)
				.collect::<Option<Vec<Instruction>>>()
				.ok_or(anyhow!("invalid input"))
		})
		.collect::<anyhow::Result<Vec<Vec<_>>>>()
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day02"),
	Some("1985".to_string())
);

static PART_ONE_MAP: phf::Map<[u8; 2], char> = phf_map! {
	// 0 = Up
	[0, 1] => '1',
	[0, 2] => '2',
	[0, 3] => '3',
	[0, 4] => '1',
	[0, 5] => '2',
	[0, 6] => '3',
	[0, 7] => '4',
	[0, 8] => '5',
	[0, 9] => '6',

	// 1 = Down
	[1, 1] => '4',
	[1, 2] => '5',
	[1, 3] => '6',
	[1, 4] => '7',
	[1, 5] => '8',
	[1, 6] => '9',
	[1, 7] => '7',
	[1, 8] => '8',
	[1, 9] => '9',

	// 2 = Left
	[2, 1] => '1',
	[2, 2] => '1',
	[2, 3] => '2',
	[2, 4] => '4',
	[2, 5] => '4',
	[2, 6] => '5',
	[2, 7] => '7',
	[2, 8] => '7',
	[2, 9] => '8',

	// 3 = Right
	[3, 1] => '2',
	[3, 2] => '3',
	[3, 3] => '3',
	[3, 4] => '5',
	[3, 5] => '6',
	[3, 6] => '6',
	[3, 7] => '8',
	[3, 8] => '9',
	[3, 9] => '9',
};

fn apply_map_to_number(
	map: &phf::Map<[u8; 2], char>,
	instruction: &Instruction,
	number: char,
) -> char {
	let key = instruction.to_key(number);
	*map.get(&key).expect("expected a key")
}

#[must_use]
pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	let result = instructions.iter().fold(
		(String::new(), None),
		|(mut code, last_pos): (String, Option<char>), instructions| {
			let last_pos = instructions
				.iter()
				.fold(last_pos, |number, instruction| {
					let number = number.unwrap_or('5');
					let new_number = apply_map_to_number(&PART_ONE_MAP, instruction, number);
					Some(new_number)
				})
				.expect("expected to step at least once");

			code.push(last_pos);
			(code, Some(last_pos))
		},
	);

	Some(result.0)
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
