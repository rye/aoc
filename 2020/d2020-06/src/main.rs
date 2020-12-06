use core::iter::FromIterator;
use std::collections::BTreeSet;
use std::io::{stdin, Read};

fn people_in_group(group: &str) -> impl Iterator<Item = &str> {
	group.split_whitespace()
}

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let groups: Vec<&str> = data.split("\n\n").collect();

	{
		let sum = groups
			.iter()
			.map(|group| group.chars().filter(|c| c.is_alphabetic()))
			.map(|alphas| BTreeSet::from_iter(alphas))
			.map(|unique_chars| unique_chars.len())
			.fold(0, |acc, qs| acc + qs);

		println!("Part One: {:?}", sum);
	}

	{
		let sum = groups
			.iter()
			.map(|group| {
				let unique_chars = people_in_group(group)
					.map(|person| person.chars().filter(|c| c.is_alphabetic()))
					.map(|chars| BTreeSet::from_iter(chars))
					.fold(None, |state, chars| {
						if state.is_none() {
							Some(chars.clone())
						} else {
							Some(state.unwrap().intersection(&chars).copied().collect())
						}
					})
					.expect("empty group?");

				unique_chars.len()
			})
			.fold(0, |acc, x| acc + x);

		println!("Part Two: {:?}", sum);
	}
}

#[cfg(test)]
mod tests {}
