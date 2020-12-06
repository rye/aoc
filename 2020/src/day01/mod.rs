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

#[cfg(test)]
mod tests;
