use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
	matches!(
		(
			passport.get("byr"),
			passport.get("iyr"),
			passport.get("eyr"),
			passport.get("hgt"),
			passport.get("hcl"),
			passport.get("ecl"),
			passport.get("pid"),
			passport.get("cid"),
		),
		(
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			Some(_),
			_
		)
	)
}

fn valid_birth_year(s: &str) -> bool {
	if let Ok(year) = s.parse::<usize>() {
		(1920..=2002).contains(&year)
	} else {
		false
	}
}

fn valid_issue_year(s: &str) -> bool {
	if let Ok(iyr) = s.parse::<usize>() {
		(2010..=2020).contains(&iyr)
	} else {
		false
	}
}

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


						let eyr_ok: bool = if let Ok(eyr) = eyr.parse::<usize>() {
							2020 <= eyr && eyr <= 2030
						} else {
							false
						};

						let hgt_ok: bool = {
							if let Some(first_letter_offset) = hgt.chars().position(|c| c.is_alphabetic()) {
								let maybe_number = &hgt[0..first_letter_offset];
								if let Ok(number) = maybe_number.parse::<usize>() {
									let maybe_rest = &hgt[first_letter_offset..];

									match maybe_rest {
										"cm" => 150 <= number && number <= 193,
										"in" => 59 <= number && number <= 76,
										_ => false,
									}
								} else {
									false
								}
							} else {
								false
							}
						};

						let hcl_ok: bool = {
							if hcl.chars().nth(0) == Some('#') {
								let is_digit_and_lowercase = hcl[1..].chars().all(|ch| {
									if ch.is_digit(16) && !ch.is_digit(10) {
										ch.is_lowercase()
									} else if ch.is_digit(10) {
										true
									} else {
										false
									}
								});

								is_digit_and_lowercase
							} else {
								false
							}
						};

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

#[cfg(test)]
mod tests {}
