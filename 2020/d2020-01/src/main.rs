use std::collections::HashSet;
use std::io::{stdin, BufRead};

/// Find a pair of elements in `list` that sum to `target`.
fn find_pair(list: &HashSet<i64>, target: &i64) -> Option<(i64, i64)> {
	for item in list {
		if list.contains(&(target - item)) {
			return Some((*item, target - item));
		}
	}
	None
}

/// Find a triple of elements in `list` that sum to `target`.
fn find_triple(list: &HashSet<i64>, target: &i64) -> Option<(i64, i64, i64)> {
	for item in list {
		if let Some(pair) = find_pair(list, &(target - item)) {
			return Some((*item, pair.0, pair.1));
		}
	}
	None
}

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	use util::Lines;

	let list: HashSet<i64> = (Lines::from(stdin.lines())).into();

	let pair = find_pair(&list, &2020_i64);

	if let Some((a, b)) = pair {
		println!("Part One: {}", a * b);
	}

	let triple = find_triple(&list, &2020_i64);

	if let Some((a, b, c)) = triple {
		println!("Part Two: {}", a * b * c);
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	fn sample() -> HashSet<i64> {
		vec![1721, 979, 366, 299, 675, 1456].drain(..).collect()
	}

	#[cfg(test)]
	mod find_pair {
		use super::super::find_pair;
		use super::*;

		#[test]
		fn sample_correct() {
			let list = sample();
			let target = 2020_i64;
			assert_eq!(find_pair(&list, &target), Some((299, 1721)))
		}

		#[test]
		fn empty_list_correct() {
			let list = HashSet::new();
			let target = 2020_i64;
			assert_eq!(find_pair(&list, &target), None)
		}
	}

	#[cfg(test)]
	mod find_triple {
		use super::super::find_triple;
		use super::*;

		#[test]
		fn sample_correct() {
			let list = sample();
			let target = 2020_i64;
			assert_eq!(find_triple(&list, &target), Some((366, 675, 979)))
		}

		#[test]
		fn empty_list_correct() {
			let list = HashSet::new();
			let target = 2020_i64;
			assert_eq!(find_triple(&list, &target), None)
		}
	}
}
