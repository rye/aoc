use std::{
	io::{stdin, BufRead},
	str::FromStr,
};

struct Rule {
	count_range: (usize, usize),
	character: char,
}

impl FromStr for Rule {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, ()> {
		let numbers = s.split(" ").nth(0).unwrap();
		let start = numbers.split("-").nth(0).unwrap();
		let end = numbers.split("-").nth(1).unwrap();

		let a = start.parse::<usize>().unwrap();
		let b = end.parse::<usize>().unwrap();

		let character = s.split(" ").nth(1).unwrap();
		let character = character.chars().nth(0).unwrap();

		Ok(Self {
			count_range: (a, b),
			character,
		})
	}
}

fn validate_password(rule: &Rule, password: &str) -> bool {
	let count_range = rule.count_range.clone();
	let character = rule.character;

	let chars: Vec<char> = password.chars().collect();
	let n_matches = &chars
		.iter()
		.filter(|c| *c == &character)
		.collect::<Vec<&char>>()
		.len();

	count_range.0 <= *n_matches && *n_matches <= count_range.1
}

fn validate_password_two(rule: &Rule, password: &str) -> bool {
	let count_range = rule.count_range.clone();
	let character = rule.character;

	let chars: Vec<char> = password.chars().collect();

	(chars[count_range.0 - 1] == character) ^ (chars[count_range.1 - 1] == character)
}

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let rules: Vec<(Rule, String)> = stdin
		.lines()
		.filter_map(Result::ok)
		.map(|s: String| {
			let rule = (&s).split(": ").nth(0).unwrap().parse::<Rule>().unwrap();
			let password = (&s).split(": ").nth(1).unwrap().to_string();

			(rule, password.clone())
		})
		.collect();

	let result: Vec<&(Rule, String)> = rules
		.iter()
		.filter(|(rule, password)| validate_password(rule, password))
		.collect();

	println!("Part One: {:?}", result.len());

	let result_deux: Vec<&(Rule, String)> = rules
		.iter()
		.filter(|(rule, password)| validate_password_two(rule, password))
		.collect();

	println!("Part Two: {:?}", result_deux.len());
}
