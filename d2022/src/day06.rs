use {core::hash::Hash, std::collections::HashSet};

pub type Intermediate = Vec<char>;
pub type Output = usize;

/// # Errors
pub fn parse(input: &str) -> anyhow::Result<Intermediate> {
	Ok(input.lines().flat_map(str::chars).collect())
}

fn find_end_of_first_non_identical_seq<T: Copy + Hash + Eq>(
	length: usize,
	slice: &[T],
) -> Option<usize> {
	let mut end_of_marker = None;

	for (idx, window) in slice.windows(length).enumerate() {
		let set: HashSet<T> = window.iter().copied().collect();

		if set.len() == length {
			end_of_marker = Some(idx + length);
			break;
		}
	}

	end_of_marker
}

#[must_use]
pub fn part_one(chars: &Intermediate) -> Option<Output> {
	find_end_of_first_non_identical_seq(4, chars)
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
	find_end_of_first_non_identical_seq(14, chars)
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
