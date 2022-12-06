use itertools::Itertools;

use std::collections::HashSet;

pub type Intermediate = Vec<char>;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.lines().flat_map(str::chars).collect())
}

#[must_use]
pub fn part_one(chars: &Intermediate) -> Option<Output> {
	let mut start_of_packet: Option<usize> = None;

	for ((_idx_0, char_0), (_idx_1, char_1), (_idx_2, char_2), (idx_3, char_3)) in
		chars.iter().enumerate().tuple_windows()
	{
		let set: HashSet<char> = vec![char_0, char_1, char_2, char_3]
			.into_iter()
			.copied()
			.collect();

		if set.len() == 4 {
			start_of_packet = Some(idx_3 + 1);
			break;
		}
	}

	start_of_packet
}

#[test]
fn part_one_example() {
	let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(7)
	);
}

#[test]
fn part_one_other_examples() {
	let example = "bvwbjplbgvbhsrlpgdmjqwftvncz";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(5)
	);

	let example = "nppdvjthqldpwncqszvftbrmjlhg";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(6)
	);

	let example = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(10)
	);

	let example = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
	assert_eq!(
		part_one(&parse(example).expect("example didn't parse?!")),
		Some(11)
	);
}

#[must_use]
pub fn part_two(chars: &Intermediate) -> Option<Output> {
	let mut start_of_message: Option<usize> = None;

	for (idx, window) in chars.windows(14).enumerate() {
		let set: HashSet<char> = window.iter().copied().collect();

		if set.len() == 14 {
			start_of_message = Some(idx + 14);
			break;
		}
	}

	start_of_message
}

#[test]
fn part_two_examples() {
	let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
	assert_eq!(
		part_two(&parse(example).expect("example didn't parse")),
		Some(19)
	);

	let example = "bvwbjplbgvbhsrlpgdmjqwftvncz";
	assert_eq!(
		part_two(&parse(example).expect("example didn't parse")),
		Some(23)
	);

	let example = "nppdvjthqldpwncqszvftbrmjlhg";
	assert_eq!(
		part_two(&parse(example).expect("example didn't parse")),
		Some(23)
	);

	let example = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
	assert_eq!(
		part_two(&parse(example).expect("example didn't parse")),
		Some(29)
	);

	let example = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
	assert_eq!(
		part_two(&parse(example).expect("example didn't parse")),
		Some(26)
	);
}
