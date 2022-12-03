use std::collections::HashSet;

pub type Intermediate<'a> = Vec<&'a str>;
pub type Output = u32;

/// # Errors
pub fn parse(str: &str) -> anyhow::Result<Intermediate> {
	Ok(str.lines().collect())
}

fn item_priority(item: char) -> Option<u32> {
	u8::try_from(item).ok().map(|byte| match byte {
		b'a'..=b'z' => u32::from((byte - b'a') + 1),
		b'A'..=b'Z' => u32::from((byte - b'A') + 26 + 1),
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
pub fn part_two(compartment_pairs: &Intermediate) -> Option<Output> {
	let mut sum = 0_u32;

	for group_rucksacks in compartment_pairs.chunks_exact(3) {
		let mut sacks = group_rucksacks.iter();

		let mut all_priorities: HashSet<u32> = sacks
			.next()
			.expect("need first sack in 3-group")
			.chars()
			.filter_map(item_priority)
			.collect();

		all_priorities = &all_priorities
			& &sacks
				.next()
				.expect("need next sack in 3-group")
				.chars()
				.filter_map(item_priority)
				.collect::<HashSet<u32>>();

		all_priorities = &all_priorities
			& &sacks
				.next()
				.expect("need last sack in 3-group")
				.chars()
				.filter_map(item_priority)
				.collect::<HashSet<u32>>();

		assert_eq!(all_priorities.len(), 1);

		sum += all_priorities.iter().sum::<u32>();
	}

	Some(sum)
}
