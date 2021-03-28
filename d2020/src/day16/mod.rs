use std::collections::{HashMap, HashSet};

type Intermediate = ();
type Solution = u64;

pub fn parse(input: &str) -> Intermediate {
	todo!()
}

pub fn part_one(input: &Intermediate) -> Option<Solution> {
	todo!()
}

#[cfg(test)]
mod part_one {
	use super::{parse, part_one};

	#[test]
	fn example() {
		let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

		let intermediate = parse(input);

		let part_one = part_one(&intermediate);

		assert_eq!(part_one, Some(71));
	}
}

pub fn part_two(input: &Intermediate) -> Option<Solution> {
	todo!()
}
