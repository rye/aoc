use std::collections::BTreeSet;

pub type Intermediate<'input> = Vec<&'input str>;
pub type Solution = usize;

fn validate_password(password: &str) -> Result<(), String> {
	let words = password.split_whitespace();

	let mut seen_set = BTreeSet::default();

	for word in words {
		if seen_set.contains(word) {
			return Err(format!("password contains word {word} more than once"));
		} else {
			seen_set.insert(word);
		}
	}

	Ok(())
}

fn password_is_valid(password: &str) -> bool {
	validate_password(password).is_ok()
}

#[test]
fn validate_password_correct() {
	assert_eq!(Ok(()), validate_password("aa bb cc dd ee"));
	assert_eq!(
		Err(format!("password contains word aa more than once")),
		validate_password("aa bb cc dd aa")
	);
	assert_eq!(Ok(()), validate_password("aa bb cc dd aaa"));
}

pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	let passwords = input.lines().collect();
	Ok(passwords)
}

pub fn part_one(passwords: &Intermediate) -> Option<Solution> {
	Some(
		passwords
			.iter()
			.filter(|password| password_is_valid(&password))
			.count(),
	)
}

pub fn part_two(_intermediate: &Intermediate) -> Option<Solution> {
	None
}
