#![allow(unused_imports)]

use std::{
	collections::*,
	io::{stdin, Read},
};

use core::{ops::RangeInclusive, str::FromStr, str::Lines};

use d2020::{day16::*, *};

type MysteriousNumber = u16;

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

/// Break the "rules" header, specifying which numbers are valid, and where in the string they can appear.
fn valid_values_and_indices_from_rules(rules: &str) -> HashMap<MysteriousNumber, HashSet<usize>> {
	let mut values_and_indices: HashMap<MysteriousNumber, HashSet<usize>> = HashMap::new();

	for (class_idx, line) in rules.lines().enumerate() {
		if let (Some(_category), Some(multirange)) = {
			let mut split = line.split(": ");
			(split.next(), split.next())
		} {
			let ranges = parse_multirange(multirange);
			for range in ranges {
				for num in range {
					values_and_indices
						.entry(num)
						.and_modify(|v| {
							v.insert(class_idx);
						})
						.or_insert([class_idx].iter().copied().collect());
				}
			}
		}
	}

	values_and_indices
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
fn parse_multirange(data: &str) -> Vec<RangeInclusive<MysteriousNumber>> {
	data
		.split(" or ")
		.map(|range_text| {
			let mut range_parts = range_text.split('-');
			let start = range_parts
				.next()
				.unwrap()
				.parse::<MysteriousNumber>()
				.unwrap();
			let end = range_parts
				.next()
				.unwrap()
				.parse::<MysteriousNumber>()
				.unwrap();
			start..=end
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::{file_parts, parse_multirange, valid_values_and_indices_from_rules};

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
				parse_multirange("1-3 or 4-6 or 1024-8192"),
				vec![1..=3, 4..=6, 1024..=8192]
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

	#[cfg(test)]
	mod valid_values_and_indices_from_rules {
		use super::valid_values_and_indices_from_rules;

		#[test]
		fn single_rule_single_range_is_correct() {
			let rules = "_: 1-3";

			let result = valid_values_and_indices_from_rules(rules);
			assert_eq!(
				{
					let mut keys = result.keys().collect::<Vec<_>>();
					keys.sort();
					keys
				},
				vec![&1_u16, &2_u16, &3_u16]
			);

			assert_eq!(result.get(&0_u16), None);
			assert_eq!(result.get(&1_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&2_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&3_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&4_u16), None);
		}

		#[test]
		fn single_rule_multiple_range_is_correct() {
			let rules = "_: 1-3 or 6-9";

			let result = valid_values_and_indices_from_rules(rules);
			assert_eq!(
				{
					let mut keys = result.keys().collect::<Vec<_>>();
					keys.sort();
					keys
				},
				vec![&1_u16, &2_u16, &3_u16, &6_u16, &7_u16, &8_u16, &9_u16]
			);

			assert_eq!(result.get(&0_u16), None);
			assert_eq!(result.get(&1_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&2_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&3_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&4_u16), None);
		}

		#[test]
		fn multiple_rules_each_single_overlapping_range_is_correct() {
			let rules = "_: 1-3\n__: 2-4";

			let result = valid_values_and_indices_from_rules(rules);
			assert_eq!(
				{
					let mut keys = result.keys().collect::<Vec<_>>();
					keys.sort();
					keys
				},
				vec![&1_u16, &2_u16, &3_u16, &4_u16]
			);

			assert_eq!(result.get(&0_u16), None);
			assert_eq!(result.get(&1_u16), Some(&vec![0].into_iter().collect()));
			assert_eq!(result.get(&2_u16), Some(&vec![0, 1].into_iter().collect()));
			assert_eq!(result.get(&3_u16), Some(&vec![0, 1].into_iter().collect()));
			assert_eq!(result.get(&4_u16), Some(&vec![1].into_iter().collect()));
			assert_eq!(result.get(&5_u16), None);
		}
	}
}
