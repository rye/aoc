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
	// Loop over all the windows, enumerating the _start-of-window_ indices as we go.
	slice
		.windows(length)
		.enumerate()
		// Count the number of unique elements in T and reduce the iterator to (idx, unique element count).
		.map(|(idx, window)| (idx, window.iter().copied().collect::<HashSet<T>>().len()))
		// Look for the idx of the first window that has a unique number of elements matching
		// our desired window length, then return the idx following the end of that window.
		.find_map(|(idx, unique_count)| match unique_count {
			sz if sz == length => Some(idx + sz),
			_ => None,
		})
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
