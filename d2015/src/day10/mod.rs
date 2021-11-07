type Intermediate = LookAndSay;

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct LookAndSay(String);

pub fn parse(input: &str) -> Intermediate {
	input
		.lines()
		.nth(0)
		.map(str::to_string)
		.map(LookAndSay)
		.unwrap()
}

impl LookAndSay {
	fn say(&self) -> Self {
		let source: &str = &self.0;
		let mut output: String = String::new();
		let mut idx = 0_usize;

		loop {
			let cur_digit = source[idx..idx + 1].chars().nth(0).unwrap();
			let offset = source[idx..].find(|c| c != cur_digit);

			let slice = if let Some(offset) = offset {
				&source[idx..(idx + offset)]
			} else {
				&source[idx..]
			};

			output.push_str(&format!("{}", slice.len()));
			output.push(cur_digit);

			if let Some(offset) = offset {
				idx = idx + offset;
			} else {
				break;
			}
		}

		Self(output)
	}
}

#[cfg(test)]
mod look_and_say {
	use super::LookAndSay;

	#[test]
	fn say_1() {
		let las = LookAndSay("1".into());
		assert_eq!(las.say(), LookAndSay("11".into()));
	}

	#[test]
	fn say_11() {
		let las = LookAndSay("11".into());
		assert_eq!(las.say(), LookAndSay("21".into()));
	}

	#[test]
	fn say_21() {
		let las = LookAndSay("21".into());
		assert_eq!(las.say(), LookAndSay("1211".into()));
	}

	#[test]
	fn say_1211() {
		let las = LookAndSay("1211".into());
		assert_eq!(las.say(), LookAndSay("111221".into()));
	}

	#[test]
	fn say_111221() {
		let las = LookAndSay("111221".into());
		assert_eq!(las.say(), LookAndSay("312211".into()));
	}
}

type Solution = usize;

pub fn part_one(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}

pub fn part_one(seed_command: &Intermediate) -> Option<Solution> {
	let mut current_las: LookAndSay = seed_command.clone();

	for _ in 0..40 {
		current_las = current_las.say();
	}

	Some(current_las.0.len())
}

pub fn part_two(seed_command: &Intermediate) -> Option<Solution> {
	let mut current_las: LookAndSay = seed_command.clone();

	for _ in 0..50 {
		current_las = current_las.say();
	}

	Some(current_las.0.len())
}
