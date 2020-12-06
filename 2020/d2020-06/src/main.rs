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
		let sum: usize = groups
			.iter()
			.map(|group| group.chars().filter(|c| c.is_alphabetic()))
			.map(|alphas| alphas.collect())
			.map(|unique_chars: BTreeSet<char>| unique_chars.len())
			.sum();

		println!("Part One: {:?}", sum);
	}

	{
		let sum: usize = groups
			.iter()
			.map(|group| {
				let unique_chars = people_in_group(group)
					.map(|person| person.chars().filter(|c| c.is_alphabetic()))
					.map(|chars| chars.collect())
					.fold(
						None,
						|state: Option<BTreeSet<char>>, chars: BTreeSet<char>| {
							if state.is_none() {
								Some(chars.clone())
							} else {
								Some(state.unwrap().intersection(&chars).copied().collect())
							}
						},
					)
					.expect("empty group?");

				unique_chars.len()
			})
			.sum();

		println!("Part Two: {:?}", sum);
	}
}

#[cfg(test)]
mod tests {}
