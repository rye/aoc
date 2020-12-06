use std::collections::BTreeSet;
use std::io::{stdin, Read};

use d2020::day06::*;

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let groups: Vec<&str> = data.split("\n\n").collect();

	{
		let sum: usize = groups
			.iter()
			.map(|group| answers(group).collect())
			.map(|answers: BTreeSet<Answer>| answers.len())
			.sum();

		println!("Part One: {:?}", sum);
	}

	{
		let sum: usize = groups
			.iter()
			.map(|group| {
				let answers_by_person = people_in_group(group).map(|person| answers(person).collect());
				let answers_by_all = intersect_all(answers_by_person);

				answers_by_all.expect("no people in group").len()
			})
			.sum();

		println!("Part Two: {:?}", sum);
	}
}
