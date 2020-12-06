use std::collections::HashMap;

pub fn has_required_fields(passport: &HashMap<&str, &str>) -> bool {
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

pub fn valid_birth_year(s: &str) -> bool {
	if let Ok(year) = s.parse::<usize>() {
		(1920..=2002).contains(&year)
	} else {
		false
	}
}

pub fn valid_issue_year(s: &str) -> bool {
	if let Ok(iyr) = s.parse::<usize>() {
		(2010..=2020).contains(&iyr)
	} else {
		false
	}
}

pub fn valid_expiry_year(s: &str) -> bool {
	if let Ok(eyr) = s.parse::<usize>() {
		(2020..=2030).contains(&eyr)
	} else {
		false
	}
}

pub fn valid_height(s: &str) -> bool {
	if let Some(first_letter_offset) = s.chars().position(|c| c.is_alphabetic()) {
		let maybe_number = &s[0..first_letter_offset];
		if let Ok(number) = maybe_number.parse::<usize>() {
			let rest = &s[first_letter_offset..];

			match rest {
				"cm" => (150..=193).contains(&number),
				"in" => (59..=76).contains(&number),
				_ => false,
			}
		} else {
			false
		}
	} else {
		false
	}
}

pub fn valid_hair_color(s: &str) -> bool {
	if let Some(rest) = s.strip_prefix('#') {
		let is_digit_and_lowercase = rest.chars().all(|ch| {
			if ch.is_digit(16) && !ch.is_digit(10) {
				ch.is_lowercase()
			} else {
				ch.is_digit(10)
			}
		});

		is_digit_and_lowercase
	} else {
		false
	}
}

#[cfg(test)]
mod tests;
