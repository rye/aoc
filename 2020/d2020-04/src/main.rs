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
		let valid_passports: Vec<&HashMap<&str, &str>> = passports
			.iter()
			.filter(|passport| {
				match (
					passport.get("byr"),
					passport.get("iyr"),
					passport.get("eyr"),
					passport.get("hgt"),
					passport.get("hcl"),
					passport.get("ecl"),
					passport.get("pid"),
					passport.get("cid"),
				) {
					(Some(_), Some(_), Some(_), Some(_), Some(_), Some(_), Some(_), _) => true,
					_ => false,
				}
			})
			.collect();

		println!("Part One: {:?}", valid_passports.len());
	}

	{
		println!("Part Two: {:?}", ());
	}
}

#[cfg(test)]
mod tests {}
