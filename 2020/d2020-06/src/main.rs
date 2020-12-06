use std::collections::BTreeSet;
use std::io::{stdin, Read};

type Answer = char;

fn people_in_group(group: &str) -> impl Iterator<Item = &str> {
	group.split_whitespace()
}

fn answers(person: &str) -> impl Iterator<Item = Answer> + '_ {
	person.chars().filter(|c| c.is_alphabetic())
}

fn intersect_all(items: impl Iterator<Item = BTreeSet<Answer>>) -> BTreeSet<Answer> {
	items
		.fold(
			None,
			|state: Option<BTreeSet<Answer>>, answers: BTreeSet<Answer>| {
				if let Some(state) = state {
					Some(state.intersection(&answers).copied().collect())
				} else {
					Some(answers)
				}
			},
		)
		.expect("empty iterator?")
}

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

				answers_by_all.len()
			})
			.sum();

		println!("Part Two: {:?}", sum);
	}
}

#[cfg(test)]
mod tests {}
