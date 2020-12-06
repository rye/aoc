use std::collections::BTreeSet;
use std::io::{stdin, Read};

fn people_in_group(group: &str) -> impl Iterator<Item = &str> {
	group.split_whitespace()
}

fn answers(person: &str) -> impl Iterator<Item = char> + '_ {
	person.chars().filter(|c| c.is_alphabetic())
}

fn intersect_all(items: impl Iterator<Item = BTreeSet<char>>) -> BTreeSet<char> {
	items
		.fold(
			None,
			|state: Option<BTreeSet<char>>, chars: BTreeSet<char>| {
				if let Some(state) = state {
					Some(state.intersection(&chars).copied().collect())
				} else {
					Some(chars)
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
			.map(|unique_chars: BTreeSet<char>| unique_chars.len())
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
