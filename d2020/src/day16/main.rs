#![allow(unused_imports)]

use std::{
	collections::*,
	io::{stdin, Read},
};

use core::{ops::RangeInclusive, str::FromStr, str::Lines};

use d2020::{day16::*, *};

fn main() {
	let data: String = string_from(stdin()).unwrap();

	{
		println!("Part One: {:?}", ());
	}

	{
		println!("Part Two: {:?}", ());
	}
}

/// Parse a range which has potentially multiple "parts" into a Vec of RangeInclusive's
/// comprised of the parts in the expected way.
///
/// # Example
///
/// ```
/// # use d2020::day16::parse_multirange;
/// assert_eq!(parse_multirange("1-3"), vec![1..=3]);
/// ```
fn parse_multirange(data: &str) -> Vec<RangeInclusive<u32>> {
	data
		.split(" or ")
		.map(|range_text| {
			let mut range_parts = range_text.split('-');
			let start = range_parts.next().unwrap().parse::<u32>().unwrap();
			let end = range_parts.next().unwrap().parse::<u32>().unwrap();
			start..=end
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::parse_multirange;

	#[cfg(test)]
	mod parse_multirange {
		use super::parse_multirange;

		#[test]
		fn one_range() {
			assert_eq!(parse_multirange("1-3"), vec![1..=3]);
		}

		#[test]
		fn two_ranges() {
			assert_eq!(parse_multirange("1-3 or 4-6"), vec![1..=3, 4..=6]);
		}

		#[test]
		fn multiple_ranges() {
			assert_eq!(
				parse_multirange("1-3 or 4-6 or 77128-192571"),
				vec![1..=3, 4..=6, 77128..=192571]
			);
		}
	}
}
