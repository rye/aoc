use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

use d2020::day05::*;

fn main() {
	let stdin = stdin();
	let stdin = stdin.lock();

	let data: Vec<String> = stdin
		.lines()
		.filter_map(Result::ok)
		.map(|line| line)
		.collect();

	{
		let seat_ids: BTreeSet<usize> = data.iter().map(|bsp| seat_id_from_bsp(bsp)).collect();

		let largest_seat_id: usize = *seat_ids.iter().next_back().unwrap();

		println!("Part One: {:?}", largest_seat_id);
	}

	{
		let seat_ids: BTreeSet<usize> = data.iter().map(|bsp| seat_id_from_bsp(bsp)).collect();

		let min: usize = *seat_ids.iter().next().unwrap();
		let max: usize = *seat_ids.iter().next_back().unwrap();

		let my_seat = (min..=max)
			.find(|seat_id| !seat_ids.contains(&seat_id))
			.unwrap();

		println!("Part Two: {:?}", my_seat);
	}
}
