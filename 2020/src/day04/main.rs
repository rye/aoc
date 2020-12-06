use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

use d2020::day04::*;

fn main() {
	let data: String = {
		let mut string: String = String::new();
		stdin()
			.read_to_string(&mut string)
			.expect("couldn't read stdin() to string");
		string
	};

	// To start with, passports are separated by \n\n
	let passports: Vec<&str> = data.split("\n\n").collect();

	// Next, we split each passport on all whitespace to get a collection of key:value groups per passport
	let passports: Vec<Vec<&str>> = passports
		.iter()
		.map(|line| line.split_whitespace().collect())
		.collect();

	// Now, turn those collections of key:value groups into hashmaps
	let passports: Vec<HashMap<&str, &str>> = passports
		.iter()
		.map(|pairs| {
			pairs
				.iter()
				.map(|pair| {
					let results: Vec<&str> = pair.split(':').collect();
					(results[0], results[1])
				})
				.collect()
		})
		.collect();

	{
		let valid_passports: Vec<&HashMap<&str, &str>> = passports
			.iter()
			.filter(|passport| has_required_fields(passport))
			.collect();

		println!("Part One: {:?}", valid_passports.len());
	}

	{
		let valid_eye_colors: HashSet<&str> = {
			let mut set = HashSet::new();
			set.insert("amb");
			set.insert("blu");
			set.insert("brn");
			set.insert("gry");
			set.insert("grn");
			set.insert("hzl");
			set.insert("oth");
			set
		};

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
					(Some(byr), Some(iyr), Some(eyr), Some(hgt), Some(hcl), Some(ecl), Some(pid), _) => {
						let byr_ok: bool = valid_birth_year(byr);
						let iyr_ok: bool = valid_issue_year(iyr);
						let eyr_ok: bool = valid_expiry_year(eyr);
						let hgt_ok: bool = valid_height(hgt);
						let hcl_ok: bool = valid_hair_color(hcl);
						let ecl_ok: bool = valid_eye_colors.contains(ecl);
						let pid_ok: bool = { pid.len() == 9 && pid.chars().all(|c| c.is_digit(10)) };

						byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
					}
					_ => false,
				}
			})
			.collect();

		println!("Part Two: {:?}", valid_passports.len());
	}
}
