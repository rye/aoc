
pub enum Line {
	Valid,
	Corrupted(char),
}

#[inline]
const fn is_open(char: char) -> bool {
	match char {
		'(' => true,
		'[' => true,
		'{' => true,
		'<' => true,
		_ => false,
	}
}

const fn is_close(char: char) -> bool {
	match char {
		')' => true,
		']' => true,
		'}' => true,
		'>' => true,
		_ => false,
	}
}

const fn corresponding_close(open: char) -> char {
	match open {
		'(' => ')',
		'[' => ']',
		'{' => '}',
		'<' => '>',
		_ => unreachable!(),
	}
}

const fn score_closing(char: char) -> u32 {
	match char {
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => 0,
	}
}

impl core::str::FromStr for Line {
	type Err = core::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut stack: Vec<char> = Vec::new();

		let mut corruption_value: Option<char> = None;

		for c in s.chars() {
			if is_open(c) {
				stack.push(c);
			} else if is_close(c) {
				match stack.pop() {
					Some(open) if c != corresponding_close(open) => {
						corruption_value = Some(c);
						break;
					}
					Some(_open) => {}
					None => panic!("stack empty :("),
				}
			} else {
				panic!("unexpected {}", c);
			}
		}

		if let Some(char) = corruption_value {
			Ok(Line::Corrupted(char))
		} else {
			Ok(Line::Valid)
		}
	}
}

#[cfg(test)]
mod line {
	use super::Line;

	mod parse {
		use super::Line;

		mod valid {

			use super::Line;

			#[test]
			fn simple() {
				assert_eq!(Ok(Line::Valid), "()".parse());
				assert_eq!(Ok(Line::Valid), "[]".parse());
				assert_eq!(Ok(Line::Valid), "{}".parse());
				assert_eq!(Ok(Line::Valid), "<>".parse());
			}

			#[test]
			fn simple_nest() {
				assert_eq!(Ok(Line::Valid), "(((((((((())))))))))".parse());
				assert_eq!(Ok(Line::Valid), "([])".parse());
				assert_eq!(Ok(Line::Valid), "{}".parse());
				assert_eq!(Ok(Line::Valid), "<([{}])>".parse());
			}

			#[test]
			fn complex() {
				assert_eq!(Ok(Line::Valid), "{()()()}".parse());
				assert_eq!(Ok(Line::Valid), "[<>({}){}[([])<>]]".parse());
			}
		}
	}
}

pub type Intermediate = Vec<Line>;

pub fn parse(input: &str) -> Result<Intermediate, core::convert::Infallible> {
	input.lines().map(str::parse).collect()
}

type Solution = u32;

pub fn part_one(line_results: &Intermediate) -> Option<Solution> {
	Some(line_results.iter().fold(0_u32, |acc, line| {
		acc
			+ match line {
				Line::Corrupted(char) => score_closing(*char),
				Line::Valid => 0_u32,
			}
	}))
}

pub fn part_two(_subsystem: &Intermediate) -> Option<Solution> {
	None
}
