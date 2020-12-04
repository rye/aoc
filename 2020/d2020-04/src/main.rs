use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, Read};

fn main() {
	let data: String = {
		let mut string: String = String::new();
		stdin().read_to_string(&mut string);
		string
	};

	let passports: Vec<&str> = data.split("\n\n").map(|line| line).collect();

	let passports: Vec<Vec<&str>> = passports
		.iter()
		.map(|line| line.split_whitespace().collect())
		.collect();

	let passports: Vec<HashMap<&str, &str>> = passports
		.iter()
		.map(|pairs| {
			pairs
				.iter()
				.map(|pair| {
					let results: Vec<&str> = pair.split(":").collect();
					(results[0], results[1])
				})
				.collect()
		})
		.collect();

	{
		println!("Part One: {:?}", ());
	}

	{
		println!("Part Two: {:?}", ());
	}
}

#[cfg(test)]
mod tests {}
