use std::collections::BTreeSet;

pub type Intermediate<'input> = Vec<&'input str>;
pub type Solution = usize;

enum SystemPolicy {
	PartOne,
	PartTwo,
}

fn validate_password(password: &str, policy: SystemPolicy) -> Result<(), String> {
	let words = password.split_whitespace();

	match policy {
		SystemPolicy::PartOne => {
			let mut seen_set = BTreeSet::default();

			for word in words {
				if seen_set.contains(word) {
					return Err(format!("password contains word {word} more than once"));
				} else {
					seen_set.insert(word);
				}
			}
		}

		SystemPolicy::PartTwo => {
			let mut seen_set = BTreeSet::default();

			for word in words {
				let mut chars = word.chars().collect::<Vec<_>>();
				chars.sort();
				if seen_set.contains(&chars) {
					return Err(format!(
						"password contains anagrams of {} more than once",
						chars.iter().collect::<String>()
					));
				} else {
					seen_set.insert(chars);
				}
			}
		}
	}

	Ok(())
}

fn password_is_valid(password: &str, policy: SystemPolicy) -> bool {
	validate_password(password, policy).is_ok()
}

#[test]
fn part_one_policy_correct() {
	assert_eq!(
		Ok(()),
		validate_password("aa bb cc dd ee", SystemPolicy::PartOne)
	);
	assert_eq!(
		Err(format!("password contains word aa more than once")),
		validate_password("aa bb cc dd aa", SystemPolicy::PartOne)
	);
	assert_eq!(
		Ok(()),
		validate_password("aa bb cc dd aaa", SystemPolicy::PartOne)
	);
}

#[test]
fn part_two_policy_correct() {
	assert_eq!(
		Ok(()),
		validate_password("abcde fghij", SystemPolicy::PartTwo)
	);
	assert_eq!(
		Ok(()),
		validate_password("iiii oiii ooii oooi oooo", SystemPolicy::PartTwo)
	);

	assert_eq!(
		Err("password contains anagrams of abcde more than once".to_string()),
		validate_password("abcde xyz ecdab", SystemPolicy::PartTwo)
	);
	assert_eq!(
		Ok(()),
		validate_password("a ab abc abd abf abj", SystemPolicy::PartTwo)
	);
	assert_eq!(
		Err("password contains anagrams of iiio more than once".to_string()),
		validate_password("oiii ioii iioi iiio", SystemPolicy::PartTwo)
	);
}
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let passwords = input.lines().collect();
	Ok(passwords)
}

pub fn part_one(passwords: &Intermediate) -> Option<Solution> {
	Some(
		passwords
			.iter()
			.filter(|password| password_is_valid(&password, SystemPolicy::PartOne))
			.count(),
	)
}

pub fn part_two(passwords: &Intermediate) -> Option<Solution> {
	Some(
		passwords
			.iter()
			.filter(|password| password_is_valid(&password, SystemPolicy::PartOne))
			.filter(|password| password_is_valid(&password, SystemPolicy::PartTwo))
			.count(),
	)
}
