use core::ops::Range;

use itertools::Itertools;

pub type Intermediate = (Vec<u64>, Vec<FunkyRangeMap>);
pub type Output = u64;

pub struct FunkyRangeMap {
	ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl core::str::FromStr for FunkyRangeMap {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();

		let _ = lines.next().unwrap();

		let ranges = lines
			.map(|line| {
				let mut parts = line.split_ascii_whitespace();

				let dst_range_start = parts.next().unwrap().parse().unwrap();
				let src_range_start = parts.next().unwrap().parse().unwrap();
				let range_length: u64 = parts.next().unwrap().parse().unwrap();

				(
					src_range_start..src_range_start + range_length,
					dst_range_start..dst_range_start + range_length,
				)
			})
			.collect();

		Ok(Self { ranges })
	}
}

impl FunkyRangeMap {
	fn apply(&self, src: u64) -> u64 {
		for (src_range, dst_range) in &self.ranges {
			if src_range.contains(&src) {
				return dst_range.start + (src - src_range.start);
			}
		}

		// Default is to return the original value if nothing else.
		src
	}
}

#[cfg(test)]
mod funky_range_map {
	use super::FunkyRangeMap;

	mod parse {
		use super::FunkyRangeMap;

		#[test]
		fn example() {
			let example = "seed-to-soil map:\n50 98 2\n52 50 48";

			let map = example.parse::<FunkyRangeMap>().unwrap();

			assert_eq!(map.apply(98), 50);
			assert_eq!(map.apply(99), 51);
			assert_eq!(map.apply(53), 55);
			assert_eq!(map.apply(10), 10);
		}
	}
}

/// # Errors
pub fn parse(data: &str) -> anyhow::Result<Intermediate> {
	let mut sections = data.split("\n\n");

	let first_line = sections.next().unwrap();

	let seeds: Vec<u64> = first_line
		.split(':')
		.last()
		.unwrap()
		.trim()
		.split_ascii_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();

	let sections: Vec<FunkyRangeMap> = sections
		.map(|section| section.parse())
		.collect::<Result<_, _>>()?;

	Ok((seeds, sections))
}

#[cfg(test)]
mod parse {
	use super::*;

	#[test]
	fn example() {
		let example = include_str!("examples/day05");

		let intermediate = parse(example).unwrap();
		assert_eq!(intermediate.0, vec![79, 14, 55, 13]);

		assert_eq!(intermediate.1.len(), 7);
	}
}

#[must_use]
pub fn part_one((seeds, maps): &Intermediate) -> Option<Output> {
	let locations: Vec<u64> = seeds
		.iter()
		.map(|seed| maps.iter().fold(*seed, |seed, map| map.apply(seed)))
		.collect();

	Some(*locations.iter().min().unwrap())
}

daocutil::test_example!(
	part_one_example,
	parse,
	part_one,
	include_str!("examples/day05"),
	Some(35)
);

#[must_use]
pub fn part_two((seeds, maps): &Intermediate) -> Option<Output> {
	let seed_ranges: Vec<Range<u64>> = seeds
		.into_iter()
		.tuples()
		.map(|(start, len)| *start..start + len)
		.collect();

	let min = seed_ranges
		.into_iter()
		.map(|range| {
			(
				range.clone(),
				range
					.map(|seed| maps.iter().fold(seed, |seed, map| map.apply(seed)))
					.min()
					.unwrap(),
			)
		})
		.map(|n| n.1)
		.min();

	min
}

daocutil::test_example!(
	part_two_example,
	parse,
	part_two,
	include_str!("examples/day05"),
	Some(46)
);
