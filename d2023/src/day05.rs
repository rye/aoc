pub type Intermediate = (Vec<u64>, Vec<FunkyRangeMap>);
pub type Output = u64;

pub struct FunkyRangeMap {
	src_cat: String,
	dst_cat: String,
	ranges: Vec<(core::ops::Range<u64>, core::ops::Range<u64>)>,
}

impl core::str::FromStr for FunkyRangeMap {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut lines = s.lines();

		let main_line = lines.next().unwrap();

		let src_cat = main_line
			.split(' ')
			.next()
			.unwrap()
			.split('-')
			.next()
			.unwrap()
			.to_owned();
		let dst_cat = main_line
			.split(' ')
			.next()
			.unwrap()
			.split('-')
			.last()
			.unwrap()
			.to_owned();

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

		Ok(Self {
			src_cat,
			dst_cat,
			ranges,
		})
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
			assert_eq!(map.src_cat, "seed");
			assert_eq!(map.dst_cat, "soil");

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

		assert_eq!(intermediate.1[0].src_cat, "seed");
		assert_eq!(intermediate.1[0].dst_cat, "soil");

		assert_eq!(intermediate.1[1].src_cat, "soil");
		assert_eq!(intermediate.1[1].dst_cat, "fertilizer");

		assert_eq!(intermediate.1[2].src_cat, "fertilizer");
		assert_eq!(intermediate.1[2].dst_cat, "water");

		assert_eq!(intermediate.1[3].src_cat, "water");
		assert_eq!(intermediate.1[3].dst_cat, "light");

		assert_eq!(intermediate.1[4].src_cat, "light");
		assert_eq!(intermediate.1[4].dst_cat, "temperature");

		assert_eq!(intermediate.1[5].src_cat, "temperature");
		assert_eq!(intermediate.1[5].dst_cat, "humidity");

		assert_eq!(intermediate.1[6].src_cat, "humidity");
		assert_eq!(intermediate.1[6].dst_cat, "location");
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
pub fn part_two(_intermediate: &Intermediate) -> Option<Output> {
	None
}
