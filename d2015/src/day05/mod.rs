type Intermediate<'input> = Vec<&'input str>;

pub fn parse<'input>(input: &'input str) -> Intermediate {
	input.lines().collect()
}

type Solution = usize;

fn contains_at_least_three_vowels(string: &str) -> bool {
	string
		.chars()
		.filter(|c| match c.to_ascii_lowercase() {
			'a' | 'e' | 'i' | 'o' | 'u' => true,
			_ => false,
		})
		.count()
		>= 3
}

fn has_at_least_one_duplicated_letter(string: &str) -> bool {
	let chars: Vec<char> = string.chars().collect();
	for window in chars.windows(2) {
		if window.get(0) == window.get(1) {
			return true;
		}
	}
	false
}

fn does_not_contain_banned_strings(string: &str) -> bool {
	const STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

	for banned_str in STRINGS {
		if string.contains(banned_str) {
			return false;
		}
	}

	true
}

fn is_nice_old(string: &impl AsRef<str>) -> bool {
	let string: &str = string.as_ref();

	contains_at_least_three_vowels(string)
		&& has_at_least_one_duplicated_letter(string)
		&& does_not_contain_banned_strings(string)
}

pub fn part_one(strings: &Intermediate) -> Option<Solution> {
	Some(strings.iter().filter(is_nice_old).count())
}

fn contains_nonoverlapping_pair(string: &str) -> bool {
	let chars: Vec<char> = string.chars().collect();
	for window in chars.windows(2) {
		let pat = format!("{}{}", window[0], window[1]);
		if string.split(&pat).count() >= 3 {
			return true;
		}
	}

	false
}

#[test]
fn aaa_cnp() {
	assert_eq!(contains_nonoverlapping_pair("aaa"), false);
}

#[test]
fn aaaa_cnp() {
	assert_eq!(contains_nonoverlapping_pair("aaaa"), true);
}

#[test]
fn xy0xy_cnp() {
	assert_eq!(contains_nonoverlapping_pair("xy0xy"), true);
}

fn contains_one_wrapped_letter(string: &str) -> bool {
	let chars: Vec<char> = string.chars().collect();

	for window in chars.windows(3) {
		if window.get(0) == window.get(2) {
			return true;
		}
	}

	false
}

fn is_nice_new(string: &impl AsRef<str>) -> bool {
	let string: &str = string.as_ref();

	contains_nonoverlapping_pair(string) && contains_one_wrapped_letter(string)
}

pub fn part_two(strings: &Intermediate) -> Option<Solution> {
	Some(strings.iter().filter(is_nice_new).count())
}

#[cfg(test)]
mod is_nice_new {
	use super::is_nice_new;

	#[test]
	fn qjhvhtzxzqqjkmpb_is_nice() {
		assert_eq!(is_nice_new(&"qjhvhtzxzqqjkmpb"), true);
	}

	#[test]
	fn xxyxx_is_nice() {
		assert_eq!(is_nice_new(&"xxyxx"), true);
	}

	#[test]
	fn uurcxstgmygtbstg_is_naughty() {
		assert_eq!(is_nice_new(&"uurcxstgmygtbstg"), false);
	}

	#[test]
	fn ieodomkazucvgmuy_is_nice() {
		assert_eq!(is_nice_new(&"ieodomkazucvgmuy"), false);
	}
}
