use anyhow::anyhow;

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

	fn apply_to_number(&self, number: u8) -> u8 {
		match self {
			Self::Up => match number {
				1..=3 => number,
				4..=9 => number - 3,
				_ => unreachable!(),
			},
			Self::Down => match number {
				1..=6 => number + 3,
				7..=9 => number,
				_ => unreachable!(),
			},
			Self::Left => match number {
				1 | 4 | 7 => number,
				2 | 5 | 8 | 3 | 6 | 9 => number - 1,
				_ => unreachable!(),
			},
			Self::Right => match number {
				1 | 2 | 4 | 5 | 7 | 8 => number + 1,
				3 | 6 | 9 => number,
				_ => unreachable!(),
			},
		}
	}
}

#[test]
fn instruction_up() {
	assert_eq!(Instruction::Up.apply_to_number(1), 1);
	assert_eq!(Instruction::Up.apply_to_number(2), 2);
	assert_eq!(Instruction::Up.apply_to_number(3), 3);
	assert_eq!(Instruction::Up.apply_to_number(4), 1);
	assert_eq!(Instruction::Up.apply_to_number(5), 2);
	assert_eq!(Instruction::Up.apply_to_number(6), 3);
	assert_eq!(Instruction::Up.apply_to_number(7), 4);
	assert_eq!(Instruction::Up.apply_to_number(8), 5);
	assert_eq!(Instruction::Up.apply_to_number(9), 6);
}

#[test]
fn instruction_left() {
	assert_eq!(Instruction::Left.apply_to_number(1), 1);
	assert_eq!(Instruction::Left.apply_to_number(2), 1);
	assert_eq!(Instruction::Left.apply_to_number(3), 2);
	assert_eq!(Instruction::Left.apply_to_number(4), 4);
	assert_eq!(Instruction::Left.apply_to_number(5), 4);
	assert_eq!(Instruction::Left.apply_to_number(6), 5);
	assert_eq!(Instruction::Left.apply_to_number(7), 7);
	assert_eq!(Instruction::Left.apply_to_number(8), 7);
	assert_eq!(Instruction::Left.apply_to_number(9), 8);
}

#[test]
fn instruction_down() {
	assert_eq!(Instruction::Down.apply_to_number(1), 4);
	assert_eq!(Instruction::Down.apply_to_number(2), 5);
	assert_eq!(Instruction::Down.apply_to_number(3), 6);
	assert_eq!(Instruction::Down.apply_to_number(4), 7);
	assert_eq!(Instruction::Down.apply_to_number(5), 8);
	assert_eq!(Instruction::Down.apply_to_number(6), 9);
	assert_eq!(Instruction::Down.apply_to_number(7), 7);
	assert_eq!(Instruction::Down.apply_to_number(8), 8);
	assert_eq!(Instruction::Down.apply_to_number(9), 9);
}

#[test]
fn instruction_right() {
	assert_eq!(Instruction::Right.apply_to_number(1), 2);
	assert_eq!(Instruction::Right.apply_to_number(2), 3);
	assert_eq!(Instruction::Right.apply_to_number(3), 3);
	assert_eq!(Instruction::Right.apply_to_number(4), 5);
	assert_eq!(Instruction::Right.apply_to_number(5), 6);
	assert_eq!(Instruction::Right.apply_to_number(6), 6);
	assert_eq!(Instruction::Right.apply_to_number(7), 8);
	assert_eq!(Instruction::Right.apply_to_number(8), 9);
	assert_eq!(Instruction::Right.apply_to_number(9), 9);
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

#[must_use]
pub fn part_one(instructions: &Intermediate) -> Option<Output> {
	let result = instructions.iter().fold(
		(String::new(), None),
		|(mut code, last_pos): (String, Option<u8>), instructions| {
			let last_pos = instructions
				.iter()
				.fold(last_pos, |number, instruction| {
					let number = number.unwrap_or(5);
					let new_number = instruction.apply_to_number(number);
					Some(new_number)
				})
				.expect("expected to step at least once");

			code.push(last_pos.to_string().chars().next().unwrap());
			(code, Some(last_pos))
		},
	);

	Some(result.0)
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
