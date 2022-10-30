use std::collections::*;

fn sus(numbers: &Vec<isize>) -> Option<isize> {
	let mut seen: HashSet<isize> = HashSet::new();

	for number in &numbers[0..25] {
		seen.insert(*number);
	}

	for number in &numbers[25..] {
		let mut not_found = true;

		for prior in &seen {
			if seen.contains(&(number - prior)) {
				not_found = false;
				seen.insert(*number);
				break;
			}
		}

		if not_found {
			return Some(*number);
		}
	}

	None
}

fn find_weakness(numbers: &Vec<isize>, impostor: isize) -> Option<Vec<isize>> {
	for start_idx in 0..numbers.len() - 2 {
		for end_idx in (start_idx + 1)..(numbers.len() - 1) {
			let mut sum = 0;
			for x in &numbers[start_idx..end_idx + 1] {
				sum += x;
				if sum > impostor {
					break;
				}
			}

			if sum > impostor {
				break;
			} else if sum == impostor {
				let region = numbers[start_idx..end_idx + 1]
					.into_iter()
					.cloned()
					.collect();
				return Some(region);
			}
		}
	}

	None
}

pub type Intermediate = Vec<isize>;
pub type Solution = isize;

pub fn parse(data: &str) -> Result<Intermediate, core::convert::Infallible> {
	Ok(
		data
			.lines()
			.map(|s| s.parse().expect("failed to parse line as isize"))
			.collect(),
	)
}

pub fn part_one(numbers: &Intermediate) -> Option<Solution> {
	sus(&numbers)
}

pub fn part_two(numbers: &Intermediate) -> Option<Solution> {
	let impostor = sus(&numbers).expect("couldn't find impostor");

	let mut result = None;

	if let Some(set) = find_weakness(&numbers, impostor) {
		if let (Some(max), Some(min)) = (set.iter().max(), set.iter().min()) {
			result = Some(max + min);
		}
	}

	result
}
