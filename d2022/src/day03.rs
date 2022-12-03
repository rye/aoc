use std::collections::HashSet;

pub type Intermediate<'a> = Vec<&'a str>;
pub type Output = u32;

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(str.lines().collect())
}

fn item_priority(item: char) -> Option<u32> {
	u8::try_from(item).ok().map(|byte| match byte {
		b'a'..=b'z' => ((byte - b'a') + 1) as u32,
		b'A'..=b'Z' => ((byte - b'A') + 26 + 1) as u32,
		_ => unreachable!(),
	})
}

#[cfg(test)]
mod item_priority {
	use super::item_priority;

	#[test]
	fn item_priority_a_lc() {
		assert_eq!(item_priority('a'), Some(1));
	}

	#[test]
	fn item_priority_z_lc() {
		assert_eq!(item_priority('z'), Some(26));
	}

	#[test]
	fn item_priority_a_uc() {
		assert_eq!(item_priority('A'), Some(27));
	}

	#[test]
	fn item_priority_z_uc() {
		assert_eq!(item_priority('Z'), Some(52));
	}
}

#[must_use]
pub fn part_one(rucksacks: &Intermediate) -> Option<Output> {
	let mut sum = 0_u32;

	for (compartment_a, compartment_b) in rucksacks
		.iter()
		.map(|rucksack| rucksack.split_at(rucksack.len() / 2))
	{
		let priorities_a: HashSet<u32> = compartment_a.chars().filter_map(item_priority).collect();
		let priorities_b: HashSet<u32> = compartment_b.chars().filter_map(item_priority).collect();

		sum += priorities_a.intersection(&priorities_b).sum::<u32>();
	}

	Some(sum)
}

#[must_use]
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
