#![allow(unused_imports)]

use std::io::{stdin, Read};
use std::{collections::*, str::FromStr};

use regex::Regex;

use d2020::crt;
use d2020::day13::*;

fn main() {
	let mut stdin = stdin();
	let mut data = String::new();
	stdin.read_to_string(&mut data).unwrap();

	let timestamp = data.lines().nth(0).unwrap().parse::<i64>().unwrap();

	let bus_intervals: Vec<Option<i64>> = data
		.lines()
		.nth(1)
		.unwrap()
		.split(',')
		.map(|n| match n {
			"x" => None,
			k => k.parse::<i64>().ok(),
		})
		.collect();

	{
		// Turn each bus ID (period) into ([time until next stop], [period])
		let mut values: Vec<(i64, i64)> = bus_intervals
			.iter()
			.filter_map(|o| *o)
			.map(|n| ((timestamp / n) * n + n - timestamp, n))
			.collect();

		// Sort. For tuples, the first subscript is sorted on first, so we'll
		// get the earliest next stop at the start of the list.
		values.sort();

		// Answer is the time until next stop * ID of the bus.
		let result = values[0].0 * values[0].1;

		println!("Part One: {:?}", result);
	}

	{
		let divisors_and_remainders: Vec<(i64, i64)> = bus_intervals
			.iter()
			.enumerate()
			.filter(|(_, x)| x.is_some())
			.map(|(i, x)| (i, x.unwrap()))
			.map(|(idx, bus_id)| (bus_id, bus_id - idx as i64))
			.collect();

		println!("Part Two: {}", crt(&divisors_and_remainders));
	}
}
