#![allow(unused_imports)]

use std::{
	collections::*,
	io::{stdin, Read},
};

use core::{ops::RangeInclusive, str::FromStr, str::Lines};

use d2020::{day16::*, *};

fn main() {
	let data: String = string_from(stdin()).unwrap();

	let file_parts = file_parts(data);

	{
		println!("Part One: {:?}", ());
	}

	{
		println!("Part Two: {:?}", ());
	}
}

/// Break the input into chunks based on the "your ticket:" and "nearby tickets:" separators.
fn file_parts(data: String) -> Vec<String> {
	let split_a: Vec<&str> = data.split("\n\nyour ticket:\n").collect();
	let split_b: Vec<&str> = split_a[1].split(&"\n\nnearby tickets:\n").collect();

	let rules = split_a[0].to_string();
	let my_ticket = split_b[0].to_string();
	let nearby_tickets = split_b[1].to_string();

	vec![rules, my_ticket, nearby_tickets]
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
	use super::{file_parts, parse_multirange};

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

	#[cfg(test)]
	mod file_parts {
		use super::file_parts;

		#[test]
		fn proper_line_splits() {
			let simple_example: String =
				"a: 1-3\n\nyour ticket:\n1\n\nnearby tickets:\n1\n2\n3".to_string();

			assert_eq!(file_parts(simple_example), vec!["a: 1-3", "1", "1\n2\n3"]);
		}
	}
}
