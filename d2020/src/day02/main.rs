use std::io::{stdin, BufRead};

use d2020::day02::*;

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
