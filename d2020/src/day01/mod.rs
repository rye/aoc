use std::collections::BTreeSet;

/// Find a pair of elements in `list` that sum to `target`.
pub fn find_pair(list: &BTreeSet<i64>, target: &i64) -> Option<(i64, i64)> {
	for item in list {
		if list.contains(&(target - item)) {
			return Some((*item, target - item));
		}
	}
	None
}

/// Find a triple of elements in `list` that sum to `target`.
pub fn find_triple(list: &BTreeSet<i64>, target: &i64) -> Option<(i64, i64, i64)> {
	for item in list {
		if let Some(pair) = find_pair(list, &(target - item)) {
			return Some((*item, pair.0, pair.1));
		}
	}
	None
}

type Intermediate = BTreeSet<i64>;
type Solution = i64;

pub fn parse(data: &str) -> Intermediate {
	data
		.lines()
		.map(|line| line.parse::<i64>().unwrap())
		.collect()
}

pub fn part_one(intermediate: &Intermediate) -> Option<Solution> {
	let pair = find_pair(intermediate, &2020_i64);

	if let Some((a, b)) = pair {
		Some(a * b)
	} else {
		None
	}
}

pub fn part_two(intermediate: &Intermediate) -> Option<Solution> {
	let triple = find_triple(intermediate, &2020_i64);

	if let Some((a, b, c)) = triple {
		Some(a * b * c)
	} else {
		None
	}
}

#[cfg(test)]
mod tests;
