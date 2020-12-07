pub fn binary_hone(instructions: &str, left: char, right: char) -> usize {
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

pub fn seat_id_from_bsp(bsp: &str) -> usize {
	let fb: &str = &bsp[0..7];
	let lr: &str = &bsp[7..10];

	let row: usize = binary_hone(fb, 'F', 'B');
	let column: usize = binary_hone(lr, 'L', 'R');

	row * 2_usize.pow(lr.len() as u32) + column
}

type Intermediate = Vec<String>;
type Solution = usize;

pub fn parse(data: &str) -> Intermediate {
	data.lines().map(|line| line.to_string()).collect()
}

pub fn part_one(specs: &Intermediate) -> Option<Solution> {
	use std::collections::BTreeSet;

	let seat_ids: BTreeSet<usize> = specs.iter().map(|bsp| seat_id_from_bsp(bsp)).collect();

	seat_ids.iter().next_back().map(|u| *u)
}

pub fn part_two(specs: &Intermediate) -> Option<Solution> {
	use std::collections::BTreeSet;

	let seat_ids: BTreeSet<usize> = specs.iter().map(|bsp| seat_id_from_bsp(bsp)).collect();

	let min: usize = *seat_ids.iter().next().unwrap();
	let max: usize = *seat_ids.iter().next_back().unwrap();

	(min..=max).find(|seat_id| !seat_ids.contains(&seat_id))
}

#[cfg(test)]
mod tests;
