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

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
