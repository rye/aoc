
use std::collections::BTreeSet;
use std::io::{stdin, BufRead};

fn binary_hone(instructions: &str, left: char, right: char) -> usize {
	let mut range: core::ops::Range<usize> = 0..2_usize.pow(instructions.len() as u32);

	for instruction in instructions.chars() {
		let midpoint = (range.start + range.end) / 2;

		match instruction {
			c if c == left => range = range.start..midpoint,
			c if c == right => range = midpoint..range.end,
			_ => panic!("strange instruction"),
		}
	}

	range.start
}

fn seat_id_from_bsp(bsp: &str) -> usize {
	let fb: &str = &bsp[0..7];
	let lr: &str = &bsp[7..10];

	let row: usize = binary_hone(fb, 'F', 'B');
	let column: usize = binary_hone(lr, 'L', 'R');

	row * 2_usize.pow(lr.len() as u32) + column
}

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

#[cfg(test)]
mod tests {}
