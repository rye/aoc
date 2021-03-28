use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
	pub field: String,
	pub ranges: [RangeInclusive<u64>; 2],
}

impl Rule {
	pub fn matches(&self, x: u64) -> bool {
		self.ranges.iter().any(|range| range.contains(&x))
	}
}

impl From<&str> for Rule {
	fn from(s: &str) -> Self {
		let RE: Regex = Regex::new(r"^(.*+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

		let caps = RE.captures(s).unwrap();

		Rule {
			field: caps.get(1).unwrap().as_str().into(),
			ranges: [
				caps.get(2).unwrap().as_str().parse().unwrap()
					..=caps.get(3).unwrap().as_str().parse().unwrap(),
				caps.get(4).unwrap().as_str().parse().unwrap()
					..=caps.get(5).unwrap().as_str().parse().unwrap(),
			],
		}
	}
}

#[derive(Debug, Clone)]
pub struct Input {
	rules: Vec<Rule>,
	mine: Vec<u64>,
	others: Vec<Vec<u64>>,
}

type Intermediate = Input;
type Solution = u64;

pub fn parse(input: &str) -> Intermediate {
	let mut sections = input.split("\n\n");

	let rules: Vec<Rule> = sections.next().unwrap().lines().map(Rule::from).collect();

	let mine = sections
		.next()
		.unwrap()
		.lines()
		.nth(1)
		.unwrap()
		.split(',')
		.map(|n| n.parse().unwrap())
		.collect();

	let others = sections
		.next()
		.unwrap()
		.lines()
		.skip(1)
		.map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
		.collect();

	Input {
		rules,
		mine,
		others,
	}
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
